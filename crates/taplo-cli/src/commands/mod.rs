use taplo_common::environment::Environment;

use crate::{
    args::{TaploArgs, TaploCommand},
    Taplo,
};

mod config;
mod format;
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
            TaploCommand::Format(fmt) => self.execute_format(fmt).await,
            TaploCommand::Lsp { cmd } => {
                #[cfg(feature = "lsp")]
                {
                    self.execute_lsp(cmd).await
                }
                #[cfg(not(feature = "lsp"))]
                {
                    let _ = cmd;
                    Err(anyhow::anyhow!("the LSP is not part of this build, please consult the documentation about enabling the functionality"))
                }
            }

            #[cfg(feature = "toml-test")]
            TaploCommand::TomlTest {} => self.execute_toml_test().await,
            TaploCommand::Lint(cmd) => self.execute_lint(cmd).await,
            TaploCommand::Config { cmd } => self.execute_config(cmd).await,
            TaploCommand::Get(cmd) => self.execute_get(cmd).await,
        }
    }
}
