# environment.spawn_local

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 241-248)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn spawn_local<F>(&self, fut: F)
    where
        F: std::future::Future + 'static,
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
    fn spawn_local<F>(&self, fut: F)
    where
        F: std::future::Future + 'static,
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
