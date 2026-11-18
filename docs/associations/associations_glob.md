# associations.glob

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 392-394)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn glob(pattern: &str) -> Result<Self, anyhow::Error> {
        Ok(Self::Glob(GlobRule::new([pattern], &[] as &[&str])?))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn glob(pattern: &str) -> Result<Self, anyhow::Error> {
        Ok(Self::Glob(GlobRule::new([pattern], &[] as &[&str])?))
    }
```

## Git History

- Commits: 16
- Authors: 5
- Bug fixes: 7
- Features: 4

## Callers

None

## Callees

None
