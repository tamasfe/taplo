# stdio.listen_stdio

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\listen\stdio.rs` (lines 8-20)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
