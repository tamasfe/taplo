# serde.visit_u64

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\serde.rs` (lines 98-110)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(IntegerInner {
            errors: Default::default(),
            syntax: None,
            repr: super::node::IntegerRepr::Dec,
            value: IntegerValue::Positive(v).into(),
        }
        .wrap()
        .into())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(IntegerInner {
            errors: Default::default(),
            syntax: None,
            repr: super::node::IntegerRepr::Dec,
            value: IntegerValue::Positive(v).into(),
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
