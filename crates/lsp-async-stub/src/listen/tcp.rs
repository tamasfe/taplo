use crate::{
    listen::{create_input, create_output},
    Server,
};
use futures::{Stream, StreamExt};
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    select,
};

impl<W: Clone + 'static> Server<W> {
    pub async fn listen_tcp<A>(
        self,
        world: W,
        address: A,
        mut shutdown_signals: impl Stream<Item = ()> + Unpin,
    ) -> Result<(), anyhow::Error>
    where
        A: ToSocketAddrs,
    {
        let listener = TcpListener::bind(&address).await?;

        let addr = listener.local_addr()?;
        tracing::info!(transport = "tcp", address = %addr, "LSP server listening");

        tracing::info!("waiting for client");

        let (conn_stream, client_address) = select! {
            l = listener.accept() => l?,
            _ = shutdown_signals.next() => {
                tracing::info!("shutting down...");
                return Ok(());
            }
        };

        tracing::info!(%client_address, "client connected");

        let (conn_read, conn_write) = conn_stream.into_split();

        let input = create_input(conn_read);
        let output = create_output(conn_write);

        self.listen_loop(world, input, output, shutdown_signals)
            .await
    }
}
