# Module: listen

**File:** `\crates\lsp-async-stub\src\listen.rs`

## Overview

Module `listen` contains 5 analyzed entities.

```
use crate::{
    rpc::{self, Message, Request},
    Server,
};
use anyhow::anyhow;
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    SinkExt, Stream, StreamExt,
};
use lsp_types::NumberOrString;
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader},
    select,
};

#[cfg(feature = "tokio-stdio")]
mod stdio;

#[cfg(feature = "tokio-tcp")]
mod tcp;

impl<W: Clone + 'static> Server<W> {
    pub(crate) async fn listen_loop(
        self,
        world: W,
        mut input: UnboundedReceiver<Message>,
        output: UnboundedSender<Message>,
        mut shutdown_signals: impl Stream<Item = ()> + Unpin,
    ) -> Result<(), anyhow::Error> {
        let ls = tokio::task::LocalSet::new();

        ls.run_until(async move {
            'l: loop {
                tokio::select! {
                    _ = shutdown_signals.next() => {
                        tracing::info!("shutdown signal received, shutting down...");

                        let task_fut = self.handle_message(
                            world.clone(),
                            Request::<()>::new()
                                .with_id(Some(NumberOrString::String(
                                    "external_shutdown".to_string(),
                                )))
                                .with_method("shutdown")
                                .into_message(),
                            output.clone().sink_map_err(|e| panic!("{}", e)),
                        );

                        select!{
                            _ = shutdown_signals.next() => {
                                tracing::warn!("shut down forcibly");
                            },
                            _ = task_fut => {}
                        }

                        drop(output);
                        drop(input);
                        break 'l;
                    },
                    msg = input.next() => {
         
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `listen.create_output` | function | No description |
| `listen.listen_loop` | function | No description |
| `listen.create_input` | function | No description |
| `listen.write_message` | function | No description |
| `listen.read_message` | function | No description |

## Git History

- Commits: 3
- Authors: 3
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
