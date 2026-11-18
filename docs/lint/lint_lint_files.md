# lint.lint_files

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\lint.rs` (lines 71-93)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn lint_files(&mut self, cmd: LintCommand) -> Result<(), anyhow::Error> {
        let config = self.load_config(&cmd.general).await?;

        let cwd = self
            .env
            .cwd_normalized()
            .ok_or_else(|| anyhow!("could not figure the current working directory"))?;

        let files = self
            .collect_files(&cwd, &config, cmd.files.into_iter())
            .await?;

        let mut result = Ok(());

        for file in files {
            if let Err(error) = self.lint_file(&file).await {
                tracing::error!(%error, path = ?file, "invalid file");
                result = Err(anyhow!("some files were not valid"));
            }
        }

        result
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn lint_files(&mut self, cmd: LintCommand) -> Result<(), anyhow::Error> {
        let config = self.load_config(&cmd.general).await?;

        let cwd = self
            .env
            .cwd_normalized()
            .ok_or_else(|| anyhow!("could not figure the current working directory"))?;

        let files = self
            .collect_files(&cwd, &config, cmd.files.into_iter())
            .await?;

        let mut result = Ok(());

        for file in files {
            if let Err(error) = self.lint_file(&file).await {
                tracing::error!(%error, path = ?file, "invalid file");
                result = Err(anyhow!("some files were not valid"));
            }
        }

        result
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
