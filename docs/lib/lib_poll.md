# lib.poll

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\lib.rs` (lines 105-113)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        match self.0.poll_unpin(cx) {
            Poll::Ready(_) => Poll::Ready(Err(rpc::Error::request_cancelled())),
            Poll::Pending => Poll::Pending,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        match self.0.poll_unpin(cx) {
            Poll::Ready(_) => Poll::Ready(Err(rpc::Error::request_cancelled())),
            Poll::Pending => Poll::Pending,
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
