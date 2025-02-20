use lsp_async_stub::{
    rpc::Error,
    util::{LspExt, Position},
    Context, Params,
};
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse, CompletionTextEdit,
    Documentation, InsertTextFormat, MarkupContent, Range, TextEdit,
};
use serde_json::Value;
use std::borrow::Cow;
use std::fmt::Write as _;
use taplo::dom::{node::TableKind, Keys, Node};
use taplo_common::{
    environment::Environment,
    schema::{ext::schema_ext_of, ValueExt},
};

use crate::{
    query::{lookup_keys, Query},
    world::World,
};

#[tracing::instrument(skip_all)]
pub async fn completion<E: Environment>(
    context: Context<World<E>>,
    params: Params<CompletionParams>,
) -> Result<Option<CompletionResponse>, Error> {
    let p = params.required()?;

    let document_uri = p.text_document_position.text_document.uri;

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&document_uri);

    // All completions are tied to schemas.
    if !ws.config.schema.enabled {
        return Ok(None);
    }

    let doc = match ws.document(&document_uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    let Some(schema_association) = ws.schemas.associations().association_for(&document_uri) else {
        return Ok(None);
    };

    let position = p.text_document_position.position;
    let Some(offset) = doc.mapper.offset(Position::from_lsp(position)) else {
        tracing::error!(?position, "document position not found");
        return Ok(None);
    };

    let query = Query::at(&doc.dom, offset);

    let value = match serde_json::to_value(&doc.dom) {
        Ok(v) => v,
        Err(error) => {
            tracing::warn!(%error, "unable to serialize DOM");
            Value::Null
        }
    };

    if query.in_table_header() {
        let key_count = query.header_keys().len();

        let object_schemas = match ws
            .schemas
            .possible_schemas_from(
                &schema_association.url,
                &value,
                &Keys::empty(),
                key_count + ws.config.completion.max_keys + 1,
            )
            .await
            .map(|s| {
                s.into_iter().filter(|(_, _, s)| {
                    s["type"].is_null()
                        || s["type"] == "object"
                        || s["type"]
                            .as_array()
                            .is_some_and(|arr| arr.iter().any(|v| v == "object"))
                })
            }) {
            Ok(s) => s,
            Err(error) => {
                tracing::error!(?error, "failed to collect schemas");
                return Ok(None);
            }
        };

        let key_range = query.header_key().map(|k| k.text_range()).and_then(|r| {
            if r.is_empty() {
                None
            } else {
                Some(r)
            }
        });

        let node = query
            .dom_node()
            .cloned()
            .unwrap_or_else(|| (Keys::empty(), doc.dom.clone()));

        return Ok(Some(CompletionResponse::Array(
            object_schemas
                // Filter out existing tables in the dom.
                .filter(|(full_key, _, _)| match doc.dom.path(full_key) {
                    Some(n) => {
                        node.0 == *full_key
                            || n.as_table().is_some_and(|t| t.kind() == TableKind::Pseudo)
                    }
                    None => true,
                })
                .map(|(full_key, _, s)| CompletionItem {
                    label: full_key.to_string(),
                    kind: Some(CompletionItemKind::STRUCT),
                    documentation: documentation(&s),
                    text_edit: key_range.map(|r| {
                        CompletionTextEdit::Edit(TextEdit {
                            range: doc.mapper.range(r).unwrap().into_lsp(),
                            new_text: full_key.to_string(),
                        })
                    }),
                    ..Default::default()
                })
                .collect(),
        )));
    }

    if query.in_table_array_header() {
        let key_count = query.header_keys().len();
        let array_of_objects_schemas = match ws
            .schemas
            .possible_schemas_from(
                &schema_association.url,
                &value,
                &Keys::empty(),
                key_count + ws.config.completion.max_keys + 1,
            )
            .await
            .map(|s| {
                s.into_iter().filter(|(_, _, s)| {
                    s["type"] == "array"
                        && (s["items"]["type"] == "object" || s["items"]["type"].is_null())
                })
            }) {
            Ok(s) => s,
            Err(error) => {
                tracing::error!(?error, "failed to collect schemas");
                return Ok(None);
            }
        };

        let key_range = query.header_key().map(|k| k.text_range()).and_then(|r| {
            if r.is_empty() {
                None
            } else {
                Some(r)
            }
        });

        return Ok(Some(CompletionResponse::Array(
            array_of_objects_schemas
                .map(|(full_key, _, s)| CompletionItem {
                    label: full_key.to_string(),
                    kind: Some(CompletionItemKind::STRUCT),
                    documentation: documentation(&s),
                    text_edit: key_range.map(|r| {
                        CompletionTextEdit::Edit(TextEdit {
                            range: doc.mapper.range(r).unwrap().into_lsp(),
                            new_text: full_key.to_string(),
                        })
                    }),
                    ..Default::default()
                })
                .collect(),
        )));
    }

    if query.empty_line() {
        let parent_table = query.parent_table_or_array_table(&doc.dom);

        let schemas = match ws
            .schemas
            .possible_schemas_from(
                &schema_association.url,
                &value,
                &lookup_keys(doc.dom.clone(), &parent_table.0),
                ws.config.completion.max_keys + 1,
            )
            .await
        {
            Ok(s) => s,
            Err(error) => {
                tracing::error!(?error, "failed to collect schemas");
                return Ok(None);
            }
        };

        return Ok(Some(CompletionResponse::Array(
            schemas
                .into_iter()
                // Filter out existing items.
                .filter(|(full_key, _, _)| match doc.dom.path(full_key) {
                    Some(n) => n.as_table().is_some_and(|t| t.kind() == TableKind::Pseudo),
                    None => true,
                })
                .map(|(_, relative_keys, schema)| CompletionItem {
                    label: relative_keys.to_string(),
                    kind: Some(CompletionItemKind::VARIABLE),
                    documentation: documentation(&schema),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    insert_text: Some(new_entry_snippet(&relative_keys, &schema, false)),
                    ..Default::default()
                })
                .collect(),
        )));
    }

    if query.in_entry_keys() {
        let mut parent_keys = if let Some((k, _)) = query.dom_node() {
            k.clone()
        } else {
            query.parent_table_or_array_table(&doc.dom).0
        };

        let entry_keys = query.entry_keys();

        parent_keys = parent_keys.skip_right(entry_keys.len());

        let schemas = match ws
            .schemas
            .possible_schemas_from(
                &schema_association.url,
                &value,
                &lookup_keys(doc.dom.clone(), &parent_keys),
                entry_keys.len() + ws.config.completion.max_keys + 1,
            )
            .await
        {
            Ok(s) => s,
            Err(error) => {
                tracing::error!(?error, "failed to collect schemas");
                return Ok(None);
            }
        };

        let key_range = query.entry_key().map(|k| k.text_range());

        let has_eq = query.entry_has_eq();

        return Ok(Some(CompletionResponse::Array(
            schemas
                .into_iter()
                .map(|(_, relative_keys, schema)| CompletionItem {
                    label: relative_keys.to_string(),
                    kind: Some(CompletionItemKind::VARIABLE),
                    documentation: documentation(&schema),
                    text_edit: key_range.map(|r| {
                        CompletionTextEdit::Edit(TextEdit {
                            range: doc.mapper.range(r).unwrap().into_lsp(),
                            new_text: if has_eq {
                                relative_keys.to_string() + " "
                            } else {
                                new_entry_snippet(&relative_keys, &schema, false)
                            },
                        })
                    }),
                    insert_text: Some(if has_eq {
                        relative_keys.to_string() + " "
                    } else {
                        new_entry_snippet(&relative_keys, &schema, false)
                    }),
                    insert_text_format: if has_eq {
                        None
                    } else {
                        Some(InsertTextFormat::SNIPPET)
                    },
                    ..Default::default()
                })
                .collect(),
        )));
    }

    if query.in_entry_value() {
        let (path, _) = query.dom_node().unwrap();

        // Pretty much same as the entry on an empty line
        if query.in_inline_table() {
            let schemas = match ws
                .schemas
                .possible_schemas_from(
                    &schema_association.url,
                    &value,
                    &lookup_keys(doc.dom.clone(), path),
                    ws.config.completion.max_keys + 1,
                )
                .await
            {
                Ok(s) => s,
                Err(error) => {
                    tracing::error!(?error, "failed to collect schemas");
                    return Ok(None);
                }
            };

            return Ok(Some(CompletionResponse::Array(
                schemas
                    .into_iter()
                    // Filter out existing items.
                    .filter(|(full_key, _, _)| match doc.dom.path(full_key) {
                        Some(n) => n.as_table().is_some_and(|t| t.kind() == TableKind::Pseudo),
                        None => true,
                    })
                    .map(|(_, relative_keys, schema)| CompletionItem {
                        label: relative_keys.to_string(),
                        kind: Some(CompletionItemKind::VARIABLE),
                        documentation: documentation(&schema),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        insert_text: Some(new_entry_snippet(&relative_keys, &schema, false)),
                        ..Default::default()
                    })
                    .collect(),
            )));
        }

        let path = if query.is_inline() {
            lookup_keys(doc.dom.clone(), &path.clone())
        } else {
            let parent = query.parent_table_or_array_table(&doc.dom);
            let entry_key = query.entry_keys();
            lookup_keys(doc.dom.clone(), &parent.0.extend(entry_key))
        };

        let schemas = match ws
            .schemas
            .possible_schemas_from(
                &schema_association.url,
                &value,
                &path,
                ws.config.completion.max_keys + 1,
            )
            .await
        {
            Ok(s) => s,
            Err(error) => {
                tracing::error!(?error, "failed to collect schemas");
                return Ok(None);
            }
        };

        let range = if query.in_array() {
            None
        } else {
            query
                .entry_value()
                .map(|k| k.text_range())
                .and_then(|r| doc.mapper.range(r))
                .map(lsp_async_stub::util::LspExt::into_lsp)
        };

        let mut completions = Vec::new();

        for (_, _, schema) in schemas {
            add_value_completions(
                &schema,
                range,
                &mut completions,
                query.is_single_quote_value(),
            );
        }

        return Ok(Some(CompletionResponse::Array(completions)));
    }

    // Only standalone keys left.
    // Almost the same as an empty line except we need to replace the incomplete keys.
    let mut parent_keys = if let Some((k, _)) = query.dom_node() {
        k.clone()
    } else {
        query.parent_table_or_array_table(&doc.dom).0
    };

    let entry_keys = query.entry_keys();

    parent_keys = parent_keys.skip_right(entry_keys.len());

    let schemas = match ws
        .schemas
        .possible_schemas_from(
            &schema_association.url,
            &value,
            &lookup_keys(doc.dom.clone(), &parent_keys),
            ws.config.completion.max_keys + 1,
        )
        .await
    {
        Ok(s) => s,
        Err(error) => {
            tracing::error!(?error, "failed to collect schemas");
            return Ok(None);
        }
    };

    Ok(Some(CompletionResponse::Array(
        schemas
            .into_iter()
            // Filter out existing items.
            .filter(|(full_key, _, _)| match doc.dom.path(full_key) {
                Some(n) => n.as_table().is_some_and(|t| t.kind() == TableKind::Pseudo),
                None => true,
            })
            .map(|(_, relative_keys, schema)| CompletionItem {
                label: relative_keys.to_string(),
                kind: Some(CompletionItemKind::VARIABLE),
                documentation: documentation(&schema),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                    range: doc
                        .mapper
                        .range(entry_keys.all_text_range())
                        .unwrap()
                        .into_lsp(),
                    new_text: new_entry_snippet(&relative_keys, &schema, false),
                })),
                ..Default::default()
            })
            .collect(),
    )))
}

