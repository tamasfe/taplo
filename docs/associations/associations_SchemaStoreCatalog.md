# associations.SchemaStoreCatalog

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 486-490)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct SchemaStoreCatalog {
    #[serde(rename = "$schema")]
    pub schema: SchemaStoreCatalogSchema,
    pub schemas: Vec<SchemaStoreSchemaMeta>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct SchemaStoreCatalog {
    #[serde(rename = "$schema")]
    pub schema: SchemaStoreCatalogSchema,
    pub schemas: Vec<SchemaStoreSchemaMeta>,
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
