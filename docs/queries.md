# Module: queries

**File:** `\crates\taplo-cli\src\commands\queries.rs`

## Overview

Module `queries` contains 2 analyzed entities.

```
use std::borrow::Cow;

use crate::{
    args::{GetCommand, OutputFormat},
    Taplo,
};
use anyhow::anyhow;
use codespan_reporting::files::SimpleFile;
use taplo::{
    dom::{Keys, Node},
    parser,
};
use taplo_common::environment::Environment;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

impl<E: Environment> Taplo<E> {
    pub async fn execute_get(&self, cmd: GetCommand) -> Result<(), anyhow::Error> {
        let mut stdout = self.env.stdout();

        // `--separator` should only be handled for a text output format
        if cmd.separator.is_some() && !matches!(cmd.output_format, OutputFormat::Value) {
            return Err(anyhow!(
                "`--separator` is only valid for `--output-format value`"
            ));
        }

        let source = match &cmd.file_path {
            Some(p) => String::from_utf8(self.env.read_file(p).await?)?,
            None => {
                let mut stdin = self.env.stdin();
                let mut s = String::new();
                stdin.read_to_string(&mut s).await?;
                s
            }
        };

        let parse = parser::parse(&source);

        let file_path = cmd
            .file_path
            .as_ref()
            .map(|p| p.to_string_lossy())
            .unwrap_or(Cow::Borrowed("-"));

        self.print_parse_errors(&SimpleFile::new(&file_path, &source), &parse.errors)
            .await?;

        if !parse.errors.is_empty() {
            return Err(anyhow!("syntax errors found"));
        }

        let node = parse.into_dom();

        if let Err(errors) = node.validate() {
            self.print_semantic_errors(&SimpleFile::new(&file_path, &source), errors)
                .await?;

            return Err(anyhow!("semantic errors found"));
        }

        match cmd.output_format {
            crate::args::OutputFormat::Json => {
                if let Some(p) = cmd.pattern {
                    let p = p.trim_start_matches('.');

                    let keys = p
                  
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `queries.execute_get` | function | No description |
| `queries.extract_value` | function | No description |

## Git History

- Commits: 8
- Authors: 2
- Bug fixes: 0
- Features: 3

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
