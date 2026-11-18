# lib.is_initialized

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 175-177)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn is_initialized(&self) -> bool {
        self.inner.lock().await.initialized
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub async fn is_initialized(&self) -> bool {
        self.inner.lock().await.initialized
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
