# rpc.server

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 309-319)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn server(code: i32) -> Error {
        if code < -32000 || code > -32099 {
            panic!("code must be between -32000 and -32099")
        }

        Error {
            code: -32603,
            message: "Server error".into(),
            data: None,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn server(code: i32) -> Error {
        if code < -32000 || code > -32099 {
            panic!("code must be between -32000 and -32099")
        }

        Error {
            code: -32603,
            message: "Server error".into(),
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