fn documentation(schema: &Value) -> Option<Documentation> {
    if let Some(ext) = schema_ext_of(schema) {
        if let Some(docs) = ext.docs {
            if let Some(docs) = docs.main {
                return Some(Documentation::MarkupContent(MarkupContent {
                    kind: lsp_types::MarkupKind::Markdown,
                    value: docs,
                }));
            }
        }
    }

    if let Some(docs) = schema["description"].as_str() {
        return Some(Documentation::MarkupContent(MarkupContent {
            kind: lsp_types::MarkupKind::Markdown,
            value: docs.into(),
        }));
    }

    None
}

fn add_value_completions(
    schema: &Value,
    range: Option<Range>,
    completions: &mut Vec<CompletionItem>,
    single_quote: bool,
) {
    let ext = schema_ext_of(schema).unwrap_or_default();
    let ext_docs = ext.docs.unwrap_or_default();
    let enum_docs = ext_docs.enum_values.unwrap_or_default();

    let schema_docs = ext_docs
        .main
        .or_else(|| schema["description"].as_str().map(Into::into));

    if let Some(enum_values) = schema["enum"].as_array() {
        for (idx, val) in enum_values.iter().enumerate() {
            let node: Node = match serde_json::from_value(val.clone()) {
                Ok(v) => v,
                Err(err) => {
                    tracing::error!(error = %err, "failed to parse JSON");
                    continue;
                }
            };

            let toml_value = node.to_toml(true, single_quote);

            completions.push(CompletionItem {
                label: toml_value.clone(),
                sort_text: Some(format!("{idx}{toml_value}")),
                kind: Some(match node {
                    Node::Table(_) => CompletionItemKind::STRUCT,
                    _ => CompletionItemKind::VALUE,
                }),
                documentation: enum_docs
                    .get(idx)
                    .cloned()
                    .flatten()
                    .or_else(|| schema_docs.clone())
                    .map(|value| {
                        Documentation::MarkupContent(MarkupContent {
                            kind: lsp_types::MarkupKind::Markdown,
                            value,
                        })
                    }),
                text_edit: range.map(|range| {
                    CompletionTextEdit::Edit(TextEdit {
                        range,
                        new_text: toml_value,
                    })
                }),
                ..Default::default()
            });
        }
        return;
    }

    if let Some(const_value) = schema.get("const") {
        if !const_value.is_null() {
            let node: Node = serde_json::from_value(const_value.clone()).unwrap();
            let toml_value = node.to_toml(true, single_quote);
            completions.push(CompletionItem {
                label: toml_value.clone(),
                kind: Some(match node {
                    Node::Table(_) => CompletionItemKind::STRUCT,
                    _ => CompletionItemKind::VALUE,
                }),
                documentation: ext_docs
                    .const_value
                    .or_else(|| schema_docs.clone())
                    .map(|value| {
                        Documentation::MarkupContent(MarkupContent {
                            kind: lsp_types::MarkupKind::Markdown,
                            value,
                        })
                    }),
                text_edit: range.map(|range| {
                    CompletionTextEdit::Edit(TextEdit {
                        range,
                        new_text: toml_value,
                    })
                }),
                ..Default::default()
            });
        }

        return;
    }

    if let Some(default_value) = schema.get("default") {
        if !default_value.is_null() {
            let node: Node = serde_json::from_value(default_value.clone()).unwrap();
            let toml_value = node.to_toml(true, single_quote);
            completions.push(CompletionItem {
                label: toml_value.clone(),
                kind: Some(match node {
                    Node::Table(_) => CompletionItemKind::STRUCT,
                    _ => CompletionItemKind::VALUE,
                }),
                documentation: ext_docs.default_value.or_else(|| schema_docs.clone()).map(
                    |value| {
                        Documentation::MarkupContent(MarkupContent {
                            kind: lsp_types::MarkupKind::Markdown,
                            value,
                        })
                    },
                ),
                text_edit: range.map(|range| {
                    CompletionTextEdit::Edit(TextEdit {
                        range,
                        new_text: toml_value,
                    })
                }),
                ..Default::default()
            });
        }
    }

    let types = match schema["type"].clone() {
        Value::Null => Vec::from([Value::String("object".into())]),
        Value::String(s) => Vec::from([Value::String(s)]),
        Value::Array(tys) => tys,
        _ => Vec::new(),
    };

    for ty in types {
        if let Some(s) = ty.as_str() {
            match s {
                "string" => {
                    completions.push(CompletionItem {
                        label: r#""""#.into(),
                        kind: Some(CompletionItemKind::VALUE),
                        documentation: Some(Documentation::MarkupContent(MarkupContent {
                            kind: lsp_types::MarkupKind::Markdown,
                            value: schema_docs.clone().unwrap_or_else(|| "string".into()),
                        })),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        text_edit: range.map(|range| {
                            CompletionTextEdit::Edit(TextEdit {
                                range,
                                new_text: r#""$0""#.into(),
                            })
                        }),
                        ..Default::default()
                    });
                }
                "boolean" => {
                    completions.push(CompletionItem {
                        label: r"true".into(),
                        kind: Some(CompletionItemKind::VALUE),
                        documentation: Some(Documentation::MarkupContent(MarkupContent {
                            kind: lsp_types::MarkupKind::Markdown,
                            value: schema_docs.clone().unwrap_or_else(|| "true value".into()),
                        })),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        text_edit: range.map(|range| {
                            CompletionTextEdit::Edit(TextEdit {
                                range,
                                new_text: r"true$0".into(),
                            })
                        }),
                        ..Default::default()
                    });
                    completions.push(CompletionItem {
                        label: r"false".into(),
                        kind: Some(CompletionItemKind::VALUE),
                        documentation: Some(Documentation::MarkupContent(MarkupContent {
                            kind: lsp_types::MarkupKind::Markdown,
                            value: schema_docs.clone().unwrap_or_else(|| "false value".into()),
                        })),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        text_edit: range.map(|range| {
                            CompletionTextEdit::Edit(TextEdit {
                                range,
                                new_text: r"false$0".into(),
                            })
                        }),
                        ..Default::default()
                    });
                }
                "array" => {
                    completions.push(CompletionItem {
                        label: r"[]".into(),
                        kind: Some(CompletionItemKind::VALUE),
                        documentation: Some(Documentation::MarkupContent(MarkupContent {
                            kind: lsp_types::MarkupKind::Markdown,
                            value: schema_docs.clone().unwrap_or_else(|| "array".into()),
                        })),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        text_edit: range.map(|range| {
                            CompletionTextEdit::Edit(TextEdit {
                                range,
                                new_text: r"[$0]".into(),
                            })
                        }),
                        ..Default::default()
                    });
                }
                "object" => {
                    completions.push(CompletionItem {
                        label: r"{ }".into(),
                        kind: Some(CompletionItemKind::VALUE),
                        documentation: Some(Documentation::MarkupContent(MarkupContent {
                            kind: lsp_types::MarkupKind::Markdown,
                            value: schema_docs.clone().unwrap_or_else(|| "object".into()),
                        })),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        text_edit: range.map(|range| {
                            CompletionTextEdit::Edit(TextEdit {
                                range,
                                new_text: r"{ $0 }".into(),
                            })
                        }),
                        ..Default::default()
                    });
                }
                _ => {}
            }
        }
    }
}

