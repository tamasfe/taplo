# rpc.with_id

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 77-79)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn with_id(self, id: Option<RequestId>) -> Self {
        Self { id, ..self }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn with_id(self, id: Option<RequestId>) -> Self {
        Self { id, ..self }
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
