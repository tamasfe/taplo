# Module: environment

**File:** `\crates\taplo-wasm\src\environment.rs`

## Overview

Module `environment` contains 5 analyzed entities.

```
use anyhow::anyhow;
use futures::FutureExt;
use js_sys::{Function, Promise, Uint8Array};
use std::{
    io,
    path::Path,
    pin::Pin,
    task::{self, Poll},
};
use taplo_common::environment::Environment;
use time::OffsetDateTime;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use url::Url;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};

pub(crate) struct JsAsyncRead {
    fut: Option<JsFuture>,
    f: Function,
}

impl JsAsyncRead {
    fn new(cb: Function) -> Self {
        Self { fut: None, f: cb }
    }
}

impl AsyncRead for JsAsyncRead {
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
            
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `environment.glob_files_normalized` | function | No description |
| `environment.find_config_file_normalized` | function | No description |
| `environment.cwd_normalized` | function | No description |
| `environment.to_file_path_normalized` | function | No description |
| `environment.Environment` | trait | No description |

## Git History

- Commits: 6
- Authors: 2
- Bug fixes: 3
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
