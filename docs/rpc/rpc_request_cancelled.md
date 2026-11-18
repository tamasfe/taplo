# rpc.request_cancelled

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 293-299)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn request_cancelled() -> Error {
        Error {
            code: -32800,
            message: "Request cancelled".into(),
            data: None,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn request_cancelled() -> Error {
        Error {
            code: -32800,
            message: "Request cancelled".into(),
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
