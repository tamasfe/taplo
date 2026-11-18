# Module: format

**File:** `\crates\taplo-cli\src\commands\format.rs`

## Overview

Module `format` contains 6 analyzed entities.

```
use std::{
    mem,
    path::{Path, PathBuf},
};

use crate::{args::FormatCommand, Taplo};
use anyhow::anyhow;
use codespan_reporting::files::SimpleFile;

use taplo::{formatter, parser};
use taplo_common::{config::Config, environment::Environment, util::Normalize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

impl<E: Environment> Taplo<E> {
    pub async fn execute_format(&mut self, cmd: FormatCommand) -> Result<(), anyhow::Error> {
        if matches!(cmd.files.first().map(|it| it.as_str()), Some("-")) {
            self.format_stdin(cmd).await
        } else {
            self.format_files(cmd).await
        }
    }

    #[tracing::instrument(skip_all)]
    async fn format_stdin(&mut self, cmd: FormatCommand) -> Result<(), anyhow::Error> {
        let mut source = String::new();
        self.env.stdin().read_to_string(&mut source).await?;

        let config = self.load_config(&cmd.general).await?;
        let display_path = match cmd.stdin_filepath.as_deref() {
            Some(filepath) if self.env.is_absolute(filepath.as_ref()) => {
                PathBuf::from(filepath).normalize()
            }
            Some(filepath) => {
                let cwd = self
                    .env
                    .cwd_normalized()
                    .ok_or_else(|| anyhow!("could not figure the current working directory"))?;
                cwd.join(filepath).normalize()
            }
            None => PathBuf::from("-"),
        };
        let p = parser::parse(&source);

        if !p.errors.is_empty() {
            self.print_parse_errors(
                &SimpleFile::new(&display_path.to_string_lossy(), source.as_str()),
                &p.errors,
            )
            .await?;

            if !cmd.force {
                return Err(anyhow!("no formatting was done due to syntax errors"));
            }
        }

        let format_opts = self.format_options(&config, &cmd, &display_path)?;

        let error_ranges = p.errors.iter().map(|e| e.range).collect:
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `format.print_diff` | function | No description |
| `format.execute_format` | function | No description |
| `format.apply_color` | function | No description |
| `format.format_options` | function | No description |
| `format.format_stdin` | function | No description |
| `format.format_files` | function | No description |

## Git History

- Commits: 19
- Authors: 6
- Bug fixes: 6
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
