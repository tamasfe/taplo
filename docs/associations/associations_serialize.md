# associations.serialize

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 530-535)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SCHEMA_STORE_CATALOG_SCHEMA_URL.serialize(serializer)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SCHEMA_STORE_CATALOG_SCHEMA_URL.serialize(serializer)
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
