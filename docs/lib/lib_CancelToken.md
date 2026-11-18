# lib.CancelToken

- **Type:** struct
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 59-63)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct CancelToken {
    cancelled: Arc<AtomicBool>,
    waker_set: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct CancelToken {
    cancelled: Arc<AtomicBool>,
    waker_set: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
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
