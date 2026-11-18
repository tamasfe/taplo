# associations.TaploSchemaMeta

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 464-474)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct TaploSchemaMeta {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    pub url: Url,
    pub url_hash: String,

    #[serde(flatten)]
    pub extra: TaploSchemaExtraInfo,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct TaploSchemaMeta {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    pub url: Url,
    pub url_hash: String,

    #[serde(flatten)]
    pub extra: TaploSchemaExtraInfo,
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
