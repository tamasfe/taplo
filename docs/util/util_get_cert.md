# util.get_cert

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\util.rs` (lines 131-143)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn get_cert(path: &Path) -> Result<reqwest::Certificate, anyhow::Error> {
            let is_der = path.extension().is_some_and(|ext| ext == "der");
            let buf = std::fs::read(path)?;
            tracing::info!(
                "Found a custom CA {}. Reading the CA...",
                path.to_string_lossy()
            );
            if is_der {
                Ok(reqwest::Certificate::from_der(&buf)?)
            } else {
                Ok(reqwest::Certificate::from_pem(&buf)?)
            }
        }
```

## Docstring

_No docstring provided._

## Source

```
        fn get_cert(path: &Path) -> Result<reqwest::Certificate, anyhow::Error> {
            let is_der = path.extension().is_some_and(|ext| ext == "der");
            let buf = std::fs::read(path)?;
            tracing::info!(
                "Found a custom CA {}. Reading the CA...",
                path.to_string_lossy()
            );
            if is_der {
                Ok(reqwest::Certificate::from_der(&buf)?)
            } else {
                Ok(reqwest::Certificate::from_pem(&buf)?)
            }
        }
```

## Git History

- Commits: 11
- Authors: 4
- Bug fixes: 7
- Features: 4

## Callers

None

## Callees

None
