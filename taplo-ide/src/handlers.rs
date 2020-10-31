use crate::request_ext::*;
use crate::{
    read_file,
    schema::{get_schema_objects, BUILTIN_SCHEME},
    Configuration, Document, HashRegex, World,
};
use crate::{request_ext, schema::ExtendedSchema};
use indexmap::IndexMap;
use itertools::Itertools;
use lsp_async_stub::{rpc::Error, Context, Params, RequestWriter};
use lsp_types::*;
use regex::Regex;
use rowan::TextRange;
use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{collections::HashMap, convert::TryFrom, mem};
use taplo::{
    analytics::NodeRef,
    dom::{NodeSyntax, TextRanges},
    formatter,
    util::{coords::Mapper, syntax::join_ranges},
};
use verify::Verify;
use wasm_bindgen_futures::spawn_local;

mod completion;
mod diagnostics;
mod document_symbols;
mod folding_ranges;
mod semantic_tokens;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct InitializationOptions {
    configuration: Option<Configuration>,
}

pub(crate) async fn initialize(
    mut context: Context<World>,
    params: Params<InitializeParams>,
) -> Result<InitializeResult, Error> {
    let p = params.required()?;

    let mut w = context.world().lock().await;

    w.workspace_uri = p.root_uri.map(|mut uri| {
        uri.set_path(&(uri.path().to_string() + "/"));
        uri
    });

    if let Some(opts_val) = p.initialization_options {
        let opts: InitializationOptions = serde_json::from_value(opts_val)
            .map_err(|e| Error::new(&format!("invalid initialization options: {}", e)))?;

        if let Some(config) = opts.configuration {
            w.configuration = config;
        }
    }

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

    let mut new_schema_associations: IndexMap<HashRegex, String> = IndexMap::new();

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

            new_schema_associations.insert(HashRegex(re), s.clone());

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
                    // unsafe: Extern JS call
                    let schema_bytes = match unsafe { read_file(fpath_str) } {
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

    if !new_schema_associations.is_empty() {
        w.schema_associations.extend(new_schema_associations);
    }

    w.schemas = schemas;
}

fn show_schema_error(mut context: Context<World>) {
    spawn_local(async move {
        context
            .write_notification::<request_ext::MessageWithOutput, _>(Some(
                MessageWithOutputParams {
                    kind: MessageKind::Error,
                    message: "Failed to load schema!".into(),
                },
            ))
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
    let mapper = Mapper::new(&p.text_document.text).zero_based(true);
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
    let mapper = Mapper::new(&change.text).zero_based(true);
    let uri = p.text_document.uri.clone();

    context
        .world()
        .lock()
        .await
        .documents
        .insert(p.text_document.uri, Document { parse, mapper });

    spawn_local(diagnostics::publish_diagnostics(context.clone(), uri));
}

pub(crate) async fn document_close(
    mut context: Context<World>,
    params: Params<DidCloseTextDocumentParams>,
) {
    let p = match params.optional() {
        None => return,
        Some(p) => p,
    };

    context
        .world()
        .lock()
        .await
        .documents
        .remove(&p.text_document.uri);

    spawn_local(diagnostics::clear_diagnostics(context, p.text_document.uri));
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

    if !w.configuration.semantic_tokens.unwrap_or_default() {
        return Ok(None);
    }

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

    format_opts.update_camel(w.configuration.formatter.clone());

    if let Some(v) = w.configuration.formatter.indent_string.clone() {
        format_opts.indent_string = v;
    } else {
        format_opts.indent_string = if p.options.insert_spaces {
            " ".repeat(p.options.tab_size as usize)
        } else {
            "\t".into()
        }
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

    let dom = doc.parse.clone().into_dom();

    let query = dom.query_position(doc.mapper.offset(pos).unwrap());

    let schemas = get_schema_objects(query.after.path, &schema);
    let syntax_range = query.after.syntax.range.clone();

    Ok(query
        .after
        .nodes
        .into_iter()
        // We only care about keys and values.
        .filter(|n| n.is_key() || n.is_value())
        // We make sure that we work with only the most accurate key.
        .filter(|n| match n {
            NodeRef::Key(k) => k.key_count() == 1,
            _ => true,
        })
        .last()
        .and_then(|node| match node {
            NodeRef::Key(k) => {
                let docs = schemas
                    .into_iter()
                    .filter_map(|s| {
                        let docs = s.ext.docs.as_ref().and_then(|d| d.main.clone());
                        let link = s.ext.links.as_ref().and_then(|l| l.key.clone());

                        let docs = docs.or_else(|| {
                            s.schema
                                .metadata
                                .as_ref()
                                .and_then(|m| m.description.clone())
                        });

                        match docs {
                            Some(d) => Some((d, link)),
                            None => None,
                        }
                    })
                    .map(|(mut docs, link)| {
                        if !w.configuration.schema.enabled.unwrap_or(false)
                            || !w.configuration.schema.links.unwrap_or(false)
                        {
                            if let Some(link) = link {
                                docs = format!(
                                    "[_<sup>more information</sup>_]({})\n\n{}",
                                    link, docs
                                );
                            }
                        }

                        docs
                    })
                    .fold(String::new(), |mut all, s| {
                        if !all.is_empty() {
                            all += "\n---\n";
                        }
                        all += &s;
                        all
                    });

                if docs.is_empty() {
                    None
                } else {
                    Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: docs,
                        }),
                        range: Some(
                            doc.mapper
                                .range(
                                    k.text_ranges()
                                        .into_iter()
                                        .find(|i| syntax_range.unwrap().contains_range(*i))
                                        .unwrap(),
                                )
                                .unwrap(),
                        ),
                    })
                }
            }
            NodeRef::Value(v) => match serde_json::to_value(node.into_node().to_value()) {
                Ok(toml_value) => {
                    let docs = schemas
                        .into_iter()
                        .filter_map(|s| {
                            let enum_doc = s.schema.enum_values.as_ref().and_then(|enum_values| {
                                enum_values
                                    .iter()
                                    .enumerate()
                                    .find_map(|(enum_idx, enum_value)| {
                                        if *enum_value == toml_value {
                                            s.ext.docs.as_ref().and_then(|i| {
                                                i.enum_values.as_ref().and_then(|e| {
                                                    e.get(enum_idx).map(|doc| (enum_idx, doc))
                                                })
                                            })
                                        } else {
                                            None
                                        }
                                    })
                            });

                            let default_value_doc = s.schema.metadata.as_ref().and_then(|meta| {
                                meta.default.as_ref().and_then(|def| {
                                    if *def == toml_value {
                                        s.ext.docs.as_ref().and_then(|i| i.default_value.clone())
                                    } else {
                                        None
                                    }
                                })
                            });

                            if let Some((idx, enum_doc)) = enum_doc {
                                if let Some(enum_doc) = enum_doc {
                                    let link = s.ext.links.as_ref().and_then(|l| {
                                        l.enum_values.as_ref().and_then(|e| e.get(idx)).clone()
                                    });

                                    let link = match link {
                                        Some(l) => l.clone(),
                                        None => None,
                                    };

                                    return Some((enum_doc.clone(), link));
                                }
                            } else if let Some(d) = default_value_doc {
                                return Some((d, None));
                            }
                            None
                        })
                        .map(|(mut docs, link)| {
                            if !w.configuration.schema.enabled.unwrap_or(false)
                                || !w.configuration.schema.links.unwrap_or(false)
                            {
                                if let Some(link) = link {
                                    docs = format!(
                                        "[_<sup>more information</sup>_]({})\n\n{}",
                                        link, docs
                                    );
                                }
                            }

                            docs
                        })
                        .fold(String::new(), |mut all, s| {
                            if !all.is_empty() {
                                all += "\n---\n";
                            }
                            all += &s;
                            all
                        });

                    if docs.is_empty() {
                        None
                    } else {
                        Some(Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: docs,
                            }),
                            range: Some(doc.mapper.range(join_ranges(v.text_ranges())).unwrap()),
                        })
                    }
                }
                Err(err) => {
                    log_debug!("invalid JSON value from TOML value: {}", err);
                    None
                }
            },
            _ => None,
        }))
}

