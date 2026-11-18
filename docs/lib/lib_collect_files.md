# lib.collect_files

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\lib.rs` (lines 91-146)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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

        tracing::info!(total, excluded, "found files");
        tracing::debug!(?files, "file details");

        Ok(files)
    }
```

## Docstring

_No docstring provided._

## Source

```
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

        tracing::info!(total, excluded, "found files");
        tracing::debug!(?files, "file details");

        Ok(files)
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
