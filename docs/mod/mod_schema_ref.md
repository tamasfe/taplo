# mod.schema_ref

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 646-648)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn schema_ref(&self) -> Option<&str> {
        self["$ref"].as_str()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn schema_ref(&self) -> Option<&str> {
        self["$ref"].as_str()
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
