# rpc.into_message

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 85-94)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn into_message(self) -> Message {
        Message {
            jsonrpc: self.jsonrpc,
            method: Some(self.method),
            id: self.id,
            params: self.params.map(|p| serde_json::to_value(p).unwrap()),
            result: None,
            error: None,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn into_message(self) -> Message {
        Message {
            jsonrpc: self.jsonrpc,
            method: Some(self.method),
            id: self.id,
            params: self.params.map(|p| serde_json::to_value(p).unwrap()),
            result: None,
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
