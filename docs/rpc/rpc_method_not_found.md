# rpc.method_not_found

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 261-267)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn method_not_found() -> Error {
        Error {
            code: -32601,
            message: "Method not found".into(),
            data: None,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn method_not_found() -> Error {
        Error {
            code: -32601,
            message: "Method not found".into(),
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
