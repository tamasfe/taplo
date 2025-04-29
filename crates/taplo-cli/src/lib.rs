use anyhow::{anyhow, Context};
use args::GeneralArgs;
use itertools::Itertools;
use std::{
    path::{Path, PathBuf},
    str,
    sync::Arc,
};
#[cfg(feature = "lint")]
use taplo_common::schema::Schemas;
use taplo_common::{config::Config, environment::Environment, util::Normalize};

pub mod args;
pub mod commands;
pub mod printing;

pub struct Taplo<E: Environment> {
    env: E,
    colors: bool,
    #[cfg(feature = "lint")]
    schemas: Schemas<E>,
    config: Option<Arc<Config>>,
}

impl<E: Environment> Taplo<E> {
    pub fn new(env: E) -> Self {
        #[cfg(all(not(target_arch = "wasm32"), feature = "lint"))]
        let http =
            taplo_common::util::get_reqwest_client(std::time::Duration::from_secs(5)).unwrap();

        #[cfg(all(target_arch = "wasm32", feature = "lint"))]
        let http = reqwest::Client::default();

        Self {
            #[cfg(feature = "lint")]
            schemas: Schemas::new(env.clone(), http),
            colors: env.atty_stderr(),
            config: None,
            env,
        }
    }

    #[tracing::instrument(skip_all)]
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

    #[tracing::instrument(skip_all, fields(?cwd))]
    async fn collect_files(
        &self,
        cwd: &Path,
        config: &Config,
        arg_patterns: impl Iterator<Item = String>,
    ) -> Result<Vec<PathBuf>, anyhow::Error> {
        let mut patterns: Vec<String> = arg_patterns
            .map(|pat| {
                if !self.env.is_absolute(Path::new(&pat)) {
                    cwd.join(&pat).normalize().to_string_lossy().into_owned()
                } else {
                    pat
                }
            })
            .collect();

        if patterns.is_empty() {
            patterns = match config.include.clone() {
                Some(patterns) => patterns,
                None => Vec::from([cwd
                    .join("**/*.toml")
                    .normalize()
                    .to_string_lossy()
                    .into_owned()]),
            };
        };

        let patterns = patterns
            .into_iter()
            .unique()
            .map(|p| glob::Pattern::new(&p).map(|_| p))
            .collect::<Result<Vec<_>, _>>()?;

        let files = patterns
            .into_iter()
            .map(|pat| self.env.glob_files_normalized(&pat))
            .collect::<Result<Vec<_>, _>>()
            .into_iter()
            .flatten()
            .flatten()
            .collect::<Vec<_>>();

        let total = files.len();

        let files = files
            .into_iter()
            .filter(|path| config.is_included(path))
            .collect::<Vec<_>>();

        let excluded = total - files.len();

        tracing::info!(total, excluded, ?files, "found files");

        Ok(files)
    }
}

pub fn default_config() -> Config {
    Config {
        plugins: None,
        ..Default::default()
    }
}
