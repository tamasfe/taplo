use anyhow::{anyhow, Context};
use args::GeneralArgs;
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

pub type EntryIter<'a, F> = std::iter::FilterMap<ignore::Walk, F>;

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
    async fn files_iter(
        &self,
        cwd: &Path,
        config: &Config,
        arg_patterns: impl Iterator<Item = String>,
    ) -> Result<
        EntryIter<'_, Box<dyn FnMut(Result<ignore::DirEntry, ignore::Error>) -> Option<PathBuf>>>,
        anyhow::Error,
    > {
        tracing::trace!("Nomarlizing patterns to absolute ones...");

        let mut patterns: Vec<String> = arg_patterns
            .map(|pat| {
                if !self.env.is_absolute(Path::new(&pat)) {
                    let pat = cwd.join(&pat).normalize().to_string_lossy().into_owned();
                    tracing::debug!("Arg(abs) {} ", &pat);
                    pat
                } else {
                    tracing::debug!("Arg(rel) {} ", &pat);
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

        tracing::trace!("Compiled patterns");

        let mut skip_patterns = vec![];
        let mut keep_patterns = vec![];

        let opts = glob::MatchOptions {
            case_sensitive: true,
            ..Default::default()
        };

        let cwd = cwd.to_path_buf();

        let mut bldr = ignore::WalkBuilder::new(&cwd);
        bldr.git_ignore(true)
            .git_exclude(true)
            .git_global(true)
            .ignore(true)
            .hidden(false)
            .same_file_system(true);

        for skip_pattern in config.exclude.iter().flatten() {
            if let Ok(pat) = glob::Pattern::new(skip_pattern) {
                tracing::trace!("Compiling pattern: {skip_pattern}");
                skip_patterns.push(pat.clone());
                bldr.filter_entry(move |entry| !pat.matches_path_with(entry.path(), opts));
            }
        }
        for keep_pattern in config
            .include
            .iter()
            .flatten().cloned()
            .map(|x| x.to_owned())
            .chain(patterns.into_iter())
        {
            if let Ok(pat) = glob::Pattern::new(&keep_pattern) {
                tracing::trace!("Compiling pattern: {pat}");
                keep_patterns.push(pat.clone());
                bldr.filter_entry(move |entry| pat.matches_path_with(entry.path(), opts));
            }
        }
        let walker = bldr.build();

        Ok(walker.filter_map(Box::new(
            move |entry: Result<ignore::DirEntry, ignore::Error>| -> Option<PathBuf> {
                let entry = entry.ok()?;
                debug_assert!(!skip_patterns
                    .iter()
                    .any(|pat| pat.matches_path_with(entry.path(), opts)));
                debug_assert!(keep_patterns
                    .iter()
                    .any(|pat| pat.matches_path_with(entry.path(), opts)));
                if entry.path() == cwd {
                    None
                } else {
                    let p = entry.path().to_path_buf();
                    tracing::debug!("Path passed filters: {}", p.display());
                    Some(p)
                }
            },
        )))
    }
}

pub fn default_config() -> Config {
    Config {
        plugins: None,
        ..Default::default()
    }
}
