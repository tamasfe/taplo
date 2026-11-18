# format.execute_format

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\format.rs` (lines 15-21)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn execute_format(&mut self, cmd: FormatCommand) -> Result<(), anyhow::Error> {
        if matches!(cmd.files.first().map(|it| it.as_str()), Some("-")) {
            self.format_stdin(cmd).await
        } else {
            self.format_files(cmd).await
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub async fn execute_format(&mut self, cmd: FormatCommand) -> Result<(), anyhow::Error> {
        if matches!(cmd.files.first().map(|it| it.as_str()), Some("-")) {
            self.format_stdin(cmd).await
        } else {
            self.format_files(cmd).await
        }
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