fn new_entry_snippet(keys: &Keys, schema: &Value, single_quote: bool) -> String {
    let value = default_value_snippet(schema, 0, single_quote);
    format!("{keys} = {value}")
}

fn default_value_snippet(
    schema: &Value,
    cursor_count: usize,
    single_quote: bool,
) -> Cow<'static, str> {
    if let Some(const_value) = schema.get("const") {
        if !const_value.is_null() {
            let node: Node = serde_json::from_value(const_value.clone()).unwrap();
            return format!("${{{}:{}}}", cursor_count, node.to_toml(true, single_quote)).into();
        }
    }

    if let Some(default_value) = schema.get("default") {
        if !default_value.is_null() {
            let node: Node = serde_json::from_value(default_value.clone()).unwrap();
            return format!("${{{}:{}}}", cursor_count, node.to_toml(true, single_quote)).into();
        }
    }

    if schema.get("enum").is_some() {
        return format!("${cursor_count}").into();
    }

    let mut init_keys = Vec::new();

    if let Some(ext) = schema_ext_of(schema) {
        if let Some(extra_init_keys) = ext.init_keys {
            init_keys.extend(extra_init_keys);
        }
    }

    if let Some(arr) = schema["required"].as_array() {
        init_keys.extend(
            arr.iter()
                .filter_map(|s| s.as_str().map(ToString::to_string)),
        );
    }

    init_keys.dedup();

    if !init_keys.is_empty() {
        let mut s = String::new();
        s += "{ ";

        for (i, init_key) in init_keys.iter().enumerate() {
            if i != 0 {
                s += ", ";
            }
            write!(
                s,
                "{init_key} = {}",
                default_value_snippet(
                    &schema["properties"][init_key],
                    cursor_count + 1,
                    single_quote
                )
            )
            .unwrap();
        }

        s += " }$0";

        return s.into();
    }

    empty_value_snippet(schema, cursor_count).into()
}

fn empty_value_snippet(schema: &Value, cursor_count: usize) -> String {
    if schema.is_schema_ref() {
        return format!("${cursor_count}");
    }

    match &schema["type"] {
        Value::Null => format!("{{ ${cursor_count} }}"),
        Value::String(s) => match s.as_str() {
            "object" => format!("{{ ${cursor_count} }}"),
            "array" => format!("[${cursor_count}]"),
            "string" => format!(r#""${cursor_count}""#),
            "boolean" => format!("${{{cursor_count}:false}}"),
            _ => format!("${cursor_count}"),
        },
        _ => format!("${cursor_count}"),
    }
}
