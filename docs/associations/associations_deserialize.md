# associations.deserialize

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 513-526)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = Cow::<'static, str>::deserialize(deserializer)?;

        if s != SCHEMA_STORE_CATALOG_SCHEMA_URL {
            return Err(Error::custom(format!(
                "expected $schema to be {SCHEMA_STORE_CATALOG_SCHEMA_URL}"
            )));
        }

        Ok(SchemaStoreCatalogSchema)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = Cow::<'static, str>::deserialize(deserializer)?;

        if s != SCHEMA_STORE_CATALOG_SCHEMA_URL {
            return Err(Error::custom(format!(
                "expected $schema to be {SCHEMA_STORE_CATALOG_SCHEMA_URL}"
            )));
        }

        Ok(SchemaStoreCatalogSchema)
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
