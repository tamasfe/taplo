# lsp.poll_ready

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\lsp.rs` (lines 54-59)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
```

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
