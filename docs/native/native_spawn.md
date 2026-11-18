# native.spawn

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 39-45)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn spawn<F>(&self, fut: F)
    where
        F: futures::Future + Send + 'static,
        F::Output: Send,
    {
        self.handle.spawn(fut);
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn spawn<F>(&self, fut: F)
    where
        F: futures::Future + Send + 'static,
        F::Output: Send,
    {
        self.handle.spawn(fut);
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
