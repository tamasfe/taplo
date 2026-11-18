# mod.from_str

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 206-214)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = Parser::new(s).parse_key_only();
        if let Some(err) = p.errors.pop() {
            return Err(QueryError::InvalidKey(err).into());
        }
        Ok(Keys::new(
            keys_from_syntax(&p.into_syntax().into()).map(Into::into),
        ))
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = Parser::new(s).parse_key_only();
        if let Some(err) = p.errors.pop() {
            return Err(QueryError::InvalidKey(err).into());
        }
        Ok(Keys::new(
            keys_from_syntax(&p.into_syntax().into()).map(Into::into),
        ))
    }
```

## Git History

- Commits: 3
- Authors: 3
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
