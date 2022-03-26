use std::sync::Arc;

use taplo_common::environment::{native::NativeEnvironment, Environment};

use crate::{args::LspCommand, default_config, Taplo};

impl<E: Environment> Taplo<E> {
    pub async fn execute_lsp(&self, cmd: LspCommand) -> Result<(), anyhow::Error> {
        let server = taplo_lsp::create_server();
        let world = taplo_lsp::create_world(NativeEnvironment::new());
        world.set_default_config(Arc::new(default_config()));

        match cmd {
            LspCommand::Tcp { address } => {
                server
                    .listen_tcp(world, &address, async_ctrlc::CtrlC::new().unwrap())
                    .await
            }
            LspCommand::Stdio {} => {
                server
                    .listen_stdio(world, async_ctrlc::CtrlC::new().unwrap())
                    .await
            }
        }
    }
}
