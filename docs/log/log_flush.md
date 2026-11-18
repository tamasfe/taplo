# log.flush

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\log.rs` (lines 30-52)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn flush(&mut self) -> io::Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            futures::executor::block_on(self.0.flush())
        }

        // On WASM we cannot do blocking writes without blocking
        // the event loop, so we simply do not wait.
        #[cfg(target_arch = "wasm32")]
        {
            use futures::FutureExt;

            let _ =
                self.0
                    .flush()
                    .boxed_local()
                    .poll_unpin(&mut futures::task::Context::from_waker(
                        &futures::task::noop_waker(),
                    ));

            Ok(())
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn flush(&mut self) -> io::Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            futures::executor::block_on(self.0.flush())
        }

        // On WASM we cannot do blocking writes without blocking
        // the event loop, so we simply do not wait.
        #[cfg(target_arch = "wasm32")]
        {
            use futures::FutureExt;

            let _ =
                self.0
                    .flush()
                    .boxed_local()
                    .poll_unpin(&mut futures::task::Context::from_waker(
                        &futures::task::noop_waker(),
                    ));

            Ok(())
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
