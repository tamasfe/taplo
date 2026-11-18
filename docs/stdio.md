# Module: stdio

**File:** `\crates\lsp-async-stub\src\listen\stdio.rs`

## Overview

Module `stdio` contains 1 analyzed entities.

```
use crate::{
    listen::{create_input, create_output},
    Server,
};
use futures::Stream;

impl<W: Clone + 'static> Server<W> {
    pub async fn listen_stdio(
        self,
        world: W,
        shutdown_signals: impl Stream<Item = ()> + Unpin,
    ) -> Result<(), anyhow::Error> {
        let input = create_input(tokio::io::stdin());
        let output = create_output(tokio::io::stdout());

        tracing::info!(transport = "stdio", "LSP server listening");

        self.listen_loop(world, input, output, shutdown_signals)
            .await
    }
}
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `stdio.listen_stdio` | function | No description |

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
