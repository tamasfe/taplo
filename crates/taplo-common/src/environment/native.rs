use std::path::Path;

use crate::config::CONFIG_FILE_NAMES;

use super::Environment;
use async_trait::async_trait;
use time::OffsetDateTime;

#[derive(Clone)]
pub struct NativeEnvironment {
    handle: tokio::runtime::Handle,
}

impl NativeEnvironment {
    #[must_use]
    pub fn new() -> Self {
        Self {
            handle: tokio::runtime::Handle::current(),
        }
    }
}

impl Default for NativeEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl Environment for NativeEnvironment {
    type Stdin = tokio::io::Stdin;
    type Stdout = tokio::io::Stdout;
    type Stderr = tokio::io::Stderr;

    fn now(&self) -> time::OffsetDateTime {
        OffsetDateTime::now_utc()
    }

    fn spawn<F>(&self, fut: F)
    where
        F: futures::Future + Send + 'static,
        F::Output: Send,
    {
        self.handle.spawn(fut);
    }

    fn spawn_local<F>(&self, fut: F)
    where
        F: futures::Future + 'static,
    {
        tokio::task::spawn_local(fut);
    }

    fn env_var(&self, name: &str) -> Option<String> {
        std::env::var(name).ok()
    }

    fn atty_stderr(&self) -> bool {
        atty::is(atty::Stream::Stderr)
    }

    fn stdin(&self) -> Self::Stdin {
        tokio::io::stdin()
    }

    fn stdout(&self) -> Self::Stdout {
        tokio::io::stdout()
    }

    fn stderr(&self) -> Self::Stderr {
        tokio::io::stderr()
    }

    fn glob_files(&self, pattern: &str) -> Result<Vec<std::path::PathBuf>, anyhow::Error> {
        let paths = glob::glob_with(
            pattern,
            glob::MatchOptions {
                case_sensitive: true,
                ..Default::default()
            },
        )?;
        Ok(paths.filter_map(Result::ok).collect())
    }

    async fn read_file(&self, path: &Path) -> Result<Vec<u8>, anyhow::Error> {
        Ok(tokio::fs::read(path).await?)
    }

    async fn write_file(&self, path: &std::path::Path, bytes: &[u8]) -> Result<(), anyhow::Error> {
        Ok(tokio::fs::write(path, bytes).await?)
    }

    fn to_file_path(&self, url: &reqwest::Url) -> Option<std::path::PathBuf> {
        url.to_file_path().ok()
    }

    fn is_absolute(&self, base: &std::path::Path) -> bool {
        base.is_absolute()
    }

    fn cwd(&self) -> Option<std::path::PathBuf> {
        std::env::current_dir().ok()
    }

    async fn find_config_file(&self, from: &Path) -> Option<std::path::PathBuf> {
        let mut p = from;

        loop {
            if let Ok(mut dir) = tokio::fs::read_dir(p).await {
                while let Ok(Some(entry)) = dir.next_entry().await {
                    for name in CONFIG_FILE_NAMES {
                        if entry.file_name() == *name {
                            let path = entry.path();
                            return Some(path);
                        }
                    }
                }
            }

            match p.parent() {
                Some(parent) => p = parent,
                None => {
                    return None;
                }
            }
        }
    }
}
