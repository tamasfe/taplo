# rpc.success

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 139-146)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn success(data: R) -> Self {
        Response {
            jsonrpc: "2.0".into(),
            id: lsp_types::NumberOrString::Number(0),
            result: Some(data),
            error: None,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn success(data: R) -> Self {
        Response {
            jsonrpc: "2.0".into(),
            id: lsp_types::NumberOrString::Number(0),
            result: Some(data),
            error: None,
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
