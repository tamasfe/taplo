# args.GeneralArgs

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-cli\src\args.rs` (lines 24-36)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct GeneralArgs {
    /// Path to the Taplo configuration file.
    #[clap(long, short, env = "TAPLO_CONFIG")]
    pub config: Option<PathBuf>,

    /// Set a cache path.
    #[clap(long)]
    pub cache_path: Option<PathBuf>,

    /// Do not search for a configuration file.
    #[clap(long)]
    pub no_auto_config: bool,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct GeneralArgs {
    /// Path to the Taplo configuration file.
    #[clap(long, short, env = "TAPLO_CONFIG")]
    pub config: Option<PathBuf>,

    /// Set a cache path.
    #[clap(long)]
    pub cache_path: Option<PathBuf>,

    /// Do not search for a configuration file.
    #[clap(long)]
    pub no_auto_config: bool,
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
