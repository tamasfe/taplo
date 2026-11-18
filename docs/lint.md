# Module: lint

**File:** `\crates\taplo-cli\src\commands\lint.rs`

## Overview

Module `lint` contains 5 analyzed entities.

```
use std::path::Path;

use crate::{args::LintCommand, Taplo};
use anyhow::{anyhow, Context};
use codespan_reporting::files::SimpleFile;
use serde_json::json;
use taplo::parser;
use taplo_common::{
    environment::Environment,
    schema::associations::{AssociationRule, SchemaAssociation, DEFAULT_CATALOGS},
};
use tokio::io::AsyncReadExt;
use url::Url;

impl<E: Environment> Taplo<E> {
    pub async fn execute_lint(&mut self, cmd: LintCommand) -> Result<(), anyhow::Error> {
        self.schemas
            .cache()
            .set_cache_path(cmd.general.cache_path.clone());

        let config = self.load_config(&cmd.general).await?;

        if !cmd.no_schema {
            if let Some(schema_url) = cmd.schema.clone() {
                self.schemas.associations().add(
                    AssociationRule::regex(".*")?,
                    SchemaAssociation {
                        meta: json!({"source": "command-line"}),
                        url: schema_url,
                        priority: 999,
                    },
                );
            } else {
                self.schemas.associations().add_from_config(&config);

                for catalog in &cmd.schema_catalog {
                    self.schemas
                        .associations()
                        .add_from_catalog(catalog)
                        .await
                        .with_context(|| "failed to load schema catalog")?;
                }

                if cmd.default_schema_catalogs {
                    for catalog in DEFAULT_CATALOGS {
                        self.schemas
                            .associations()
                            .add_from_catalog(&Url::parse(catalog).unwrap())
                            .await
                            .with_context(|| "failed to load schema catalog")?;
                    }
                }
            }
        }

        if matches!(cmd.files.first().map(|it| it.as_str()), Some("-")) {
            self.lint_stdin(cmd).a
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `lint.lint_source` | function | No description |
| `lint.execute_lint` | function | No description |
| `lint.lint_stdin` | function | No description |
| `lint.lint_files` | function | No description |
| `lint.lint_file` | function | No description |

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 1
- Features: 3

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
