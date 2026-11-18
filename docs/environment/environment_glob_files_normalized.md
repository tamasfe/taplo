# environment.glob_files_normalized

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment.rs` (lines 59-65)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn glob_files_normalized(&self, glob: &str) -> Result<Vec<PathBuf>, anyhow::Error> {
        Ok(self
            .glob_files(glob)?
            .into_iter()
            .map(Normalize::normalize)
            .collect())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn glob_files_normalized(&self, glob: &str) -> Result<Vec<PathBuf>, anyhow::Error> {
        Ok(self
            .glob_files(glob)?
            .into_iter()
            .map(Normalize::normalize)
            .collect())
    }
```

## Git History

- Commits: 4
- Authors: 1
- Bug fixes: 1
- Features: 3

## Callers

None

## Callees

None
