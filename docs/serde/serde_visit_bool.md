# serde.visit_bool

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\serde.rs` (lines 66-77)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BoolInner {
            errors: Default::default(),
            syntax: None,
            value: v.into(),
        }
        .wrap()
        .into())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BoolInner {
            errors: Default::default(),
            syntax: None,
            value: v.into(),
        }
        .wrap()
        .into())
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
