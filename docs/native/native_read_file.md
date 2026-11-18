# native.read_file

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 90-92)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn read_file(&self, path: &Path) -> Result<Vec<u8>, anyhow::Error> {
        Ok(tokio::fs::read(path).await?)
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn read_file(&self, path: &Path) -> Result<Vec<u8>, anyhow::Error> {
        Ok(tokio::fs::read(path).await?)
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
