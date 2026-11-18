# lib.Cancellation

- **Type:** struct
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 35-38)
- **Role:** internal
- **Stability:** unknown

## Signature

```
struct Cancellation {
    cancelled: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}
```

## Docstring

_No docstring provided._

## Source

```
struct Cancellation {
    cancelled: Arc<AtomicBool>,
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
