use crate::{
    rpc::{self, Message, Request},
    Server,
};
use anyhow::anyhow;
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    SinkExt, Stream, StreamExt,
};
use lsp_types::NumberOrString;
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader},
    select,
};

#[cfg(feature = "tokio-stdio")]
mod stdio;

#[cfg(feature = "tokio-tcp")]
mod tcp;

impl<W: Clone + 'static> Server<W> {
    pub(crate) async fn listen_loop(
        self,
        world: W,
        mut input: UnboundedReceiver<Message>,
        output: UnboundedSender<Message>,
        mut shutdown_signals: impl Stream<Item = ()> + Unpin,
    ) -> Result<(), anyhow::Error> {
        let ls = tokio::task::LocalSet::new();

        ls.run_until(async move {
            'l: loop {
                tokio::select! {
                    _ = shutdown_signals.next() => {
                        tracing::info!("shutdown signal received, shutting down...");

                        let task_fut = self.handle_message(
                            world.clone(),
                            Request::<()>::new()
                                .with_id(Some(NumberOrString::String(
                                    "external_shutdown".to_string(),
                                )))
                                .with_method("shutdown")
                                .into_message(),
                            output.clone().sink_map_err(|e| panic!("{}", e)),
                        );

                        select!{
                            _ = shutdown_signals.next() => {
                                tracing::warn!("shut down forcibly");
                            },
                            _ = task_fut => {}
                        }

                        drop(output);
                        drop(input);
                        break 'l;
                    },
                    msg = input.next() => {
                        match msg {
                            Some(msg) => {
                                if msg.method.as_ref().map(|m| m == "exit").unwrap_or(false) {
                                    drop(output);
                                    drop(input);
                                    break;
                                }

                                let task_fut = self.handle_message(
                                    world.clone(),
                                    msg,
                                    output.clone().sink_map_err(|e| panic!("{}", e)),
                                );

                                tokio::task::spawn_local(async move {
                                    if let Err(e) = task_fut.await {
                                        tracing::error!(error = %e, "handler returned error");
                                    }
                                });
                            }
                            None => break 'l,
                        }
                    }
                };
            }

            if self.is_shutting_down().await {
                Ok(())
            } else {
                Err(anyhow!("got exit message without shutdown notice"))
            }
        })
        .await?;

        Ok(())
    }
}

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

pub(crate) async fn write_message<T: AsyncWrite + Unpin>(
    out: &mut T,
    msg: rpc::Message,
) -> anyhow::Result<()> {
    let msg = serde_json::to_vec(&msg)?;

    out.write_all(format!("Content-Length: {}\r\n\r\n", msg.len()).as_bytes())
        .await?;
    out.write_all(&msg).await?;
    out.flush().await?;

    Ok(())
}

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

pub(crate) async fn read_message<R: tokio::io::AsyncBufRead + Unpin>(
    mut input: R,
) -> Result<Option<rpc::Message>, anyhow::Error> {
    let mut size = 0;
    let mut buf = String::new();

    // Parse headers
    loop {
        buf.clear();

        if input.read_line(&mut buf).await? == 0 {
            return Ok(None);
        };

        if !buf.ends_with("\r\n") {
            return Err(anyhow!("malformed header: {:?}", buf));
        }

        let buf = &buf[..buf.len() - 2];
        if buf.is_empty() {
            break;
        }
        let mut parts = buf.splitn(2, ": ");
        let header_name = parts
            .next()
            .ok_or_else(|| anyhow!("invalid header: {}", buf))?;
        let header_value = match parts.next() {
            Some(h) => h,
            None => {
                return Err(anyhow!("malformed header: {:?}", buf));
            }
        };
        if header_name == "Content-Length" {
            size = match header_value.parse::<usize>() {
                Ok(s) => s,
                Err(err) => {
                    return Err(anyhow!("invalid content-length: {}", err));
                }
            }
        }
    }

    // Parse the message itself.
    let mut buf = buf.into_bytes();
    buf.resize(size, 0);

    input.read_exact(&mut buf).await?;

    Ok(Some(serde_json::from_slice(&buf)?))
}
