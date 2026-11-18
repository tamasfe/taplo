# completion.documentation

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\completion.rs` (lines 438-458)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
