# Module: taplo

**File:** `\crates\taplo-cli\bin\taplo.rs`

## Overview

Module `taplo` contains 3 analyzed entities.

```
use clap::Parser;
use std::process::exit;
use taplo_cli::{
    args::{Colors, TaploArgs},
    Taplo,
};
use taplo_common::{environment::native::NativeEnvironment, log::setup_stderr_logging};
use tracing::Instrument;

#[tokio::main]
async fn main() {
    let cli = TaploArgs::parse();
    setup_stderr_logging(
        NativeEnvironment::new(),
        cli.log_spans,
        cli.verbose,
        match cli.colors {
            Colors::Auto => None,
            Colors::Always => Some(true),
            Colors::Never => Some(false),
        },
    );

    match Taplo::new(NativeEnvironment::new())
        .execute(cli)
        .instrument(tracing::info_span!("taplo"))
        .await
    {
        Ok(_) => {
            exit(0);
        }
        Err(error) => {
            tracing::error!(error = %format!("{error:#}"), "operation failed");
            exit(1);
        }
    }
}
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `taplo.parsing` | function | No description |
| `taplo.formatting` | function | No description |
| `taplo.conversion` | function | No description |

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
