# completion.completion

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\completion.rs` (lines 25-436)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
... (truncated)
```

## Git History

- Commits: 7
- Authors: 2
- Bug fixes: 3
- Features: 3

## Callers

None

## Callees

None
