# native.find_config_file

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 110-132)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
