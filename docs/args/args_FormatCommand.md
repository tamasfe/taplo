# args.FormatCommand

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-cli\src\args.rs` (lines 88-121)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct FormatCommand {
    #[clap(flatten)]
    pub general: GeneralArgs,

    /// A formatter option given as a "key=value", can be set multiple times.
    ///
    /// The valid options and values are available here: https://taplo.tamasfe.dev/configuration/formatter-options.html
    #[clap(long = "option", short)]
    pub options: Vec<String>,

    /// Ignore syntax errors and force formatting.
    #[clap(long, short)]
    pub force: bool,

    /// Dry-run and report any files that are not correctly formatted.
    #[clap(long)]
    pub check: bool,

    /// Print the differences in patch formatting to `stdout`
    #[clap(long)]
    pub diff: bool,

    /// Paths or glob patterns to TOML documents.
    ///
    /// If the only argument is "-", the standard input will be used.
    pub files: Vec<String>,

    /// A path to the file that the Taplo CLI will treat like stdin.
    ///
    /// This option does not change the file input source. This option should be used only when the
    /// source input arises from the stdin.
    #[clap(long)]
    pub stdin_filepath: Option<String>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct FormatCommand {
    #[clap(flatten)]
    pub general: GeneralArgs,

    /// A formatter option given as a "key=value", can be set multiple times.
    ///
    /// The valid options and values are available here: https://taplo.tamasfe.dev/configuration/formatter-options.html
    #[clap(long = "option", short)]
    pub options: Vec<String>,

    /// Ignore syntax errors and force formatting.
    #[clap(long, short)]
    pub force: bool,

    /// Dry-run and report any files that are not correctly formatted.
    #[clap(long)]
    pub check: bool,

    /// Print the differences in patch formatting to `stdout`
    #[clap(long)]
    pub diff: bool,

    /// Paths or glob patterns to TOML documents.
    ///
    /// If the only argument is "-", the standard input will be used.
    pub files: Vec<String>,

    /// A path to the file that the Taplo CLI will treat like stdin.
    ///
    /// This option does not change the file input source. This option should be used only when the
    /// source input arises from the stdin.
    #[clap(long)]
    pub stdin_filepath: Option<String>,
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
