# environment.poll_write

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 87-127)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, std::io::Error>> {
        if self.fut.is_none() {
            let this = JsValue::null();

            let ret: JsValue = match self.f.call1(&this, &Uint8Array::from(buf).into()) {
                Ok(val) => val,
                Err(error) => {
                    return Poll::Ready(Err(io::Error::other(
                        format!("{:?}", error),
                    )));
                }
            };

            self.fut = Some(JsFuture::from(Promise::from(ret)));
        }

        if let Some(fut) = self.fut.as_mut() {
            match fut.poll_unpin(cx) {
                task::Poll::Ready(val) => {
                    let res = match val {
                        Ok(num_written) => {
                            let n = num_written.as_f64().unwrap_or(0.0).floor() as usize;
                            Ok(n)
                        }
                        Err(err) => Err(io::Error::other(format!("{:?}", err))),
                    };

                    self.fut = None;

                    Poll::Ready(res)
                }
                task::Poll::Pending => Poll::Pending,
            }
        } else {
            unreachable!()
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, std::io::Error>> {
        if self.fut.is_none() {
            let this = JsValue::null();

            let ret: JsValue = match self.f.call1(&this, &Uint8Array::from(buf).into()) {
                Ok(val) => val,
                Err(error) => {
                    return Poll::Ready(Err(io::Error::other(
                        format!("{:?}", error),
                    )));
                }
            };

            self.fut = Some(JsFuture::from(Promise::from(ret)));
        }

        if let Some(fut) = self.fut.as_mut() {
            match fut.poll_unpin(cx) {
                task::Poll::Ready(val) => {
                    let res = match val {
                        Ok(num_written) => {
                            let n = num_written.as_f64().unwrap_or(0.0).floor() as usize;
                            Ok(n)
                        }
                        Err(err) => Err(io::Error::other(format!("{:?}", err))),
                    };

                    self.fut = None;

                    Poll::Ready(res)
                }
                task::Poll::Pending => Poll::Pending,
            }
        } else {
            unreachable!()
        }
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