pub(crate) async fn links(
    mut context: Context<World>,
    params: Params<DocumentLinkParams>,
) -> Result<Option<Vec<DocumentLink>>, Error> {
    let p = params.required()?;

    let uri = p.text_document.uri;

    let w = context.world().lock().await;

    if !w.configuration.schema.enabled.unwrap_or(false)
        || !w.configuration.schema.links.unwrap_or(false)
    {
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

    let mut links: Vec<DocumentLink> = Vec::new();

    let key_links = dom
        .iter()
        .filter(|(_, n)| n.is_key())
        .unique_by(|(p, _)| p.clone())
        .filter_map(|(path, node)| {
            let schemas = get_schema_objects(path, &schema);

            if schemas.is_empty() {
                None
            } else {
                Some(
                    schemas
                        .into_iter()
                        .filter_map(|s| s.ext.links.as_ref().and_then(|links| links.key.clone()))
                        .unique()
                        .map(move |link| (link, node.clone()))
                        .filter_map(|(link, node)| {
                            let target = match Url::parse(&link) {
                                Ok(u) => u,
                                Err(e) => {
                                    log_error!("invalid link in schema: {}", e);
                                    return None;
                                }
                            };

                            Some(
                                node.clone()
                                    .text_ranges()
                                    .iter()
                                    .map(|text_range| DocumentLink {
                                        range: doc.mapper.range(*text_range).unwrap(),
                                        target: target.clone(),
                                        tooltip: None,
                                    })
                                    .collect::<Vec<DocumentLink>>(),
                            )
                        }),
                )
            }
        });

    let value_links =
        dom.iter()
            .filter(|(_, n)| n.is_value())
            .unique_by(|(p, _)| p.clone())
            .filter_map(|(path, node)| {
                let schemas = get_schema_objects(path, &schema);

                if schemas.is_empty() {
                    None
                } else {
                    match serde_json::to_value(node.into_node().to_value()) {
                        Ok(toml_value) => Some(
                            schemas
                                .into_iter()
                                .filter_map(move |s| {
                                    s.schema
                                        .enum_values
                                        .as_ref()
                                        .and_then(|enum_values| {
                                            enum_values
                                                .iter()
                                                .enumerate()
                                                .find(|(_, e)| *e == &toml_value)
                                        })
                                        .and_then(|(idx, _)| {
                                            s.ext.links.as_ref().and_then(|links| {
                                                links.enum_values.as_ref().and_then(|enum_docs| {
                                                    enum_docs.get(idx).cloned()
                                                })
                                            })
                                        })
                                })
                                .filter_map(|doc| match doc {
                                    Some(doc) => Some(doc.clone()),
                                    None => None,
                                })
                                .unique()
                                .map(move |link| (link, node.clone()))
                                .filter_map(|(link, node)| {
                                    let target = match Url::parse(&link) {
                                        Ok(u) => u,
                                        Err(e) => {
                                            log_error!("invalid link in schema: {}", e);
                                            return None;
                                        }
                                    };

                                    Some(
                                        node.clone()
                                            .text_ranges()
                                            .iter()
                                            .map(|text_range| DocumentLink {
                                                range: doc.mapper.range(*text_range).unwrap(),
                                                target: target.clone(),
                                                tooltip: None,
                                            })
                                            .collect::<Vec<DocumentLink>>(),
                                    )
                                }),
                        ),
                        Err(err) => {
                            log_debug!("invalid JSON value from TOML value: {}", err);
                            None
                        }
                    }
                }
            });

    links.extend(key_links.flatten().flatten());
    links.extend(value_links.flatten().flatten());

    if links.is_empty() {
        Ok(None)
    } else {
        Ok(Some(links))
    }
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
