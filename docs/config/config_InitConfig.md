# config.InitConfig

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\config.rs` (lines 14-18)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct InitConfig {
    pub cache_path: Option<PathBuf>,
    #[serde(default = "default_configuration_section")]
    pub configuration_section: String,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct InitConfig {
    pub cache_path: Option<PathBuf>,
    #[serde(default = "default_configuration_section")]
    pub configuration_section: String,
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
