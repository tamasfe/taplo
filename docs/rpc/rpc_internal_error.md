# rpc.internal_error

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 277-283)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn internal_error() -> Error {
        Error {
            code: -32603,
            message: "Internal error".into(),
            data: None,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn internal_error() -> Error {
        Error {
            code: -32603,
            message: "Internal error".into(),
            data: None,
        }
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
