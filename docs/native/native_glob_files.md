# native.glob_files

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 79-88)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
```

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 1
- Features: 2

## Callers

None

## Callees

None
