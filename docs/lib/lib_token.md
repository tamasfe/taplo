# lib.token

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 41-47)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn token(&self) -> CancelToken {
        CancelToken {
            cancelled: self.cancelled.clone(),
            waker_set: Arc::new(AtomicBool::new(false)),
            waker: self.waker.clone(),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn token(&self) -> CancelToken {
        CancelToken {
            cancelled: self.cancelled.clone(),
            waker_set: Arc::new(AtomicBool::new(false)),
            waker: self.waker.clone(),
        }
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
