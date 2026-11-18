# native.spawn_local

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 47-52)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn spawn_local<F>(&self, fut: F)
    where
        F: futures::Future + 'static,
    {
        tokio::task::spawn_local(fut);
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn spawn_local<F>(&self, fut: F)
    where
        F: futures::Future + 'static,
    {
        tokio::task::spawn_local(fut);
    }
```

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 1
- Features: 2

## Callers

None

## Callees

None
