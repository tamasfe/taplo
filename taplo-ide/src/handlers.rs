use crate::{file_exists, request_ext::*};
use crate::{is_absolute_path, request_ext};
use crate::{read_file, Configuration, Document, HashRegex, World};
use anyhow::anyhow;
use indexmap::IndexMap;
use itertools::Itertools;
use lsp_async_stub::{rpc::Error, Context, Params, RequestWriter};
use lsp_types::*;
use regex::Regex;
use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom, mem};
use taplo::{
    analytics::NodeRef,
    formatter,
    schema::{util::get_schema_objects, BUILTIN_SCHEMAS, BUILTIN_SCHEME, REGEX_ASSOCIATIONS},
    util::{coords::Mapper, syntax::join_ranges},
    value::Value,
};
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

    drop(w);

    // safety: guaranteed to execute only after this function
    // has returned, and the reply has been already sent.
    spawn_local(async move {
        context
            .write_notification::<request_ext::UpdateBuiltInSchemas, _>(Some(
                UpdateBuiltInSchemasParams {
                    associations: REGEX_ASSOCIATIONS.clone(),
                },
            ))
            .await
            .unwrap();
    });

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

pub(crate) async fn config_file_changed(mut context: Context<World>, _: Params<()>) {
    spawn_local(async move {
        if let Err(err) = load_config(context.clone()).await {
            log_error!("{}", err);
            context
                .write_notification::<request_ext::MessageWithOutput, _>(Some(
                    MessageWithOutputParams {
                        kind: MessageKind::Error,
                        message: "Failed to load configuration!".into(),
                    },
                ))
                .await
                .unwrap();
        }
    });
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

    let config = w.configuration.clone();

    drop(w);

    let mut new_schema_associations: IndexMap<HashRegex, String> = IndexMap::new();

    if let Some(assoc) = config.schema.associations {
        for (k, s) in assoc {
            if is_deprecated_schema(&s) {
                log_warn!("Schema is deprecated, ignoring it ({})", &s);
                continue;
            }

            let re = match Regex::new(&k) {
                Ok(r) => r,
                Err(err) => {
                    log_error!("Invalid schema association pattern: {}", err);
                    show_schema_error(context.clone());
                    continue;
                }
            };

            // Save the schema association
            new_schema_associations.insert(HashRegex(re), s.clone());

            // Then get the schema itself.

            if schemas.contains_key(&s) {
                continue;
            }

            if s.starts_with(&format!("{}://", BUILTIN_SCHEME))
                && !schemas.iter().any(|(k, _)| k == &s)
            {
                log_error!("Invalid built-in schema: {}", s);
                show_schema_error(context.clone());
                continue;
            }

            match get_schema(&s, context.clone()).await {
                Ok(schema) => {
                    schemas.insert(s, schema);
                }
                Err(err) => {
                    log_error!("Failed to load schema: {}", err);
                    show_schema_error(context.clone());
                }
            }
        }
    }
    let mut w = context.world().lock().await;

    if !new_schema_associations.is_empty() {
        w.schema_associations.extend(new_schema_associations);
    }

    w.schemas = schemas;
    drop(w);

    if let Err(e) = load_config(context.clone()).await {
        log_error!("failed to load configuration file: {}", e);

        let mut c = context.clone();
        spawn_local(async move {
            c.write_notification::<request_ext::MessageWithOutput, _>(Some(
                MessageWithOutputParams {
                    kind: MessageKind::Error,
                    message: "Failed to load configuration!".into(),
                },
            ))
            .await
            .unwrap();
        });
    }
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
    spawn_local(update_configuration(context));
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
    let mapper = Mapper::new_utf16(&p.text_document.text, false);
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
    let mapper = Mapper::new_utf16(&change.text, false);
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

    let (format_opts, scopes) = w.get_config_formatter_options(&p.text_document.uri, format_opts);

    Ok(Some(vec![TextEdit {
        range: doc.mapper.all_range(),
        new_text: taplo::formatter::format_with_path_scopes(
            doc.parse.clone().into_dom(),
            format_opts,
            scopes,
        ),
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

    let schemas = get_schema_objects(query.after.path, &schema, true);
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
            NodeRef::Value(v) => {
                let val = match Value::try_from(node.into_node()) {
                    Ok(val) => val,
                    Err(_) => {
                        return None;
                    }
                };

                match serde_json::to_value(val) {
                    Ok(toml_value) => {
                        let docs = schemas
                            .into_iter()
                            .filter_map(|s| {
                                let enum_doc =
                                    s.schema.enum_values.as_ref().and_then(|enum_values| {
                                        enum_values.iter().enumerate().find_map(
                                            |(enum_idx, enum_value)| {
                                                if *enum_value == toml_value {
                                                    s.ext.docs.as_ref().and_then(|i| {
                                                        i.enum_values.as_ref().and_then(|e| {
                                                            e.get(enum_idx)
                                                                .map(|doc| (enum_idx, doc))
                                                        })
                                                    })
                                                } else {
                                                    None
                                                }
                                            },
                                        )
                                    });

                                let default_value_doc =
                                    s.schema.metadata.as_ref().and_then(|meta| {
                                        meta.default.as_ref().and_then(|def| {
                                            if *def == toml_value {
                                                s.ext
                                                    .docs
                                                    .as_ref()
                                                    .and_then(|i| i.default_value.clone())
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
                                range: Some(
                                    doc.mapper.range(join_ranges(v.text_ranges())).unwrap(),
                                ),
                            })
                        }
                    }
                    Err(err) => {
                        log_debug!("invalid JSON value from TOML value: {}", err);
                        None
                    }
                }
            }
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
            let schemas = get_schema_objects(path, &schema, true);

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
                let schemas = get_schema_objects(path, &schema, true);

                let val = match Value::try_from(node.into_node()) {
                    Ok(val) => val,
                    Err(_) => {
                        return None;
                    }
                };

                if schemas.is_empty() {
                    None
                } else {
                    match serde_json::to_value(val) {
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

pub(crate) async fn get_schema(
    mut path: &str,
    mut context: Context<World>,
) -> Result<RootSchema, anyhow::Error> {
    if path.starts_with(&format!("{}://", BUILTIN_SCHEME)) {
        match BUILTIN_SCHEMAS.get(path) {
            Some(s) => Ok(s.clone()),
            None => Err(anyhow!("builtin schema was not found: {}", path)),
        }
    } else if path.starts_with("http://") || path.starts_with("https://") {
        let schema_uri = Url::parse(path)?;
        let cached_res = context
            .write_request::<request_ext::GetCachedSchemaRequest, _>(Some(GetCachedSchemaParams {
                schema_uri: schema_uri.clone(),
            }))
            .await
            .unwrap()
            .into_result()?;

        if let Some(s) = cached_res.schema_json {
            return serde_json::from_str(&s).map_err(Into::into);
        }

        let w = context.world().lock().await;
        let client = w.http_client.clone();
        drop(w);

        let res = client.get(path).send().await?;
        let schema: RootSchema = res.json().await.map_err::<anyhow::Error, _>(Into::into)?;

        context
            .write_notification::<request_ext::CacheSchemaRequest, _>(Some(CacheSchemaParams {
                schema_uri,
                schema_json: serde_json::to_string(&schema)?,
            }))
            .await
            .unwrap();

        Ok(schema)
    } else {
        if path.starts_with("file://") {
            path = path.trim_start_matches("file://");
            serde_json::from_slice(unsafe { &read_file(path).map_err(|e| anyhow!("{:?}", e))? })
                .map_err(Into::into)
        } else {
            if unsafe { is_absolute_path(path) } {
                serde_json::from_slice(unsafe { &read_file(path).map_err(|e| anyhow!("{:?}", e))? })
                    .map_err(Into::into)
            } else {
                match context.world().lock().await.workspace_absolute(path) {
                    Some(p) => serde_json::from_slice(unsafe {
                        &read_file(p.to_str().unwrap()).map_err(|e| anyhow!("{:?}", e))?
                    })
                    .map_err(Into::into),
                    None => Err(anyhow!("cannot determine workspace root for relative path")),
                }
            }
        }
    }
}

fn is_deprecated_schema(s: &str) -> bool {
    s.starts_with("toml_builtin://")
}

async fn load_config_file(mut context: Context<World>) -> Result<(), anyhow::Error> {
    let mut w = context.world().lock().await;
    w.taplo_config = None;

    if !w.configuration.taplo_config_enabled.unwrap_or(false) {
        return Ok(());
    }

    if let Some(config_path) = &w.configuration.taplo_config {
        if !config_path.is_empty() {
            if unsafe { is_absolute_path(&config_path) } {
                let f = unsafe { read_file(&config_path).map_err(|e| anyhow!("{:?}", e))? };

                w.taplo_config = Some(toml::from_slice(&f)?);

                return Ok(());
            } else {
                let workspace_path = w.workspace_path();

                match workspace_path {
                    Some(ws_path) => {
                        let f = unsafe {
                            read_file(ws_path.join(&config_path).to_str().unwrap())
                                .map_err(|e| anyhow!("{:?}", e))?
                        };

                        w.taplo_config = Some(toml::from_slice(&f)?);

                        return Ok(());
                    }
                    None => {
                        log_warn!("couldn't load workspace relative config, as there is no workspace open");
                        return Ok(());
                    }
                }
            }
        }
    }

    let workspace_path = w.workspace_path();

    match workspace_path {
        Some(ws_path) => {
            for name in taplo_cli::config::CONFIG_FILE_NAMES {
                if unsafe { file_exists(ws_path.join(name).to_str().unwrap()) } {
                    if let Ok(cfg_file) = unsafe { read_file(ws_path.join(name).to_str().unwrap()) }
                    {
                        w.taplo_config = Some(toml::from_slice(&cfg_file)?);
                        drop(w);

                        spawn_local(async move {
                            context
                                .write_notification::<request_ext::WatchConfigFile, _>(Some(
                                    WatchConfigFileParams {
                                        config_path: ws_path.join(name).to_str().unwrap().into(),
                                    },
                                ))
                                .await
                                .unwrap();
                        });

                        return Ok(());
                    }
                }
            }
            Ok(())
        }
        None => Ok(()),
    }
}

async fn load_config(mut context: Context<World>) -> Result<(), anyhow::Error> {
    load_config_file(context.clone()).await?;

    let w = context.world().lock().await;
    let c = w.taplo_config.clone();
    // FIXME(perf)
    let old_schemas = w.schemas.clone();
    drop(w);

    let mut new_schemas: HashMap<String, RootSchema> = HashMap::new();

    if let Some(c) = c {
        for schema_name in c.collect_schemas() {
            if old_schemas.contains_key(&schema_name) {
                continue;
            }

            match get_schema("", context.clone()).await {
                Ok(s) => {
                    new_schemas.insert(schema_name, s);
                }
                Err(err) => {
                    log_error!("Failed to load schema: {}", err);
                    show_schema_error(context.clone());
                }
            }
        }
    }

    let mut w = context.world().lock().await;

    w.schemas.extend(new_schemas);

    Ok(())
}
