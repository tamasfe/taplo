# lib.on_notification

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 584-595)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn on_notification<N, F>(mut self, handler: fn(Context<W>, Params<N::Params>) -> F) -> Self
    where
        N: Notification + 'static,
        F: Future<Output = ()> + 'static,
    {
        self.inner.handlers.insert(
            N::METHOD.into(),
            Box::new(handler::NotificationHandler::<N, _, _>::new(handler)),
        );
        tracing::info!(method = N::METHOD, "registered notification handler");
        self
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn on_notification<N, F>(mut self, handler: fn(Context<W>, Params<N::Params>) -> F) -> Self
    where
        N: Notification + 'static,
        F: Future<Output = ()> + 'static,
    {
        self.inner.handlers.insert(
            N::METHOD.into(),
            Box::new(handler::NotificationHandler::<N, _, _>::new(handler)),
        );
        tracing::info!(method = N::METHOD, "registered notification handler");
        self
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
