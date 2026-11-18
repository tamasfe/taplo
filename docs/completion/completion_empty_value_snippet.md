# completion.empty_value_snippet

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\completion.rs` (lines 754-770)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
