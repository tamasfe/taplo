# mod.execute

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\mod.rs` (lines 20-54)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn execute(&mut self, taplo: TaploArgs) -> Result<(), anyhow::Error> {
        self.colors = match taplo.colors {
            crate::args::Colors::Auto => self.env.atty_stderr(),
            crate::args::Colors::Always => true,
            crate::args::Colors::Never => false,
        };

        match taplo.cmd {
            #[cfg(feature = "completions")]
            TaploCommand::Completions { shell } => {
                use anyhow::anyhow;
                use clap::CommandFactory;
                use clap_complete::{generate, shells::Shell};
                use std::{io::stdout, str::FromStr};

                let shell = Shell::from_str(&shell).map_err(|e| anyhow!(e))?;
                generate(
                    shell,
                    &mut TaploArgs::command(),
                    TaploArgs::command().get_bin_name().unwrap(),
                    &mut stdout(),
                );
                Ok(())
            }
            TaploCommand::Config { cmd } => self.execute_config(cmd).await,
            TaploCommand::Format(fmt) => self.execute_format(fmt).await,
            TaploCommand::Get(cmd) => self.execute_get(cmd).await,
            #[cfg(feature = "lint")]
            TaploCommand::Lint(cmd) => self.execute_lint(cmd).await,
            #[cfg(feature = "lsp")]
            TaploCommand::Lsp { cmd } => self.execute_lsp(cmd).await,
            #[cfg(feature = "toml-test")]
            TaploCommand::TomlTest {} => self.execute_toml_test().await,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub async fn execute(&mut self, taplo: TaploArgs) -> Result<(), anyhow::Error> {
        self.colors = match taplo.colors {
            crate::args::Colors::Auto => self.env.atty_stderr(),
            crate::args::Colors::Always => true,
            crate::args::Colors::Never => false,
        };

        match taplo.cmd {
            #[cfg(feature = "completions")]
            TaploCommand::Completions { shell } => {
                use anyhow::anyhow;
                use clap::CommandFactory;
                use clap_complete::{generate, shells::Shell};
                use std::{io::stdout, str::FromStr};

                let shell = Shell::from_str(&shell).map_err(|e| anyhow!(e))?;
                generate(
                    shell,
                    &mut TaploArgs::command(),
                    TaploArgs::command().get_bin_name().unwrap(),
                    &mut stdout(),
                );
                Ok(())
            }
            TaploCommand::Config { cmd } => self.execute_config(cmd).await,
            TaploCommand::Format(fmt) => self.execute_format(fmt).await,
            TaploCommand::Get(cmd) => self.execute_get(cmd).await,
            #[cfg(feature = "lint")]
            TaploCommand::Lint(cmd) => self.execute_lint(cmd).await,
            #[cfg(feature = "lsp")]
            TaploCommand::Lsp { cmd } => self.execute_lsp(cmd).await,
            #[cfg(feature = "toml-test")]
            TaploCommand::TomlTest {} => self.execute_toml_test().await,
        }
    }
```

## Git History

- Commits: 6
- Authors: 2
- Bug fixes: 2
- Features: 2

## Callers

None

## Callees

None
