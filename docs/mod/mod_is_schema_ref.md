# mod.is_schema_ref

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 642-644)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn is_schema_ref(&self) -> bool {
        self["$ref"].is_string()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn is_schema_ref(&self) -> bool {
        self["$ref"].is_string()
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
