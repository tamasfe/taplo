# args.GetCommand

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-cli\src\args.rs` (lines 185-245)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct GetCommand {
    /// The format specifying how the output is printed.
    ///
    /// All newlines are in the output are LF.
    ///
    /// Format-specific remarks:
    ///
    /// --- value:
    ///
    /// If the value is a string, all surrounding quotes will be stripped and
    /// all escape sequences will be unescaped.
    ///
    /// If the value is an integer or float, it will be output in a decimal format without any rounding.
    ///
    /// If the value is an array, all of its items will be output on separate lines.
    ///
    /// If the value is a table or an array that contains tables, the operation will fail.
    ///
    /// --- toml:
    ///
    /// Comments and formatting will not be preserved.
    /// It is possible to select arrays and individual values that are not tables,
    /// in this case the output will not be a valid TOML document.
    #[clap(short, long, value_enum, default_value = "value")]
    pub output_format: OutputFormat,

    /// Strip the trailing newline from the output.
    ///
    /// If this is not provided, all output will end with a line-feed character.
    #[clap(short, long)]
    pub strip_newline: bool,

    /// Path to the TOML document, if omitted the standard input will be used.
    #[clap(short, long)]
    pub file_path: Option<PathBuf>,

    /// A dotted key pattern to the value within the TOML document.
    ///
    /// If omitted, the entire document will be printed.
    ///
    /// If the pattern yielded no values, the operation will fail.
    ///
    /// The pattern supports `jq`-like syntax and glob patterns as well:
    ///
    /// Examples:
    ///
    /// - table.array[1].foo
    /// - table.array.1.foo
    /// - table.array[*].foo
    /// - table.array.*.foo
    /// - dependencies.tokio-*.version
    ///
    pub pattern: Option<String>,

    /// A string that separates array values when printing to stdout.
    ///
    /// If `--separator` is specified with a non text output format,
    /// the operation will fail.
    #[clap(long)]
    pub separator: Option<String>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct GetCommand {
    /// The format specifying how the output is printed.
    ///
    /// All newlines are in the output are LF.
    ///
    /// Format-specific remarks:
    ///
    /// --- value:
    ///
    /// If the value is a string, all surrounding quotes will be stripped and
    /// all escape sequences will be unescaped.
    ///
    /// If the value is an integer or float, it will be output in a decimal format without any rounding.
    ///
    /// If the value is an array, all of its items will be output on separate lines.
    ///
    /// If the value is a table or an array that contains tables, the operation will fail.
    ///
    /// --- toml:
    ///
    /// Comments and formatting will not be preserved.
    /// It is possible to select arrays and individual values that are not tables,
    /// in this case the output will not be a valid TOML document.
    #[clap(short, long, value_enum, default_value = "value")]
    pub output_format: OutputFormat,

    /// Strip the trailing newline from the output.
    ///
    /// If this is not provided, all output will end with a line-feed character.
    #[clap(short, long)]
    pub strip_newline: bool,

    /// Path to the TOML document, if omitted the standard input will be used.
    #[clap(short, long)]
    pub file_path: Option<PathBuf>,

    /// A dotted key pattern to the value within the TOML document.
    ///
    /// If omitted, the entire document will be printed.
    ///
    /// If the pattern yielded no values, the operation will fail.
    ///
    /// The pattern supports `jq`-like syntax and glob patterns as well:
    ///
    /// Examples:
    ///
    /// - table.array[1].foo
    /// - table.array.1.foo
    /// - table.array[*].foo
    /// - table.array.*.foo
    /// - dependencies.tokio-*.version
    ///
    pub pattern: Option<String>,

    /// A string that separates array values when printing to stdout.
    ///
    /// If `--separator` is specified with a non text output format,
    /// the operation will fail.
    #[clap(long)]
    pub separator: Option<String>,
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
