# mod.taplo_config_schema

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 33-35)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn taplo_config_schema() -> Arc<Value> {
        Arc::new(serde_json::to_value(schemars::schema_for!(crate::config::Config)).unwrap())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn taplo_config_schema() -> Arc<Value> {
        Arc::new(serde_json::to_value(schemars::schema_for!(crate::config::Config)).unwrap())
    }
```

## Git History

- Commits: 13
- Authors: 5
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
