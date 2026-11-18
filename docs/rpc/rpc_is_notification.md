# rpc.is_notification

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\rpc.rs` (lines 23-25)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_notification(&self) -> bool {
        self.method.is_some() && self.id.is_none()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_notification(&self) -> bool {
        self.method.is_some() && self.id.is_none()
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
