# ext.schema_ext_of

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\ext.rs` (lines 35-47)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn schema_ext_of(schema: &Value) -> Option<TaploSchemaExt> {
    schema.get(EXTENSION_KEY).and_then(|val| {
        if val.is_object() {
            serde_json::from_value(val.clone())
                .tap_err(
                    |error| tracing::warn!(key = EXTENSION_KEY, %error, "invalid schema extension"),
                )
                .ok()
        } else {
            None
        }
    })
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn schema_ext_of(schema: &Value) -> Option<TaploSchemaExt> {
    schema.get(EXTENSION_KEY).and_then(|val| {
        if val.is_object() {
            serde_json::from_value(val.clone())
                .tap_err(
                    |error| tracing::warn!(key = EXTENSION_KEY, %error, "invalid schema extension"),
                )
                .ok()
        } else {
            None
        }
    })
}
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
