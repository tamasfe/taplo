# rpc.invalid_request

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 253-259)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn invalid_request() -> Error {
        Error {
            code: -32600,
            message: "Invalid request".into(),
            data: None,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn invalid_request() -> Error {
        Error {
            code: -32600,
            message: "Invalid request".into(),
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
