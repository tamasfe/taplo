# associations.SchemaStoreSchemaMeta

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 494-504)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct SchemaStoreSchemaMeta {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub url: Url,
    #[serde(default)]
    pub file_match: Vec<String>,
    #[serde(default)]
    pub versions: IndexMap<String, Url>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct SchemaStoreSchemaMeta {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub url: Url,
    #[serde(default)]
    pub file_match: Vec<String>,
    #[serde(default)]
    pub versions: IndexMap<String, Url>,
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
