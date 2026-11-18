# rpc.with_request_id

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 133-135)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn with_request_id(self, id: RequestId) -> Self {
        Response { id, ..self }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn with_request_id(self, id: RequestId) -> Self {
        Response { id, ..self }
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
