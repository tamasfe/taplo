# native.write_file

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\environment\native.rs` (lines 94-96)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn write_file(&self, path: &std::path::Path, bytes: &[u8]) -> Result<(), anyhow::Error> {
        Ok(tokio::fs::write(path, bytes).await?)
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn write_file(&self, path: &std::path::Path, bytes: &[u8]) -> Result<(), anyhow::Error> {
        Ok(tokio::fs::write(path, bytes).await?)
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
