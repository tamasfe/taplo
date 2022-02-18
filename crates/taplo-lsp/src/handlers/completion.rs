use std::borrow::Cow;

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

    // All completions are based on schemas.
    if !ws.config.schema.enabled {
        return Ok(None);
    }

    let doc = ws.document(&document_uri)?;

    let schema_association = match ws
        .schemas
        .associations()
        .association_for(document_uri.as_str())
    {
        Some(ass) => ass,
        None => return Ok(None),
    };

    let position = p.text_document_position.position;
    let offset = match doc.mapper.offset(Position::from_lsp(position)) {
        Some(ofs) => ofs,
        None => {
            tracing::error!(?position, "document position not found");
            return Ok(None);
        }
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
        let object_schemas = match ws
            .schemas
            .child_schemas_from(
                &schema_association.url,
                &value,
                &Keys::empty(),
                ws.config.completion.max_keys + 1,
            )
            .await
            .map(|s| {
                s.into_iter().filter(|(_, _, s)| {
                    s["type"].is_null()
                        || s["type"] == "object"
                        || s["type"]
                            .as_array()
                            .map(|arr| arr.iter().any(|v| v == "object"))
                            .unwrap_or(false)
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

        tracing::info!("aaaaaaa");

        return Ok(Some(CompletionResponse::Array(
            object_schemas
                // Filter out existing tables in the dom.
                .filter(|(full_key, _, _)| match doc.dom.path(full_key) {
                    Some(n) => {
                        node.0 == *full_key
                            || n.as_table()
                                .map(|t| t.kind() == TableKind::Pseudo)
                                .unwrap_or(false)
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
        let array_of_objects_schemas = match ws
            .schemas
            .child_schemas_from(
                &schema_association.url,
                &value,
                &Keys::empty(),
                ws.config.completion.max_keys + 1,
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
            .child_schemas_from(
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
                    Some(n) => n
                        .as_table()
                        .map(|t| t.kind() == TableKind::Pseudo)
                        .unwrap_or(false),
                    None => true,
                })
                .map(|(_, relative_keys, schema)| CompletionItem {
                    label: relative_keys.to_string(),
                    kind: Some(CompletionItemKind::VARIABLE),
                    documentation: documentation(&schema),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    insert_text: Some(new_entry_snippet(&relative_keys, &schema)),
                    ..Default::default()
                })
                .collect(),
        )));
    }

    if query.in_entry_keys() {
        let parent = query.parent_table_or_array_table(&doc.dom);

        let schemas = match ws
            .schemas
            .child_schemas_from(
                &schema_association.url,
                &value,
                &lookup_keys(doc.dom.clone(), &parent.0),
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

        let key_range = query.entry_key().map(|k| k.text_range());

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
                            new_text: relative_keys.to_string() + " ",
                        })
                    }),
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
                .child_schemas_from(
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
                        Some(n) => n
                            .as_table()
                            .map(|t| t.kind() == TableKind::Pseudo)
                            .unwrap_or(false),
                        None => true,
                    })
                    .map(|(_, relative_keys, schema)| CompletionItem {
                        label: relative_keys.to_string(),
                        kind: Some(CompletionItemKind::VARIABLE),
                        documentation: documentation(&schema),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        insert_text: Some(new_entry_snippet(&relative_keys, &schema)),
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
            parent.0.extend(entry_key)
        };

        let schemas = match ws
            .schemas
            .child_schemas_from(
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
                .map(|r| r.into_lsp())
        };

        let mut completions = Vec::new();

        for (_, _, schema) in schemas.into_iter() {
            add_value_completions(&schema, range, &mut completions);
        }

        return Ok(Some(CompletionResponse::Array(completions)));
    }

    // Only standalone keys left.
    // Almost the same as an empty line except we need to replace the keys.
    let parent_table = query.parent_table_or_array_table(&doc.dom);

    let schemas = match ws
        .schemas
        .child_schemas_from(
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

    let entry_keys = query.entry_keys();

    Ok(Some(CompletionResponse::Array(
        schemas
            .into_iter()
            // Filter out existing items.
            .filter(|(full_key, _, _)| match doc.dom.path(full_key) {
                Some(n) => n
                    .as_table()
                    .map(|t| t.kind() == TableKind::Pseudo)
                    .unwrap_or(false),
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
                    new_text: new_entry_snippet(&relative_keys, &schema),
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

            let toml_value = node.to_toml(true);

            completions.push(CompletionItem {
                label: toml_value.clone(),
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
        let node: Node = serde_json::from_value(const_value.clone()).unwrap();
        let toml_value = node.to_toml(true);
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

    if let Some(default_value) = schema.get("default") {
        let node: Node = serde_json::from_value(default_value.clone()).unwrap();
        let toml_value = node.to_toml(true);
        completions.push(CompletionItem {
            label: toml_value.clone(),
            kind: Some(match node {
                Node::Table(_) => CompletionItemKind::STRUCT,
                _ => CompletionItemKind::VALUE,
            }),
            documentation: ext_docs
                .default_value
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
}

fn new_entry_snippet(keys: &Keys, schema: &Value) -> String {
    let value = entry_default_value_snippet(schema);
    format!("{keys} = {value}")
}

fn entry_default_value_snippet(schema: &Value) -> Cow<'static, str> {
    if let Some(const_value) = schema.get("const") {
        let node: Node = serde_json::from_value(const_value.clone()).unwrap();
        return node.to_toml(true).into();
    }

    if let Some(default) = schema.get("default") {
        let node: Node = serde_json::from_value(default.clone()).unwrap();
        return format!("${{0:{}}}", node.to_toml(true)).into();
    }

    if schema.get("enum").is_some() {
        return Cow::Borrowed("$0");
    }

    default_value_snippet(schema)
}

fn default_value_snippet(schema: &Value) -> Cow<'static, str> {
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
            s += &format!(
                "{init_key} = {}",
                empty_value_snippet(&schema["properties"][init_key], i + 1)
            );
        }

        s += " }$0";

        return s.into();
    }

    empty_value_snippet(schema, 0).into()
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
            _ => format!("${cursor_count}"),
        },
        Value::Array(_) => format!("${cursor_count}"),
        _ => format!("${cursor_count}"),
    }
}
