# environment.poll_read

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 29-72)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> task::Poll<std::io::Result<()>> {
        if self.fut.is_none() {
            let this = JsValue::null();
            let ret: JsValue = match self.f.call1(&this, &JsValue::from(buf.remaining())) {
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
                        Ok(chunk) => {
                            let arr = js_sys::Uint8Array::from(chunk).to_vec();
                            if !arr.is_empty() {
                                buf.put_slice(&arr);
                            }

                            Ok(())
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
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> task::Poll<std::io::Result<()>> {
        if self.fut.is_none() {
            let this = JsValue::null();
            let ret: JsValue = match self.f.call1(&this, &JsValue::from(buf.remaining())) {
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
                        Ok(chunk) => {
                            let arr = js_sys::Uint8Array::from(chunk).to_vec();
                            if !arr.is_empty() {
                                buf.put_slice(&arr);
                            }

                            Ok(())
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
