# listen.create_output

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\listen.rs` (lines 99-118)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) fn create_output(
    sink: impl AsyncWrite + Unpin + Send + 'static,
) -> UnboundedSender<rpc::Message> {
    let (sender, mut receiver) = unbounded::<rpc::Message>();
    tokio::spawn(async move {
        let mut out = sink;

        while let Some(message) = receiver.next().await {
            let method = message.method.clone();
            let id = message.id.clone();

            if let Err(err) = write_message(&mut out, message).await {
                tracing::error!(error = %err, "error writing message");
            };

            tracing::debug!(?method, ?id, "message written");
        }
    });
    sender
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) fn create_output(
    sink: impl AsyncWrite + Unpin + Send + 'static,
) -> UnboundedSender<rpc::Message> {
    let (sender, mut receiver) = unbounded::<rpc::Message>();
    tokio::spawn(async move {
        let mut out = sink;

        while let Some(message) = receiver.next().await {
            let method = message.method.clone();
            let id = message.id.clone();

            if let Err(err) = write_message(&mut out, message).await {
                tracing::error!(error = %err, "error writing message");
            };

            tracing::debug!(?method, ?id, "message written");
        }
    });
    sender
}
```

## Git History

- Commits: 3
- Authors: 3
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
