use taplo_common::environment::Environment;

use crate::{
    args::{TaploArgs, TaploCommand},
    Taplo,
};

mod format;
mod lint;
#[cfg(feature = "lsp")]
mod lsp;
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
            #[cfg(feature = "lsp")]
            TaploCommand::Lsp { cmd } => self.execute_lsp(cmd).await,
            TaploCommand::TomlTest {} => self.execute_toml_test().await,
            TaploCommand::Lint(cmd) => self.execute_lint(cmd).await,
        }
    }
}
