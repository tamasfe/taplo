use taplo_common::environment::{native::NativeEnvironment, Environment};

use crate::{
    args::{LspCommand, LspCommandIo},
    Taplo,
};

impl<E: Environment> Taplo<E> {
    pub async fn execute_lsp(&mut self, cmd: LspCommand) -> Result<(), anyhow::Error> {
        self.schemas
            .cache()
            .set_cache_path(cmd.general.cache_path.clone());

        let config = self.load_config(&cmd.general).await?;

        let server = taplo_lsp::create_server();
        let world = taplo_lsp::create_world(NativeEnvironment::new());
        world.set_default_config(config);

        match cmd.io {
            LspCommandIo::Tcp { address } => {
                server
                    .listen_tcp(world, &address, async_ctrlc::CtrlC::new().unwrap())
                    .await
            }
            LspCommandIo::Stdio {} => {
                server
                    .listen_stdio(world, async_ctrlc::CtrlC::new().unwrap())
                    .await
            }
        }
    }
}
