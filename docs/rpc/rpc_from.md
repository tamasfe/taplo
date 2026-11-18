# rpc.from

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 195-210)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn from(res: Result<R, E>) -> Self {
        match res {
            Ok(r) => Response {
                jsonrpc: "2.0".into(),
                id: lsp_types::NumberOrString::Number(0),
                result: Some(r),
                error: None,
            },
            Err(err) => Response {
                jsonrpc: "2.0".into(),
                id: lsp_types::NumberOrString::Number(0),
                result: None,
                error: Some(err.into()),
            },
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn from(res: Result<R, E>) -> Self {
        match res {
            Ok(r) => Response {
                jsonrpc: "2.0".into(),
                id: lsp_types::NumberOrString::Number(0),
                result: Some(r),
                error: None,
            },
            Err(err) => Response {
                jsonrpc: "2.0".into(),
                id: lsp_types::NumberOrString::Number(0),
                result: None,
                error: Some(err.into()),
            },
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
