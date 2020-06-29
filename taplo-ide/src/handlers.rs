use crate::request_ext::*;
use crate::{
    analytics::{collect_keys, Key, PositionInfo},
    read_file,
    schema::{get_schema_objects, BUILTIN_SCHEME},
    Document, HashRegex, World,
};
use indexmap::IndexMap;
use lsp_async_stub::{rpc::Error, Context, Params, RequestWriter};
use lsp_types::*;
use regex::Regex;
use schemars::schema::RootSchema;
use std::{collections::HashMap, convert::TryFrom, mem};
use taplo::{dom::Common, formatter, util::coords::Mapper};
use verify::Verify;
use wasm_bindgen_futures::spawn_local;
use crate::request_ext;

mod completion;
mod diagnostics;
mod document_symbols;
mod folding_ranges;
mod semantic_tokens;

pub(crate) async fn initialize(
    mut context: Context<World>,
    params: Params<InitializeParams>,
) -> Result<InitializeResult, Error> {
    let p = params.required()?;

    context.world().lock().await.workspace_uri = p.root_uri.map(|mut uri| {
        uri.set_path(&(uri.path().to_string() + "/"));
        uri
    });

    // Update configuration after initialization.
    // !! This might cause race conditions with this response,
    // !! it is fine in the single-threaded wasm environment.
    spawn_local(update_configuration(context));

    Ok(InitializeResult {
        capabilities: ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::Full)),
            semantic_tokens_provider: Some(
                SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: false.into(),
                    },
                    legend: SemanticTokensLegend {
                        token_types: semantic_tokens::TokenType::LEGEND.into(),
                        token_modifiers: semantic_tokens::TokenModifier::MODIFIERS.into(),
                    },
                    range_provider: None,
                    document_provider: Some(SemanticTokensDocumentProvider::Bool(true)),
                }),
            ),
            folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
            document_symbol_provider: Some(true),
            document_formatting_provider: Some(true),
            hover_provider: Some(true),
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(false),
                trigger_characters: Some(vec![
                    ".".into(),
                    "=".into(),
                    "[".into(),
                    "{".into(),
                    ",".into(),
                    "\"".into(),
                ]),
                ..Default::default()
            }),
            document_link_provider: Some(DocumentLinkOptions {
                resolve_provider: None,
                work_done_progress_options: Default::default(),
            }),
            ..Default::default()
        },
        server_info: Some(ServerInfo {
            name: "ebToml".into(),
            version: Some("1.0.0".into()),
        }),
    })
}

