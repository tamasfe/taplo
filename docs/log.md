# Module: log

**File:** `\crates\taplo-common\src\log.rs`

## Overview

Module `log` contains 4 analyzed entities.

```
use std::io::{self, Write};
use tokio::io::{AsyncWrite, AsyncWriteExt};
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*, util::SubscriberInitExt, EnvFilter};

use crate::environment::Environment;

struct BlockingWrite<W: AsyncWrite>(W);

impl<W: AsyncWrite + Unpin> Write for BlockingWrite<W> {
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
}

pub fn setup_stderr_logging(e: impl Environment, spans: bool, verbose: bool, colors: Option<bool>) {
    let registry = tracing_subscriber::registry();

    let colors = match colors {
        None => e.atty_stderr(),
        Some(v) => v,
    };

    let registry = registry.with(match e.env_var("RUST_LOG") {
        Some(log) => EnvFilter::new(log),
        None => EnvFilter::default().add_directive(tracing::Level::INFO.int
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `log.write` | function | No description |
| `log.flush` | function | No description |
| `log.setup_stderr_logging` | function | No description |
| `log.BlockingWrite` | struct | No description |

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
