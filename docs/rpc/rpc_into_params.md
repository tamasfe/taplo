# rpc.into_params

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 169-176)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn into_params<P: DeserializeOwned>(self) -> Response<P> {
        Response {
            jsonrpc: self.jsonrpc,
            id: self.id,
            result: self.result.map(|v| serde_json::from_value(v).unwrap()),
            error: self.error,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn into_params<P: DeserializeOwned>(self) -> Response<P> {
        Response {
            jsonrpc: self.jsonrpc,
            id: self.id,
            result: self.result.map(|v| serde_json::from_value(v).unwrap()),
            error: self.error,
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
