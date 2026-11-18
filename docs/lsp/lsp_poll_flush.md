# lsp.poll_flush

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\lsp.rs` (lines 72-77)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn poll_flush(
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
    fn poll_flush(
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
