use taplo_common::environment::Environment;

use crate::{
    args::{TaploArgs, TaploCommand},
    Taplo,
};

mod config;
mod format;
#[cfg(feature = "lint")]
mod lint;
#[cfg(feature = "lsp")]
mod lsp;
mod queries;

#[cfg(feature = "toml-test")]
mod toml_test;

impl<E: Environment> Taplo<E> {
    pub async fn execute(&mut self, taplo: TaploArgs) -> Result<(), anyhow::Error> {
        self.colors = match taplo.colors {
            crate::args::Colors::Auto => self.env.atty_stderr(),
            crate::args::Colors::Always => true,
            crate::args::Colors::Never => false,
        };

        match taplo.cmd {
            #[cfg(feature = "completions")]
            TaploCommand::Completions { shell: shell_s } => {
                use anyhow::anyhow;
                use clap::CommandFactory;
                use clap_complete::{generate, shells::Shell};
                use std::{io::stdout, str::FromStr};

                let mut cmd = TaploArgs::command();
                let bin_name = TaploArgs::command()
                    .get_bin_name()
                    .unwrap_or("taplo")
                    .to_string();

                let s = shell_s.to_ascii_lowercase();

                match s.as_str() {
                    "bash" => {
                        generate(
                            clap_complete::Shell::Bash,
                            &mut cmd,
                            &bin_name,
                            &mut stdout(),
                        );
                        Ok(())
                    }
                    "zsh" | "zsh5" | "zsh6" => {
                        generate(
                            clap_complete::Shell::Zsh,
                            &mut cmd,
                            &bin_name,
                            &mut stdout(),
                        );
                        Ok(())
                    }
                    "fish" => {
                        generate(
                            clap_complete::Shell::Fish,
                            &mut cmd,
                            &bin_name,
                            &mut stdout(),
                        );
                        Ok(())
                    }
                    "powershell" | "pwsh" => {
                        generate(
                            clap_complete::Shell::PowerShell,
                            &mut cmd,
                            &bin_name,
                            &mut stdout(),
                        );
                        Ok(())
                    }
                    "elvish" => {
                        generate(
                            clap_complete::Shell::Elvish,
                            &mut cmd,
                            &bin_name,
                            &mut stdout(),
                        );
                        Ok(())
                    }

                    // new: Nushell and Fig
                    "nushell" | "nu" => {
                        use clap_complete_nushell::Nushell;
                        generate(Nushell, &mut cmd, &bin_name, &mut stdout());
                        Ok(())
                    }
                    "fig" => {
                        use clap_complete_fig::Fig;
                        generate(Fig, &mut cmd, &bin_name, &mut stdout());
                        Ok(())
                    }
                    other => Err(anyhow!(format!("{other} is not support"))),
                }
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
}
