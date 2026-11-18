# environment.poll_shutdown

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 136-141)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), std::io::Error>> {
        Poll::Ready(Ok(()))
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), std::io::Error>> {
        Poll::Ready(Ok(()))
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
