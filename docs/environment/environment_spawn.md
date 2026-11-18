# environment.spawn

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 231-239)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn spawn<F>(&self, fut: F)
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send,
    {
        spawn_local(async move {
            fut.await;
        })
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn spawn<F>(&self, fut: F)
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send,
    {
        spawn_local(async move {
            fut.await;
        })
    }
```

## Git History

- Commits: 6
- Authors: 2
- Bug fixes: 3
- Features: 2

## Callers

None

## Callees

None
