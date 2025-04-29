use clap::{crate_version, Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
#[cfg(feature = "lint")]
use url::Url;

#[derive(Clone, Parser)]
#[clap(name = "taplo")]
#[clap(bin_name = "taplo")]
#[clap(version = crate_version!())]
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

#[derive(Clone, Args)]
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

#[derive(Clone, Copy, ValueEnum)]
pub enum Colors {
    /// Determine whether to colorize output automatically.
    Auto,
    /// Always colorize output.
    Always,
    /// Never colorize output.
    Never,
}

#[derive(Clone, Subcommand)]
pub enum TaploCommand {
    /// Lint TOML documents.
    #[clap(visible_aliases = &["check", "validate"])]
    #[cfg(feature = "lint")]
    Lint(LintCommand),

    /// Format TOML documents.
    ///
    /// Files are modified in-place unless the input comes from the standard input, in which case the formatted result is printed to the standard output.
    #[clap(visible_aliases = &["fmt"])]
    Format(FormatCommand),

    /// Language server operations.
    #[cfg(feature = "lsp")]
    Lsp {
        #[clap(flatten)]
        cmd: LspCommand,
    },

    /// Operations with the Taplo config file.
    #[clap(visible_aliases = &["cfg"])]
    Config {
        #[clap(subcommand)]
        cmd: ConfigCommand,
    },

    /// Extract a value from the given TOML document.
    Get(GetCommand),

    /// Start a decoder for `toml-test` (https://github.com/BurntSushi/toml-test).
    #[cfg(feature = "toml-test")]
    TomlTest {},

    /// Generate completions for Taplo CLI
    #[cfg(feature = "completions")]
    Completions { shell: String },
}

#[derive(Clone, Args)]
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

#[cfg(feature = "lsp")]
#[derive(Clone, Args)]
pub struct LspCommand {
    #[clap(flatten)]
    pub general: GeneralArgs,

    #[clap(subcommand)]
    pub io: LspCommandIo,
}

#[cfg(feature = "lsp")]
#[derive(Clone, Subcommand)]
pub enum LspCommandIo {
    /// Run the language server and listen on a TCP address.
    Tcp {
        /// The address to listen on.
        #[clap(long, default_value = "0.0.0.0:9181")]
        address: String,
    },
    /// Run the language server over the standard input and output.
    Stdio {},
}

#[derive(Clone, Subcommand)]
pub enum ConfigCommand {
    /// Print the default `.taplo.toml` configuration file.
    Default,
    /// Print the JSON schema of the `.taplo.toml` configuration file.
    Schema,
}

#[cfg(feature = "lint")]
#[derive(Clone, Args)]
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

#[derive(Clone, Args)]
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

#[derive(Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    /// Extract the value outputting it in a text format.
    Value,
    /// Output format in JSON.
    Json,
    /// Output format in TOML.
    Toml,
}
