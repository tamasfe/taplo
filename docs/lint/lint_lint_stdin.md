# lint.lint_stdin

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\lint.rs` (lines 64-68)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn lint_stdin(&self, _cmd: LintCommand) -> Result<(), anyhow::Error> {
        let mut source = String::new();
        self.env.stdin().read_to_string(&mut source).await?;
        self.lint_source("-", &source).await
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn lint_stdin(&self, _cmd: LintCommand) -> Result<(), anyhow::Error> {
        let mut source = String::new();
        self.env.stdin().read_to_string(&mut source).await?;
        self.lint_source("-", &source).await
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
