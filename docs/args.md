# Module: args

**File:** `\crates\taplo-cli\src\args.rs`

## Overview

Module `args` contains 6 analyzed entities.

```
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

    /// Start a
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `args.TaploArgs` | struct | No description |
| `args.FormatCommand` | struct | No description |
| `args.LintCommand` | struct | No description |
| `args.LspCommand` | struct | No description |
| `args.GeneralArgs` | struct | No description |
| `args.GetCommand` | struct | No description |

## Git History

- Commits: 14
- Authors: 7
- Bug fixes: 1
- Features: 6

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
