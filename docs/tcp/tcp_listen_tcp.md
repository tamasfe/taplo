# tcp.listen_tcp

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\listen\tcp.rs` (lines 12-45)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
