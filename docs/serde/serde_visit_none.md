# serde.visit_none

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\serde.rs` (lines 150-158)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &self,
        ))
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &self,
        ))
    }
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
