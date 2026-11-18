# rpc.into_response

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 40-47)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn into_response(self) -> Response<serde_json::Value> {
        Response {
            jsonrpc: self.jsonrpc,
            id: self.id.unwrap(),
            error: self.error,
            result: self.result,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn into_response(self) -> Response<serde_json::Value> {
        Response {
            jsonrpc: self.jsonrpc,
            id: self.id.unwrap(),
            error: self.error,
            result: self.result,
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
