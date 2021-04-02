use crate::{
    external::*,
    msg_ext::{self, CachePathParams},
    utils::LspExt,
    Configuration, Document, HashRegex, World, WorldState,
};
use hex::ToHex;
use itertools::Itertools;
use lsp_async_stub::{rpc::Error, Context, Params, RequestWriter};
use lsp_types::*;
use regex::Regex;
use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, convert::TryFrom};
use taplo::{
    analytics::NodeRef,
    formatter,
    schema::{util::get_schema_objects, CachedSchema, SchemaIndex},
    util::{coords::Mapper, syntax::join_ranges},
    value::Value,
};

mod code_action;
mod completion;
mod diagnostics;
mod document_symbols;
mod folding_ranges;
mod semantic_tokens;

pub(crate) use code_action::code_action;

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
                    full: Some(SemanticTokensFullOptions::Bool(true)),
                    range: Some(false),
                }),
            ),
            code_action_provider: Some(CodeActionProviderCapability::Options(CodeActionOptions {
                code_action_kinds: Some(vec![CodeActionKind::REFACTOR]),
                resolve_provider: None,
                work_done_progress_options: Default::default(),
            })),
            folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
            document_symbol_provider: Some(OneOf::Left(true)),
            document_formatting_provider: Some(OneOf::Left(true)),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
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

pub(crate) async fn cache_path(mut context: Context<World>, params: Params<CachePathParams>) {
    if let Some(params) = params.optional() {
        let mut w = context.world().lock().await;
        w.cache_path = Some(params.path.into());
    }
}

async fn update_configuration(mut context: Context<World>, configuration: Option<Configuration>) {
    let config = match configuration {
        Some(c) => c,
        None => {
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
                Err(e) => panic!("{}", e),
            };

            serde_json::from_value(config_vals.remove(0)).unwrap_or_default()
        }
    };

    let mut w = context.world().lock().await;

    w.configuration = config;

    if !w.configuration.schema.enabled.unwrap_or_default() {
        return;
    }

    if w.configuration.schema.associations.is_some() {
        w.schema_associations.clear();
    }

    if let Some(assoc) = &w.configuration.schema.associations {
        w.schema_associations = assoc
            .iter()
            .filter_map(|(k, v)| match Regex::new(k) {
                Ok(re) => Some((HashRegex(re), v.clone())),
                Err(err) => {
                    log_error!("invalid schema pattern: {}", err);
                    None
                }
            })
            .collect();
    }

    drop(w);

    if let Err(e) = load_config_file(context.clone()).await {
        log_error!("failed to load configuration file: {}", e);

        let mut c = context.clone();
        spawn(async move {
            c.write_notification::<msg_ext::MessageWithOutput, _>(Some(
                msg_ext::MessageWithOutputParams {
                    kind: msg_ext::MessageKind::Error,
                    message: "Failed to load configuration!".into(),
                },
            ))
            .await
            .unwrap();
        });
    }

    let mut w = context.world().lock().await;

    let mut index = None;

    if !w.configuration.schema.repository_enabled.unwrap_or(true) {
        return;
    }

    if let Some(index_url) = &w.configuration.schema.repository_url {
        match w.http_client.clone().get(index_url).send().await {
            Ok(res) => match res.json::<SchemaIndex>().await {
                Ok(idx) => {
                    index = Some(idx);

                    if let Some(cache_path) = &w.cache_path {
                        match mkdir(cache_path.to_str().unwrap()) {
                            Ok(_) => {
                                if let Err(err) = write_file(
                                    cache_path
                                        .join("schema_index")
                                        .with_extension("json")
                                        .to_str()
                                        .unwrap(),
                                    &serde_json::to_vec(&index).unwrap(),
                                )
                                .await
                                {
                                    log_error!("failed to save schema index: {}", err);
                                }
                            }
                            Err(err) => {
                                log_error!("failed to save schema index: {}", err);
                            }
                        }
                    }
                }
                Err(err) => {
                    log_error!("invalid schema index: {}", err);
                }
            },
            Err(err) => {
                log_error!("failed to download schema index: {}", err);
            }
        }
    }

    if index.is_none() {
        if let Some(cache_path) = &w.cache_path {
            let index_path = cache_path.join("schema_index").with_extension("json");

            if file_exists(index_path.to_str().unwrap()) {
                match read_file(index_path.to_str().unwrap()).await {
                    Ok(data) => match serde_json::from_slice(&data) {
                        Ok(idx) => index = Some(idx),
                        Err(err) => {
                            log_error!("invalid schema index (cached): {}", err);
                        }
                    },
                    Err(err) => {
                        log_error!("failed invalid schema index: {}", err);
                    }
                }
            }
        }
    }

    if let Some(index) = index {
        for schema in index.schemas {
            for pat in &schema.extra.patterns {
                match Regex::new(pat) {
                    Ok(re) => {
                        w.index_schema_associations
                            .insert(HashRegex(re), schema.url.clone());
                    }
                    Err(err) => {
                        log_error!(
                            r#"invalid pattern for schema "{}" ({}): {}"#,
                            schema.title.as_ref().map(|s| s.as_str()).unwrap_or(""),
                            schema.url,
                            err
                        );
                    }
                };
            }

            let client = w.http_client.clone();

            if let Some(updated) = schema.updated {
                match time::OffsetDateTime::parse(updated, time::Format::Rfc3339) {
                    Ok(updated) => {
                        if let Some(cache_path) = w.cache_path.clone() {
                            let mut hasher = Sha256::new();
                            hasher.update(schema.url.as_bytes());
                            let url_hash = hasher.finalize().encode_hex::<String>();

                            let file_path = cache_path
                                .join("schemas")
                                .join(&url_hash)
                                .with_extension("json");
                            let fp = file_path.to_str().unwrap();

                            if let Ok(true) =
                                needs_update(fp, (updated.unix_timestamp() * 1000) as u64)
                            {
                                let path = schema.url;

                                spawn(async move {
                                    let res = client.get(&path).send().await.unwrap();
                                    let schema: RootSchema = res
                                        .json()
                                        .await
                                        .map_err::<anyhow::Error, _>(Into::into)
                                        .unwrap();

                                    let s = schema.clone();

                                    match mkdir(cache_path.join("schemas").to_str().unwrap()) {
                                        Ok(_) => {
                                            spawn(async move {
                                                if let Err(err) = write_file(
                                                    cache_path
                                                        .join("schemas")
                                                        .join(&url_hash)
                                                        .with_extension("json")
                                                        .to_str()
                                                        .unwrap(),
                                                    &serde_json::to_vec(&CachedSchema {
                                                        url: Some(path),
                                                        schema: s,
                                                    })
                                                    .unwrap(),
                                                )
                                                .await
                                                {
                                                    log_error!("failed to update schema: {}", err);
                                                };
                                            });
                                        }
                                        Err(err) => {
                                            log_error!("failed to update schema: {}", err);
                                        }
                                    }
                                });
                            }
                        }
                    }
                    Err(err) => {
                        log_error!("schema update date is invalid ({}): {}", &schema.url, err);
                    }
                }
            }
        }
    }
}

