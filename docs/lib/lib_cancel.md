# lib.cancel

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 49-55)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn cancel(&mut self) {
        self.cancelled.store(true, Ordering::SeqCst);

        if let Some(w) = (*self.waker.lock().unwrap()).take() {
            w.wake();
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn cancel(&mut self) {
        self.cancelled.store(true, Ordering::SeqCst);

        if let Some(w) = (*self.waker.lock().unwrap()).take() {
            w.wake();
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
