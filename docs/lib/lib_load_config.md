# lib.load_config

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\lib.rs` (lines 44-88)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn load_config(&mut self, general: &GeneralArgs) -> Result<Arc<Config>, anyhow::Error> {
        if let Some(c) = self.config.clone() {
            return Ok(c);
        }

        let mut config_path = general.config.clone();

        if config_path.is_none() && !general.no_auto_config {
            if let Some(cwd) = self.env.cwd_normalized() {
                config_path = self.env.find_config_file_normalized(&cwd).await
            }
        }

        let mut config = Config::default();
        if let Some(c) = config_path {
            tracing::info!(path = ?c, "found configuration file");
            match self.env.read_file(&c).await {
                Ok(cfg) => match toml::from_str(str::from_utf8(&cfg)?) {
                    Ok(c) => config = c,
                    Err(error) => {
                        tracing::warn!(%error, "invalid configuration file");
                    }
                },
                Err(error) => {
                    tracing::warn!(%error, "failed to read configuration file");
                }
            }
        }

        config
            .prepare(
                &self.env,
                &self
                    .env
                    .cwd_normalized()
                    .ok_or_else(|| anyhow!("working directory is required"))?,
            )
            .context("invalid configuration")?;

        let c = Arc::new(config);

        self.config = Some(c.clone());

        Ok(c)
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn load_config(&mut self, general: &GeneralArgs) -> Result<Arc<Config>, anyhow::Error> {
        if let Some(c) = self.config.clone() {
            return Ok(c);
        }

        let mut config_path = general.config.clone();

        if config_path.is_none() && !general.no_auto_config {
            if let Some(cwd) = self.env.cwd_normalized() {
                config_path = self.env.find_config_file_normalized(&cwd).await
            }
        }

        let mut config = Config::default();
        if let Some(c) = config_path {
            tracing::info!(path = ?c, "found configuration file");
            match self.env.read_file(&c).await {
                Ok(cfg) => match toml::from_str(str::from_utf8(&cfg)?) {
                    Ok(c) => config = c,
                    Err(error) => {
                        tracing::warn!(%error, "invalid configuration file");
                    }
                },
                Err(error) => {
                    tracing::warn!(%error, "failed to read configuration file");
                }
            }
        }

        config
            .prepare(
                &self.env,
                &self
                    .env
                    .cwd_normalized()
                    .ok_or_else(|| anyhow!("working directory is required"))?,
            )
            .context("invalid configuration")?;

        let c = Arc::new(config);

        self.config = Some(c.clone());

        Ok(c)
    }
```

## Git History

- Commits: 10
- Authors: 4
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
