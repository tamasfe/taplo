# args.LintCommand

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-cli\src\args.rs` (lines 156-182)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct LintCommand {
    #[clap(flatten)]
    pub general: GeneralArgs,

    /// URL to the schema to be used for validation.
    #[clap(long)]
    pub schema: Option<Url>,

    /// URL to a schema catalog (index) that is compatible with Schema Store or Taplo catalogs.
    ///
    /// Can be specified multiple times.
    #[clap(long)]
    pub schema_catalog: Vec<Url>,

    /// Use the default online catalogs for schemas.
    #[clap(long)]
    pub default_schema_catalogs: bool,

    /// Disable all schema validations.
    #[clap(long)]
    pub no_schema: bool,

    /// Paths or glob patterns to TOML documents.
    ///
    /// If the only argument is "-", the standard input will be used.
    pub files: Vec<String>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct LintCommand {
    #[clap(flatten)]
    pub general: GeneralArgs,

    /// URL to the schema to be used for validation.
    #[clap(long)]
    pub schema: Option<Url>,

    /// URL to a schema catalog (index) that is compatible with Schema Store or Taplo catalogs.
    ///
    /// Can be specified multiple times.
    #[clap(long)]
    pub schema_catalog: Vec<Url>,

    /// Use the default online catalogs for schemas.
    #[clap(long)]
    pub default_schema_catalogs: bool,

    /// Disable all schema validations.
    #[clap(long)]
    pub no_schema: bool,

    /// Paths or glob patterns to TOML documents.
    ///
    /// If the only argument is "-", the standard input will be used.
    pub files: Vec<String>,
}
```

## Git History

- Commits: 14
- Authors: 7
- Bug fixes: 1
- Features: 6

## Callers

None

## Callees

None
