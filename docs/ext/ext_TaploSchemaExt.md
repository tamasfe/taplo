# ext.TaploSchemaExt

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\ext.rs` (lines 9-16)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct TaploSchemaExt {
    pub hidden: Option<bool>,
    pub links: Option<ExtLinks>,
    pub docs: Option<ExtDocs>,
    pub init_keys: Option<Vec<String>>,
    #[serde(default)]
    pub plugins: Vec<String>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct TaploSchemaExt {
    pub hidden: Option<bool>,
    pub links: Option<ExtLinks>,
    pub docs: Option<ExtDocs>,
    pub init_keys: Option<Vec<String>>,
    #[serde(default)]
    pub plugins: Vec<String>,
}
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
