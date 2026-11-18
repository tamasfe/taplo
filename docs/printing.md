# Module: printing

**File:** `\crates\taplo-cli\src\printing.rs`

## Overview

Module `printing` contains 4 analyzed entities.

```
use crate::Taplo;
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{Ansi, NoColor},
    },
};
use itertools::Itertools;
use std::ops::Range;
use taplo::{dom, parser, rowan::TextRange};
use taplo_common::environment::Environment;
#[cfg(feature = "lint")]
use taplo_common::schema::NodeValidationError;
use tokio::io::AsyncWriteExt;

impl<E: Environment> Taplo<E> {
    pub(crate) async fn print_parse_errors(
        &self,
        file: &SimpleFile<&str, &str>,
        errors: &[parser::Error],
    ) -> Result<(), anyhow::Error> {
        let mut out_diag = Vec::<u8>::new();

        let config = codespan_reporting::term::Config::default();

        for error in errors.iter().unique_by(|e| e.range) {
            let diag = Diagnostic::error()
                .with_message("invalid TOML")
                .with_labels(Vec::from([
                    Label::primary((), std_range(error.range)).with_message(&error.message)
                ]));

            if self.colors {
                term::emit(&mut Ansi::new(&mut out_diag), &config, file, &diag)?;
            } else {
                term::emit(&mut NoColor::new(&mut out_diag), &config, file, &diag)?;
            }
        }

        let mut stderr = self.env.stderr();

        stderr.write_all(&out_diag).await?;
        stderr.flush().await?;

        Ok(())
    }

    pub(crate) async fn print_semantic_errors(
        &self,
        file: &SimpleFile<&str, &str>,
        errors: impl Iterator<Item = dom::Error>,
    ) -> Result<(), anyhow::Error> {
        let mut out_diag = Vec::<u8>::new();

        let config = codespan_reporting::term::Config::default();

        for error in errors {
            let diag = match &error {
                dom::Error::ConflictingKeys { key, other } => Diagnostic::error()
                    .with_message(error.to_string())
                    .with_labels(Vec::from([
                        Label::pri
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `printing.print_parse_errors` | function | No description |
| `printing.std_range` | function | No description |
| `printing.print_schema_errors` | function | No description |
| `printing.print_semantic_errors` | function | No description |

## Git History

- Commits: 3
- Authors: 3
- Bug fixes: 1
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
