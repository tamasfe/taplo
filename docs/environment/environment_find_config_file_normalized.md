# environment.find_config_file_normalized

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment.rs` (lines 81-83)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn find_config_file_normalized(&self, from: &Path) -> Option<PathBuf> {
        self.find_config_file(from).await.map(Normalize::normalize)
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn find_config_file_normalized(&self, from: &Path) -> Option<PathBuf> {
        self.find_config_file(from).await.map(Normalize::normalize)
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