async fn update_configuration(mut context: Context<World>) {
    let res = context
        .write_request::<request::WorkspaceConfiguration, _>(Some(ConfigurationParams {
            items: vec![ConfigurationItem {
                scope_uri: None,
                section: Some("evenBetterToml".into()),
            }],
        }))
        .await
        .unwrap()
        .into_result();

    let mut config_vals = match res {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    let mut w = context.world().lock().await;

    w.configuration = serde_json::from_value(config_vals.remove(0)).unwrap_or_default();

    if !w.configuration.schema.enabled.unwrap_or_default() {
        return;
    }

    w.schema_associations.clear();

    let mut schemas: HashMap<String, RootSchema> = mem::take(&mut w.schemas);

    let base_url = w.workspace_uri.clone();
    let config = w.configuration.clone();

    drop(w);

    let mut new_schema_associatons: IndexMap<HashRegex, String> = IndexMap::new();

    if let Some(assoc) = config.schema.associations {
        for (k, s) in assoc {
            let re = match Regex::new(&k) {
                Ok(r) => r,
                Err(err) => {
                    log_error!("Invalid schema association pattern: {}", err);
                    show_schema_error(context.clone());
                    continue;
                }
            };

            new_schema_associatons.insert(HashRegex(re), s.clone());

            if schemas.contains_key(&s) {
                continue;
            }

            if s.starts_with(BUILTIN_SCHEME) && !schemas.iter().any(|(k, _)| k == &s) {
                log_error!("Invalid built-in schema: {}", s);
                show_schema_error(context.clone());
                continue;
            }

            let mut url_opts = Url::options();

            if let Some(base_url) = &base_url {
                if s.starts_with("./") {
                    url_opts = url_opts.base_url(Some(base_url));
                }
            }

            let url = match url_opts.parse(&s) {
                Ok(u) => u,
                Err(err) => {
                    log_error!("Invalid schema URL: {}", err);
                    show_schema_error(context.clone());
                    continue;
                }
            };

            match url.scheme() {
                "file" => {
                    let fpath_str = url.path();

                    let schema_bytes = match read_file(fpath_str) {
                        Ok(b) => b,
                        Err(err) => {
                            log_error!("Failed to read schema file: {:?}", err);
                            show_schema_error(context.clone());
                            continue;
                        }
                    };

                    let root_schema = match serde_json::from_slice::<RootSchema>(&schema_bytes) {
                        Ok(s) => s,
                        Err(err) => {
                            log_error!("Invalid schema: {}", err);
                            show_schema_error(context.clone());
                            continue;
                        }
                    };

                    if let Err(errors) = root_schema.verify() {
                        log_error!(
                            "Invalid schema: \n{}",
                            errors
                                .iter()
                                .map(|e| format!("{}", e))
                                .collect::<Vec<String>>()
                                .join("\n")
                        );
                        show_schema_error(context.clone());
                        continue;
                    }

                    schemas.insert(s, root_schema);
                }
                "http" | "https" => {}
                scheme => {
                    log_error!("Invalid schema URL scheme: {}", scheme);
                    show_schema_error(context.clone());
                    continue;
                }
            }
        }
    }
    let mut w = context.world().lock().await;

    if !new_schema_associatons.is_empty() {
        w.schema_associations.extend(new_schema_associatons);
    }

    w.schemas = schemas;
}

fn show_schema_error(mut context: Context<World>) {
    spawn_local(async move {
        context
            .write_notification::<request_ext::MessageWithOutput, _>(Some(MessageWithOutputParams {
                kind: MessageKind::Error,
                message: "Failed to load schema!".into(),
            }))
            .await
            .unwrap();
    });
}

pub(crate) async fn configuration_change(
    context: Context<World>,
    _params: Params<DidChangeConfigurationParams>,
) {
    update_configuration(context).await;
}

pub(crate) async fn document_open(
    mut context: Context<World>,
    params: Params<DidOpenTextDocumentParams>,
) {
    let p = match params.optional() {
        None => return,
        Some(p) => p,
    };

    let parse = taplo::parser::parse(&p.text_document.text);
    let mapper = Mapper::new(&p.text_document.text);
    let uri = p.text_document.uri.clone();

    context
        .world()
        .lock()
        .await
        .documents
        .insert(p.text_document.uri, Document { parse, mapper });

    spawn_local(diagnostics::publish_diagnostics(context.clone(), uri));
}

pub(crate) async fn document_change(
    mut context: Context<World>,
    params: Params<DidChangeTextDocumentParams>,
) {
    let mut p = match params.optional() {
        None => return,
        Some(p) => p,
    };

    // We expect one full change
    let change = match p.content_changes.pop() {
        None => return,
        Some(c) => c,
    };

    let parse = taplo::parser::parse(&change.text);
    let mapper = Mapper::new(&change.text);
    let uri = p.text_document.uri.clone();

    context
        .world()
        .lock()
        .await
        .documents
        .insert(p.text_document.uri, Document { parse, mapper });

    spawn_local(diagnostics::publish_diagnostics(context.clone(), uri));
}

pub(crate) async fn semantic_tokens(
    mut context: Context<World>,
    params: Params<SemanticTokensParams>,
) -> Result<Option<SemanticTokensResult>, Error> {
    let p = params.required()?;

    let w = context.world().lock().await;
    let doc = w
        .documents
        .get(&p.text_document.uri)
        .ok_or_else(Error::invalid_params)?;

    Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: semantic_tokens::create_tokens(&doc.parse.clone().into_syntax(), &doc.mapper),
    })))
}

pub(crate) async fn folding_ranges(
    mut context: Context<World>,
    params: Params<FoldingRangeParams>,
) -> Result<Option<Vec<FoldingRange>>, Error> {
    let p = params.required()?;

    let w = context.world().lock().await;

    let doc = w
        .documents
        .get(&p.text_document.uri)
        .ok_or_else(Error::invalid_params)?;

    Ok(Some(folding_ranges::create_folding_ranges(
        &doc.parse.clone().into_syntax(),
        &doc.mapper,
    )))
}