pub(crate) async fn configuration_change(
    context: Context<World>,
    _params: Params<DidChangeConfigurationParams>,
) {
    spawn(update_configuration(context, None));
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

    spawn(diagnostics::publish_diagnostics(context.clone(), uri));
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

    spawn(diagnostics::publish_diagnostics(context.clone(), uri));
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

    spawn(diagnostics::clear_diagnostics(context, p.text_document.uri));
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
        range: doc.mapper.all_range().into_lsp(),
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

    let schema_path = match w.get_schema_name(&uri) {
        Some(s) => s,
        None => return Ok(None),
    };

    drop(w);

    let schema: RootSchema = match WorldState::get_schema(&schema_path, context.clone()).await {
        Ok(s) => s,
        Err(err) => {
            log_error!("failed to load schema ({}): {}", &schema_path, err);
            return Ok(None);
        }
    };

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

    let schema_path = match w.get_schema_name(&uri) {
        Some(s) => s,
        None => return Ok(None),
    };

    drop(w);

    let schema: RootSchema = match WorldState::get_schema(&schema_path, context.clone()).await {
        Ok(s) => s,
        Err(err) => {
            log_error!("failed to load schema ({}): {}", &schema_path, err);
            return Ok(None);
        }
    };

    let w = context.world().lock().await;

    let dom = doc.parse.clone().into_dom();

    let query = dom.query_position(
        doc.mapper
            .offset(taplo::util::coords::Position::from_lsp(pos))
            .unwrap(),
    );

    let schemas = get_schema_objects(query.after.path, &schema, true);
    let syntax_range = query.after.syntax.range;

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
                                .unwrap()
                                .into_lsp(),
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
                                            l.enum_values.as_ref().and_then(|e| e.get(idx))
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
                                    doc.mapper
                                        .range(join_ranges(v.text_ranges()))
                                        .unwrap()
                                        .into_lsp(),
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

    let schema_path = match w.get_schema_name(&uri) {
        Some(s) => s,
        None => return Ok(None),
    };

    drop(w);

    let schema: RootSchema = match WorldState::get_schema(&schema_path, context.clone()).await {
        Ok(s) => s,
        Err(err) => {
            log_error!("failed to load schema: {}", err);
            return Ok(None);
        }
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
                        .map(move |link| (link, node))
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
                                        range: doc.mapper.range(*text_range).unwrap().into_lsp(),
                                        data: None,
                                        target: Some(target.clone()),
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
                                    Some(doc) => Some(doc),
                                    None => None,
                                })
                                .unique()
                                .map(move |link| (link, node))
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
                                                range: doc
                                                    .mapper
                                                    .range(*text_range)
                                                    .unwrap()
                                                    .into_lsp(),
                                                data: None,
                                                target: Some(target.clone()),
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
    params: Params<msg_ext::TomlToJsonParams>,
) -> Result<msg_ext::TomlToJsonResponse, Error> {
    let p = params.required()?;

    let parse = taplo::parser::parse(&p.text);

    if !parse.errors.is_empty() {
        return Ok(msg_ext::TomlToJsonResponse {
            text: None,
            errors: Some(parse.errors.iter().map(|e| e.to_string()).collect()),
        });
    }

    let dom = parse.into_dom();

    if !dom.errors().is_empty() {
        return Ok(msg_ext::TomlToJsonResponse {
            text: None,
            errors: Some(dom.errors().iter().map(|e| e.to_string()).collect()),
        });
    }

    let val = taplo::value::Value::try_from(dom).unwrap();

    Ok(msg_ext::TomlToJsonResponse {
        text: Some(serde_json::to_string_pretty(&val).unwrap()),
        errors: None,
    })
}

