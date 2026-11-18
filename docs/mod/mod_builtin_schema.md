# mod.builtin_schema

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 38-44)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn builtin_schema(url: &Url) -> Option<Arc<Value>> {
        if url.as_str() == TAPLO_CONFIG_URL {
            Some(taplo_config_schema())
        } else {
            None
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn builtin_schema(url: &Url) -> Option<Arc<Value>> {
        if url.as_str() == TAPLO_CONFIG_URL {
            Some(taplo_config_schema())
        } else {
            None
        }
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
