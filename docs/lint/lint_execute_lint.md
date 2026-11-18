# lint.execute_lint

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\lint.rs` (lines 16-61)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
            self.lint_stdin(cmd).await
        } else {
            self.lint_files(cmd).await
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
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
            self.lint_stdin(cmd).await
        } else {
            self.lint_files(cmd).await
        }
    }
```

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 1
- Features: 3

## Callers

None

## Callees

None
