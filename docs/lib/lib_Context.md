# lib.Context

- **Type:** struct
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 157-164)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct Context<W: Clone> {
    inner: Arc<AsyncMutex<Inner<W>>>,
    cancel_token: CancelToken,
    last_req_id: Option<rpc::RequestId>, // For cancellation
    rw: Arc<AsyncMutex<Box<dyn MessageWriter>>>,
    world: W,
    deferred: DeferredTasks,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct Context<W: Clone> {
    inner: Arc<AsyncMutex<Inner<W>>>,
    cancel_token: CancelToken,
    last_req_id: Option<rpc::RequestId>, // For cancellation
    rw: Arc<AsyncMutex<Box<dyn MessageWriter>>>,
    world: W,
    deferred: DeferredTasks,
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
