# lib.on_request

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 597-608)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn on_request<R, F>(mut self, handler: fn(Context<W>, Params<R::Params>) -> F) -> Self
    where
        R: Request + 'static,
        F: Future<Output = Result<R::Result, rpc::Error>> + 'static,
    {
        self.inner.handlers.insert(
            R::METHOD.into(),
            Box::new(handler::RequestHandler::<R, _, _>::new(handler)),
        );
        tracing::info!(method = R::METHOD, "registered request handler");
        self
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn on_request<R, F>(mut self, handler: fn(Context<W>, Params<R::Params>) -> F) -> Self
    where
        R: Request + 'static,
        F: Future<Output = Result<R::Result, rpc::Error>> + 'static,
    {
        self.inner.handlers.insert(
            R::METHOD.into(),
            Box::new(handler::RequestHandler::<R, _, _>::new(handler)),
        );
        tracing::info!(method = R::METHOD, "registered request handler");
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
