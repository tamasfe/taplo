use async_trait::async_trait;
use futures::Future;
use std::path::{Path, PathBuf};
use time::OffsetDateTime;
use tokio::io::{AsyncRead, AsyncWrite};
use url::Url;

use crate::util::Normalize;

#[cfg(not(target_family = "wasm"))]
pub mod native;

/// An environment in which the operations with Taplo are executed.
///
/// This is mostly needed for sandboxed environments such as WebAssembly.
#[async_trait(?Send)]
pub trait Environment: Clone + Send + Sync + 'static {
    type Stdin: AsyncRead + Unpin;
    type Stdout: AsyncWrite + Unpin;
    type Stderr: AsyncWrite + Unpin;

    fn now(&self) -> OffsetDateTime;

    fn spawn<F>(&self, fut: F)
    where
        F: Future + Send + 'static,
        F::Output: Send;

    fn spawn_local<F>(&self, fut: F)
    where
        F: Future + 'static;

    fn env_var(&self, name: &str) -> Option<String>;

    fn env_vars(&self) -> Vec<(String, String)>;

    fn atty_stderr(&self) -> bool;
    fn stdin(&self) -> Self::Stdin;
    fn stdout(&self) -> Self::Stdout;
    fn stderr(&self) -> Self::Stderr;

    fn glob_files(&self, glob: &str) -> Result<Vec<PathBuf>, anyhow::Error>;

    async fn read_file(&self, path: &Path) -> Result<Vec<u8>, anyhow::Error>;

    async fn write_file(&self, path: &Path, bytes: &[u8]) -> Result<(), anyhow::Error>;

    fn to_file_path(&self, url: &Url) -> Option<PathBuf>;

    fn is_absolute(&self, path: &Path) -> bool;

    /// Absolute current working dir.
    fn cwd(&self) -> Option<PathBuf>;

    async fn find_config_file(&self, from: &Path) -> Option<PathBuf>;

    /// Same as [`Self::glob_files`], but the returned paths are
    /// [normalized](Normalize:normalize) in addition.
    fn glob_files_normalized(&self, glob: &str) -> Result<Vec<PathBuf>, anyhow::Error> {
        Ok(self
            .glob_files(glob)?
            .into_iter()
            .map(Normalize::normalize)
            .collect())
    }

    /// Same as [`Self::cwd`], but the returned path is
    /// [normalized](Normalize:normalize) in addition.
    fn cwd_normalized(&self) -> Option<PathBuf> {
        self.cwd().map(Normalize::normalize)
    }

    /// Same as [`Self::to_file_path`], but the returned path is
    /// [normalized](Normalize:normalize) in addition.
    fn to_file_path_normalized(&self, url: &Url) -> Option<PathBuf> {
        self.to_file_path(url).map(Normalize::normalize)
    }

    /// Same as [`Self::find_config_file`], but the returned path is
    /// [normalized](Normalize:normalize) in addition.
    async fn find_config_file_normalized(&self, from: &Path) -> Option<PathBuf> {
        self.find_config_file(from).await.map(Normalize::normalize)
    }
}
