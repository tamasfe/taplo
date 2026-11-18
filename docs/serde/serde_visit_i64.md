# serde.visit_i64

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\serde.rs` (lines 79-96)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(IntegerInner {
            errors: Default::default(),
            syntax: None,
            repr: super::node::IntegerRepr::Dec,
            value: if v.is_negative() {
                IntegerValue::Negative(v)
            } else {
                IntegerValue::Positive(v as _)
            }
            .into(),
        }
        .wrap()
        .into())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(IntegerInner {
            errors: Default::default(),
            syntax: None,
            repr: super::node::IntegerRepr::Dec,
            value: if v.is_negative() {
                IntegerValue::Negative(v)
            } else {
                IntegerValue::Positive(v as _)
            }
            .into(),
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
