# serde.visit_some

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\serde.rs` (lines 160-165)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
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
