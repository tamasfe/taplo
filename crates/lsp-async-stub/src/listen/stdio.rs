use crate::{
    listen::{create_input, create_output},
    Server,
};
use futures::Stream;

impl<W: Clone + 'static> Server<W> {
    pub async fn listen_stdio(
        self,
        world: W,
        shutdown_signals: impl Stream<Item = ()> + Unpin,
    ) -> Result<(), anyhow::Error> {
        let input = create_input(tokio::io::stdin());
        let output = create_output(tokio::io::stdout());

        tracing::info!(transport = "stdio", "LSP server listening");

        self.listen_loop(world, input, output, shutdown_signals)
            .await
    }
}
