# serde.deserialize

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\serde.rs` (lines 264-269)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        de.deserialize_any(TomlVisitor)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        de.deserialize_any(TomlVisitor)
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
