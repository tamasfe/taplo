# world.load_config

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\world.rs` (lines 218-261)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn load_config(
        &mut self,
        env: &impl Environment,
        default_config: &Config,
    ) -> Result<(), anyhow::Error> {
        self.taplo_config = default_config.clone();

        let root_path = env
            .to_file_path_normalized(&self.root)
            .ok_or_else(|| anyhow!("invalid root URL"))?;

        if self.config.taplo.config_file.enabled {
            let config_path = if let Some(p) = &self.config.taplo.config_file.path {
                tracing::debug!(path = ?p, "using config file at specified path");

                if env.is_absolute(p) {
                    Some(p.clone())
                } else if self.root != *DEFAULT_WORKSPACE_URL {
                    Some(root_path.join(p))
                } else {
                    tracing::debug!("relative config path is not valid for detached workspace");
                    None
                }
            } else if self.root != *DEFAULT_WORKSPACE_URL {
                tracing::debug!("discovering config file in workspace");
                env.find_config_file_normalized(&root_path).await
            } else {
                None
            };

            if let Some(config_path) = config_path {
                tracing::info!(path = ?config_path, "using config file");
                self.taplo_config =
                    toml::from_str(str::from_utf8(&env.read_file(&config_path).await?)?)?;
            }
        }

        self.taplo_config.rule.extend(self.config.rules.clone());
        self.taplo_config.prepare(env, &root_path)?;

        tracing::debug!("using config: {:#?}", self.taplo_config);

        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub(crate) async fn load_config(
        &mut self,
        env: &impl Environment,
        default_config: &Config,
    ) -> Result<(), anyhow::Error> {
        self.taplo_config = default_config.clone();

        let root_path = env
            .to_file_path_normalized(&self.root)
            .ok_or_else(|| anyhow!("invalid root URL"))?;

        if self.config.taplo.config_file.enabled {
            let config_path = if let Some(p) = &self.config.taplo.config_file.path {
                tracing::debug!(path = ?p, "using config file at specified path");

                if env.is_absolute(p) {
                    Some(p.clone())
                } else if self.root != *DEFAULT_WORKSPACE_URL {
                    Some(root_path.join(p))
                } else {
                    tracing::debug!("relative config path is not valid for detached workspace");
                    None
                }
            } else if self.root != *DEFAULT_WORKSPACE_URL {
                tracing::debug!("discovering config file in workspace");
                env.find_config_file_normalized(&root_path).await
            } else {
                None
            };

            if let Some(config_path) = config_path {
                tracing::info!(path = ?config_path, "using config file");
                self.taplo_config =
                    toml::from_str(str::from_utf8(&env.read_file(&config_path).await?)?)?;
            }
        }

        self.taplo_config.rule.extend(self.config.rules.clone());
        self.taplo_config.prepare(env, &root_path)?;

        tracing::debug!("using config: {:#?}", self.taplo_config);

        Ok(())
    }
```

## Git History

- Commits: 13
- Authors: 3
- Bug fixes: 4
- Features: 5

## Callers

None

## Callees

None
