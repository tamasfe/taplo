# rpc.invalid_params

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 269-275)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn invalid_params() -> Error {
        Error {
            code: -32602,
            message: "Invalid params".into(),
            data: None,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn invalid_params() -> Error {
        Error {
            code: -32602,
            message: "Invalid params".into(),
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
