# config.SchemaConfig

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\config.rs` (lines 82-88)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct SchemaConfig {
    pub enabled: bool,
    pub associations: HashMap<String, String>,
    pub catalogs: Vec<Url>,
    pub links: bool,
    pub cache: SchemaCacheConfig,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct SchemaConfig {
    pub enabled: bool,
    pub associations: HashMap<String, String>,
    pub catalogs: Vec<Url>,
    pub links: bool,
    pub cache: SchemaCacheConfig,
}
```

## Git History

- Commits: 3
- Authors: 1
- Bug fixes: 1
- Features: 2

## Callers

None

## Callees

None
