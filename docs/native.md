# Module: native

**File:** `\crates\taplo-common\src\environment\native.rs`

## Overview

Module `native` contains 19 analyzed entities.

```
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

    fn env_vars(&self) -> Vec<(String, String)> {
        std::env::vars().collect()
    }

    fn atty_stderr(&self) -> bool {
        use std::io::IsTerminal;
        std::io::stderr().is_terminal()
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
        Ok(tokio::fs::read(
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `native.glob_files` | function | No description |
| `native.read_file` | function | No description |
| `native.is_absolute` | function | No description |
| `native.now` | function | No description |
| `native.spawn` | function | No description |
| `native.default` | function | No description |
| `native.spawn_local` | function | No description |
| `native.env_var` | function | No description |
| `native.env_vars` | function | No description |
| `native.atty_stderr` | function | No description |
| `native.stdin` | function | No description |
| `native.new` | function | No description |
| `native.stdout` | function | No description |
| `native.stderr` | function | No description |
| `native.write_file` | function | No description |
| `native.cwd` | function | No description |
| `native.find_config_file` | function | No description |
| `native.to_file_path` | function | No description |
| `native.NativeEnvironment` | struct | No description |

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 1
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
