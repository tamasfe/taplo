# log.write

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\log.rs` (lines 10-28)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            futures::executor::block_on(self.0.write(buf))
        }

        // On WASM we cannot do blocking writes without blocking
        // the event loop, so we simply do not wait.
        #[cfg(target_arch = "wasm32")]
        {
            use futures::FutureExt;

            let _ = self.0.write_all(buf).boxed_local().poll_unpin(
                &mut futures::task::Context::from_waker(&futures::task::noop_waker()),
            );

            Ok(buf.len())
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            futures::executor::block_on(self.0.write(buf))
        }

        // On WASM we cannot do blocking writes without blocking
        // the event loop, so we simply do not wait.
        #[cfg(target_arch = "wasm32")]
        {
            use futures::FutureExt;

            let _ = self.0.write_all(buf).boxed_local().poll_unpin(
                &mut futures::task::Context::from_waker(&futures::task::noop_waker()),
            );

            Ok(buf.len())
        }
    }
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
