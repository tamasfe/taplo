# lib.is_shutting_down

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 179-181)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn is_shutting_down(&self) -> bool {
        self.inner.lock().await.shutting_down
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub async fn is_shutting_down(&self) -> bool {
        self.inner.lock().await.shutting_down
    }
```

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 2
- Features: 1

## Callers

None

## Callees

None
