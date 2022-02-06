use taplo_common::environment::{Environment, native::NativeEnvironment};

use crate::{args::LspCommand, Taplo};

impl<E: Environment> Taplo<E> {
    pub async fn execute_lsp(&self, cmd: LspCommand) -> Result<(), anyhow::Error> {
        match cmd {
            LspCommand::Tcp { address } => {
                let server = taplo_lsp::create_server();
                let world = taplo_lsp::create_world(NativeEnvironment);
                server
                    .listen_tcp(
                        world,
                        &address,
                        async_ctrlc::CtrlC::new().unwrap(),
                    )
                    .await
            }
            LspCommand::Stdio {} => {
                let server = taplo_lsp::create_server();
                let world = taplo_lsp::create_world(NativeEnvironment);
                server
                    .listen_stdio(world, async_ctrlc::CtrlC::new().unwrap())
                    .await
            }
        }
    }
}
