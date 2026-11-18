# lint.lint_file

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\lint.rs` (lines 95-99)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn lint_file(&self, file: &Path) -> Result<(), anyhow::Error> {
        let source = self.env.read_file(file).await?;
        let source = String::from_utf8(source)?;
        self.lint_source(&file.to_string_lossy(), &source).await
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn lint_file(&self, file: &Path) -> Result<(), anyhow::Error> {
        let source = self.env.read_file(file).await?;
        let source = String::from_utf8(source)?;
        self.lint_source(&file.to_string_lossy(), &source).await
    }
```

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 1
- Features: 3

## Callers

None

## Callees

None
