# config.execute_config

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\config.rs` (lines 8-26)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn execute_config(&self, cmd: ConfigCommand) -> Result<(), anyhow::Error> {
        let mut stdout = self.env.stdout();
        match cmd {
            ConfigCommand::Default => {
                stdout
                    .write_all(toml::to_string_pretty(&default_config())?.as_bytes())
                    .await?;
                stdout.flush().await?;
                Ok(())
            }
            ConfigCommand::Schema => {
                stdout
                    .write_all(serde_json::to_string_pretty(&schema_for!(Config))?.as_bytes())
                    .await?;
                stdout.flush().await?;
                Ok(())
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub async fn execute_config(&self, cmd: ConfigCommand) -> Result<(), anyhow::Error> {
        let mut stdout = self.env.stdout();
        match cmd {
            ConfigCommand::Default => {
                stdout
                    .write_all(toml::to_string_pretty(&default_config())?.as_bytes())
                    .await?;
                stdout.flush().await?;
                Ok(())
            }
            ConfigCommand::Schema => {
                stdout
                    .write_all(serde_json::to_string_pretty(&schema_for!(Config))?.as_bytes())
                    .await?;
                stdout.flush().await?;
                Ok(())
            }
        }
    }
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
