# associations.regex

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 396-398)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn regex(regex: &str) -> Result<Self, anyhow::Error> {
        Ok(Self::Regex(Regex::new(regex)?))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn regex(regex: &str) -> Result<Self, anyhow::Error> {
        Ok(Self::Regex(Regex::new(regex)?))
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
