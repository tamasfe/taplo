# serde.visit_f64

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\serde.rs` (lines 112-123)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(FloatInner {
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
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(FloatInner {
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