pub(crate) async fn document_symbols(
    mut context: Context<World>,
    params: Params<DocumentSymbolParams>,
) -> Result<Option<DocumentSymbolResponse>, Error> {
    let p = params.required()?;

    let w = context.world().lock().await;

    let doc = w
        .documents
        .get(&p.text_document.uri)
        .ok_or_else(Error::invalid_params)?;

    Ok(Some(DocumentSymbolResponse::Nested(
        document_symbols::create_symbols(&doc),
    )))
}

pub(crate) async fn format(
    mut context: Context<World>,
    params: Params<DocumentFormattingParams>,
) -> Result<Option<Vec<TextEdit>>, Error> {
    let p = params.required()?;

    let w = context.world().lock().await;

    let doc = w
        .documents
        .get(&p.text_document.uri)
        .ok_or_else(Error::invalid_params)?;

    let mut format_opts = formatter::Options::default();

    if let Some(v) = w.configuration.formatter.align_entries {
        format_opts.align_entries = v;
    }

    if let Some(v) = w.configuration.formatter.array_auto_collapse {
        format_opts.array_auto_collapse = v;
    }

    if let Some(v) = w.configuration.formatter.array_auto_expand {
        format_opts.array_auto_expand = v;
    }

    if let Some(v) = w.configuration.formatter.column_width {
        format_opts.column_width = v;
    }

    if let Some(v) = w.configuration.formatter.array_trailing_comma {
        format_opts.array_trailing_comma = v;
    }

    if let Some(v) = w.configuration.formatter.trailing_newline {
        format_opts.trailing_newline = v;
    }

    if let Some(v) = w.configuration.formatter.compact_arrays {
        format_opts.compact_arrays = v;
    }

    if let Some(v) = w.configuration.formatter.compact_inline_tables {
        format_opts.compact_inline_tables = v;
    }

    if let Some(v) = w.configuration.formatter.indent_string.clone() {
        format_opts.indent_string = v;
    } else {
        format_opts.indent_string = if p.options.insert_spaces {
            " ".repeat(p.options.tab_size as usize)
        } else {
            "\t".into()
        }
    }

    if let Some(v) = w.configuration.formatter.indent_tables {
        format_opts.indent_tables = v;
    }

    if let Some(v) = w.configuration.formatter.crlf {
        format_opts.crlf = v;
    }

    if let Some(v) = w.configuration.formatter.reorder_keys {
        format_opts.reorder_keys = v;
    }

    let mut range = doc.mapper.all_range();
    range.end.line += 1; // Make sure to cover everything

    Ok(Some(vec![TextEdit {
        range,
        new_text: taplo::formatter::format_syntax(doc.parse.clone().into_syntax(), format_opts),
    }]))
}

pub(crate) async fn completion(
    mut context: Context<World>,
    params: Params<CompletionParams>,
) -> Result<Option<CompletionResponse>, Error> {
    let p = params.required()?;

    let uri = p.text_document_position.text_document.uri;
    let pos = p.text_document_position.position;

    let w = context.world().lock().await;

    if !w.configuration.schema.enabled.unwrap_or_default() {
        return Ok(None);
    }

    let doc: Document = match w.documents.get(&uri) {
        Some(d) => d.clone(),
        None => return Err(Error::new("document not found")),
    };

    let schema: RootSchema = match w.get_schema_by_uri(&uri) {
        Some(s) => s.clone(),
        None => return Ok(None),
    };

    drop(w);

    Ok(Some(CompletionResponse::List(CompletionList {
        is_incomplete: false,
        items: completion::get_completions(doc, pos, schema),
    })))
}

pub(crate) async fn hover(
    mut context: Context<World>,
    params: Params<HoverParams>,
) -> Result<Option<Hover>, Error> {
    let p = params.required()?;

    let uri = p.text_document_position_params.text_document.uri;
    let pos = p.text_document_position_params.position;

    let w = context.world().lock().await;

    if !w.configuration.schema.enabled.unwrap_or_default() {
        return Ok(None);
    }

    let doc: Document = match w.documents.get(&uri) {
        Some(d) => d.clone(),
        None => return Err(Error::new("document not found")),
    };

    let schema: RootSchema = match w.get_schema_by_uri(&uri) {
        Some(s) => s.clone(),
        None => return Ok(None),
    };

    let info = PositionInfo::new(doc, pos);

    let range = info.node.as_ref().and_then(|n| match n {
        taplo::dom::Node::Key(k) => info.doc.mapper.range(k.text_range()),
        _ => None,
    });

    let schemas = get_schema_objects(info.keys, &schema);

    Ok(schemas
        .first()
        .and_then(|s| {
            s.ext
                .docs
                .as_ref()
                .and_then(|docs| docs.main.clone())
                .or_else(|| {
                    s.schema
                        .metadata
                        .as_ref()
                        .and_then(|meta| meta.description.clone())
                })
        })
        .and_then(|desc| range.map(|range| (desc, range)))
        .map(|(value, range)| Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value,
            }),
            range: Some(range),
        }))
}

