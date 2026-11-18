# lib.create_context

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 555-576)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn create_context<D>(
        inner: Arc<AsyncMutex<Inner<W>>>,
        rw: Arc<AsyncMutex<Box<dyn MessageWriter>>>,
        world: W,
        req: &rpc::Request<D>,
    ) -> Context<W> {
        let cancel = Cancellation::default();
        let cancel_token = cancel.token();

        if let Some(id) = &req.id {
            inner.lock().await.tasks.insert(id.clone(), cancel);
        }

        Context {
            cancel_token,
            world,
            inner,
            last_req_id: None,
            rw,
            deferred: Default::default(),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn create_context<D>(
        inner: Arc<AsyncMutex<Inner<W>>>,
        rw: Arc<AsyncMutex<Box<dyn MessageWriter>>>,
        world: W,
        req: &rpc::Request<D>,
    ) -> Context<W> {
        let cancel = Cancellation::default();
        let cancel_token = cancel.token();

        if let Some(id) = &req.id {
            inner.lock().await.tasks.insert(id.clone(), cancel);
        }

        Context {
            cancel_token,
            world,
            inner,
            last_req_id: None,
            rw,
            deferred: Default::default(),
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
