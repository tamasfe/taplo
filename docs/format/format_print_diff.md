# format.print_diff

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\format.rs` (lines 84-92)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn print_diff(
        &self,
        _path: impl AsRef<Path>,
        _original: &str,
        _formatted: &str,
    ) -> Result<(), anyhow::Error> {
        tracing::warn!("the `--diff` flag is not available in this build yet");
        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn print_diff(
        &self,
        _path: impl AsRef<Path>,
        _original: &str,
        _formatted: &str,
    ) -> Result<(), anyhow::Error> {
        tracing::warn!("the `--diff` flag is not available in this build yet");
        Ok(())
    }
```

## Git History

- Commits: 19
- Authors: 6
- Bug fixes: 6
- Features: 2

## Callers

None

## Callees

None
