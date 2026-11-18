# args.TaploArgs

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-cli\src\args.rs` (lines 10-21)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct TaploArgs {
    #[clap(long, value_enum, global = true, default_value = "auto")]
    pub colors: Colors,
    /// Enable a verbose logging format.
    #[clap(long, global = true)]
    pub verbose: bool,
    /// Enable logging spans.
    #[clap(long, global = true)]
    pub log_spans: bool,
    #[clap(subcommand)]
    pub cmd: TaploCommand,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct TaploArgs {
    #[clap(long, value_enum, global = true, default_value = "auto")]
    pub colors: Colors,
    /// Enable a verbose logging format.
    #[clap(long, global = true)]
    pub verbose: bool,
    /// Enable logging spans.
    #[clap(long, global = true)]
    pub log_spans: bool,
    #[clap(subcommand)]
    pub cmd: TaploCommand,
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
