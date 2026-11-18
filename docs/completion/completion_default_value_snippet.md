# completion.default_value_snippet

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\completion.rs` (lines 686-752)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
