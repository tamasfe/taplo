# rpc.into_request

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 31-38)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn into_request(self) -> Request<serde_json::Value> {
        Request {
            jsonrpc: self.jsonrpc,
            method: self.method.unwrap(),
            id: self.id,
            params: self.params,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn into_request(self) -> Request<serde_json::Value> {
        Request {
            jsonrpc: self.jsonrpc,
            method: self.method.unwrap(),
            id: self.id,
            params: self.params,
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
