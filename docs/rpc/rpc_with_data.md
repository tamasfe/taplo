# rpc.with_data

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 240-243)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn with_data(mut self, data: impl Serialize) -> Self {
        self.data = Some(serde_json::to_value(data).unwrap());
        self
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn with_data(mut self, data: impl Serialize) -> Self {
        self.data = Some(serde_json::to_value(data).unwrap());
        self
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