// Required because JSON Keys can be in any order,
// and toml-rs cannot handle it by default.
#[derive(Default, Serialize, Deserialize)]
#[serde(transparent)]
struct JsonVal(
    #[serde(serialize_with = "toml::ser::tables_last")] HashMap<String, serde_json::Value>,
);

pub(crate) async fn json_to_toml(
    _context: Context<World>,
    params: Params<msg_ext::JsonToTomlParams>,
) -> Result<msg_ext::JsonToTomlResponse, Error> {
    let p = params.required()?;

    match serde_json::from_str::<JsonVal>(&p.text) {
        Ok(v) => match toml::to_string_pretty(&v) {
            Ok(v) => Ok(msg_ext::JsonToTomlResponse {
                text: Some(v),
                error: None,
            }),
            Err(err) => Ok(msg_ext::JsonToTomlResponse {
                text: None,
                error: Some(err.to_string()),
            }),
        },
        Err(err) => Ok(msg_ext::JsonToTomlResponse {
            text: None,
            error: Some(err.to_string()),
        }),
    }
}

pub(crate) async fn syntax_tree(
    mut context: Context<World>,
    params: Params<msg_ext::SyntaxTreeParams>,
) -> Result<msg_ext::SyntaxTreeResponse, Error> {
    let p = params.required()?;

    let w = context.world().lock().await;

    let doc = w.documents.get(&p.uri).ok_or_else(Error::invalid_params)?;

    Ok(msg_ext::SyntaxTreeResponse {
        text: format!("{:#?}", doc.parse.clone().into_syntax()),
    })
}

async fn load_config_file(mut context: Context<World>) -> Result<(), anyhow::Error> {
    let mut w = context.world().lock().await;
    w.taplo_config = None;

    if !w.configuration.taplo_config_enabled.unwrap_or(false) {
        return Ok(());
    }

    if let Some(config_path) = &w.configuration.taplo_config {
        if !config_path.is_empty() {
            if is_absolute_path(&config_path) {
                let f = read_file(&config_path).await?;

                w.taplo_config = Some(toml::from_slice(&f)?);

                return Ok(());
            } else {
                let workspace_path = w.workspace_path();

                match workspace_path {
                    Some(ws_path) => {
                        let f = read_file(ws_path.join(&config_path).to_str().unwrap()).await?;

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
                if file_exists(ws_path.join(name).to_str().unwrap()) {
                    if let Ok(cfg_file) = read_file(ws_path.join(name).to_str().unwrap()).await {
                        w.taplo_config = Some(toml::from_slice(&cfg_file)?);
                        drop(w);

                        return Ok(());
                    }
                }
            }
            Ok(())
        }
        None => Ok(()),
    }
}
