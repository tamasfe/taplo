# args.LspCommand

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-cli\src\args.rs` (lines 125-131)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct LspCommand {
    #[clap(flatten)]
    pub general: GeneralArgs,

    #[clap(subcommand)]
    pub io: LspCommandIo,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct LspCommand {
    #[clap(flatten)]
    pub general: GeneralArgs,

    #[clap(subcommand)]
    pub io: LspCommandIo,
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
