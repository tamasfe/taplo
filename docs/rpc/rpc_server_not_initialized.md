# rpc.server_not_initialized

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 285-291)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn server_not_initialized() -> Error {
        Error {
            code: -32002,
            message: "Server not initialized".into(),
            data: None,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn server_not_initialized() -> Error {
        Error {
            code: -32002,
            message: "Server not initialized".into(),
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
