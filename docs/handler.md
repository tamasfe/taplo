# Module: handler

**File:** `\crates\lsp-async-stub\src\handler.rs`

## Overview

Module `handler` contains 8 analyzed entities.

```
use super::{Context, MessageWriter, Params};
use crate::rpc;
use async_trait::async_trait;
use futures::{Future, SinkExt};
use lsp_types::{notification::Notification, request::Request};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

#[async_trait(?Send)]
pub(crate) trait Handler<W: Clone> {
    fn method(&self) -> &'static str;

    async fn handle(
        &mut self,
        context: Context<W>,
        message: rpc::Request<serde_json::Value>,
        writer: Option<&mut dyn MessageWriter>,
    );

    fn box_clone(&self) -> Box<dyn Handler<W>>;
}

impl<W: Clone> Clone for Box<dyn Handler<W>> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub struct RequestHandler<R, F, W>
where
    R: Request,
    F: Future<Output = Result<R::Result, rpc::Error>>,
    W: Clone,
{
    f: fn(Context<W>, Params<R::Params>) -> F,
    t: PhantomData<W>,
}

impl<R, F, W> Clone for RequestHandler<R, F, W>
where
    R: Request,
    F: Future<Output = Result<R::Result, rpc::Error>>,
    W: Clone,
{
    fn clone(&self) -> Self {
        Self {
            f: self.f,
            t: Default::default(),
        }
    }
}

impl<R, F, W> RequestHandler<R, F, W>
where
    R: Request,
    F: Future<Output = Result<R::Result, rpc::Error>>,
    W: Clone,
{
    pub fn new(f: fn(Context<W>, Params<R::Params>) -> F) -> Self {
        Self {
            f,
            t: Default::default(),
        }
    }
}

#[async_trait(?Send)]
impl<R, F, P, W> Handler<W> for RequestHandler<R, F, W>
where
    R: Request<Params = P> + 'static,
    P: Serialize + DeserializeOwned + 'static,
    F: Future<Output = Result<R::Result, rpc::Error>> + 'static,
    W: Clone + 'static,
{
    fn method(&self) -> &'static str {
        R::METHOD
    }

    async fn handle(
        &mut self,
        context: Context<W>,
        message: rpc::Request<serde_json::Value>,
        writer: Option<&mut dyn MessageWriter>,
    ) {
        let req_id = message.id.clone();
        let req
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `handler.new` | function | No description |
| `handler.clone` | function | No description |
| `handler.method` | function | No description |
| `handler.box_clone` | function | No description |
| `handler.handle` | function | No description |
| `handler.RequestHandler` | struct | No description |
| `handler.NotificationHandler` | struct | No description |
| `handler.Handler` | trait | No description |

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
