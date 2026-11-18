# completion.add_value_completions

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\completion.rs` (lines 460-679)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
