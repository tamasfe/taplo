# serde.visit_str

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\serde.rs` (lines 125-137)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StrInner {
            errors: Default::default(),
            repr: super::node::StrRepr::Basic,
            syntax: None,
            value: v.to_string().into(),
        }
        .wrap()
        .into())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StrInner {
            errors: Default::default(),
            repr: super::node::StrRepr::Basic,
            syntax: None,
            value: v.to_string().into(),
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