pub(crate) async fn links(
    mut context: Context<World>,
    params: Params<DocumentLinkParams>,
) -> Result<Option<Vec<DocumentLink>>, Error> {
    let p = params.required()?;

    let uri = p.text_document.uri;

    let w = context.world().lock().await;

    if !w.configuration.schema.enabled.unwrap_or_default() {
        return Ok(None);
    }

    let doc: Document = match w.documents.get(&uri) {
        Some(d) => d.clone(),
        None => return Err(Error::new("document not found")),
    };

    let schema: RootSchema = match w.get_schema_by_uri(&uri) {
        Some(s) => s.clone(),
        None => return Ok(None),
    };

    let dom = doc.parse.clone().into_dom();

    let keys = collect_keys(&dom.into(), Vec::new());

    let mut links = Vec::with_capacity(keys.len());

    for key in keys {
        let mut all_keys = key.parent_keys;
        all_keys.push(Key::Property(key.key.full_key_string()));

        let link = get_schema_objects(all_keys, &schema)
            .first()
            .and_then(|s| s.ext.links.as_ref().and_then(|links| links.key.clone()));

        if let Some(link) = link {
            let target = match Url::parse(&link) {
                Ok(u) => u,
                Err(e) => {
                    log_error!("invalid link in schema: {}", e);
                    continue;
                }
            };

            let range = doc.mapper.range(key.key.text_range()).unwrap();

            links.push(DocumentLink {
                range,
                target,
                tooltip: None,
            })
        }
    }

    Ok(Some(links))
}

pub(crate) async fn toml_to_json(
    _context: Context<World>,
    params: Params<TomlToJsonParams>,
) -> Result<TomlToJsonResponse, Error> {
    let p = params.required()?;

    let parse = taplo::parser::parse(&p.text);

    if !parse.errors.is_empty() {
        return Ok(TomlToJsonResponse {
            text: None,
            errors: Some(parse.errors.iter().map(|e| e.to_string()).collect()),
        });
    }

    let dom = parse.into_dom();

    if !dom.errors().is_empty() {
        return Ok(TomlToJsonResponse {
            text: None,
            errors: Some(dom.errors().iter().map(|e| e.to_string()).collect()),
        });
    }

    let val = taplo::value::Value::try_from(dom).unwrap();

    Ok(TomlToJsonResponse {
        text: Some(serde_json::to_string_pretty(&val).unwrap()),
        errors: None,
    })
}

pub(crate) async fn line_mappings(
    mut context: Context<World>,
    params: Params<LineMappingsParams>,
) -> Result<LineMappingsResponse, Error> {
    let p = params.required()?;

    let w = context.world().lock().await;

    let doc = w.documents.get(&p.uri).ok_or_else(Error::invalid_params)?;

    Ok(LineMappingsResponse {
        lines: doc
            .mapper
            .lines()
            .iter()
            .map(|r| format!("{:?}", r))
            .collect(),
    })
}

pub(crate) async fn syntax_tree(
    mut context: Context<World>,
    params: Params<SyntaxTreeParams>,
) -> Result<SyntaxTreeResponse, Error> {
    let p = params.required()?;

    let w = context.world().lock().await;

    let doc = w.documents.get(&p.uri).ok_or_else(Error::invalid_params)?;

    Ok(SyntaxTreeResponse {
        text: format!("{:#?}", doc.parse.clone().into_syntax()),
    })
}

pub(crate) async fn dom_tree(
    mut context: Context<World>,
    params: Params<DomTreeParams>,
) -> Result<DomTreeResponse, Error> {
    let p = params.required()?;

    let w = context.world().lock().await;

    let doc = w.documents.get(&p.uri).ok_or_else(Error::invalid_params)?;

    Ok(DomTreeResponse {
        text: format!("{:#?}", doc.parse.clone().into_dom()),
    })
}
