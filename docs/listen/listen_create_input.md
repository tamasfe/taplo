# listen.create_input

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\listen.rs` (lines 134-160)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) fn create_input(
    stream: impl AsyncRead + Unpin + Send + 'static,
) -> UnboundedReceiver<rpc::Message> {
    let (sender, receiver) = unbounded::<rpc::Message>();

    tokio::spawn(async move {
        let mut stream = BufReader::new(stream);

        loop {
            match read_message(&mut stream).await {
                Ok(msg) => match msg {
                    Some(msg) => {
                        if sender.unbounded_send(msg).is_err() {
                            return;
                        }
                    }
                    None => return,
                },
                Err(err) => {
                    tracing::error!(error = %err, "error reading message");
                }
            }
        }
    });

    receiver
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) fn create_input(
    stream: impl AsyncRead + Unpin + Send + 'static,
) -> UnboundedReceiver<rpc::Message> {
    let (sender, receiver) = unbounded::<rpc::Message>();

    tokio::spawn(async move {
        let mut stream = BufReader::new(stream);

        loop {
            match read_message(&mut stream).await {
                Ok(msg) => match msg {
                    Some(msg) => {
                        if sender.unbounded_send(msg).is_err() {
                            return;
                        }
                    }
                    None => return,
                },
                Err(err) => {
                    tracing::error!(error = %err, "error reading message");
                }
            }
        }
    });

    receiver
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
