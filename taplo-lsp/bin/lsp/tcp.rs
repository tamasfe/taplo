use anyhow::anyhow;
use futures::{
    channel::mpsc::unbounded,
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
};
use futures::{SinkExt, StreamExt};
use lsp_async_stub::{rpc, Server};
use std::sync::Arc;
use taplo_lsp::{log_error, log_info, World};
use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite},
    net::TcpListener,
    runtime::Runtime,
    task::JoinHandle,
};

use crate::{common::write_message, is_shutting_down, shutdown, SHUTDOWN_CHAN};

pub(crate) fn run(
    rt: Arc<Runtime>,
    server: Server<World>,
    world: World,
    addr: &str,
    port: usize,
) -> i32 {
    let address = format!("{}:{}", addr, port);

    rt.block_on(async {
        let listener = match TcpListener::bind(&address).await {
            Ok(l) => l,
            Err(err) => {
                log_error!("failed to listen: {}", err);
                return 1;
            }
        };

        let mut shutdown_chan = SHUTDOWN_CHAN.get().unwrap().subscribe();

        log_info!("waiting for TCP client on {}...", address);

        let (tcp_stream, tcp_addr) = tokio::select! {
            _ = shutdown_chan.recv() => {
                return 0;
            }
            conn = listener.accept() => {
                conn.unwrap()
            }
        };

        log_info!("client connected ({}).", &tcp_addr);

        let (tcp_read, tcp_write) = tcp_stream.into_split();

        let (mut input, input_handle) = create_input(rt.clone(), tcp_read);
        let (output, output_handle) = create_output(rt.clone(), tcp_write);

        loop {
            tokio::select! {
                shutdown_msg = shutdown_chan.recv() => {
                    if let Err(e) = server.handle_message(
                        world.clone(),
                        shutdown_msg.unwrap(),
                        output.clone().sink_map_err(|e| panic!("{}", e))
                    ).await {
                        log_error!("{}", e);
                    };
                    break;
                }
                msg = input.next() => {
                    match msg {
                        Some(msg) => {
                            if msg.method.as_ref().map(|m| m == "exit").unwrap_or(false) {
                                break;
                            } else if msg.method.as_ref().map(|m| m == "shutdown").unwrap_or(false) {
                                // We broadcast it so that every task will know that we're shutting down.
                                log_info!("received shutdown request.");
                                shutdown(msg);
                                continue;
                            }

                            let task_fut = server.handle_message(
                                world.clone(),
                                msg,
                                output.clone().sink_map_err(|e| panic!("{}", e)),
                            );

                            tokio::spawn(async move {
                                if let Err(e) = task_fut.await {
                                    log_error!("{}", e);
                                }
                            });

                        }
                        None => break,
                    }
                }
            };
        }

        drop(output);
        drop(listener);

        let _ = input_handle.await;
        let _ = output_handle.await;

        if server.is_shutting_down().await {
            0
        } else {
            1
        }
    })
}

pub(crate) fn create_input(
    rt: Arc<Runtime>,
    stream: impl AsyncRead + Unpin + Send + 'static,
) -> (UnboundedReceiver<rpc::Message>, JoinHandle<()>) {
    let (mut sender, receiver) = unbounded::<rpc::Message>();

    let handle = rt.spawn(async move {
        let mut shutdown_chan = SHUTDOWN_CHAN.get().unwrap().subscribe();
        let mut stream = tokio::io::BufReader::new(stream);

        loop {
            tokio::select! {
                _ = shutdown_chan.recv() => break,
                msg = read_message(&mut stream) => {
                    match msg {
                        Ok(msg) => {
                            match msg {
                                Some(msg) => {
                                    sender.send(msg).await.unwrap();
                                    sender.flush().await.unwrap();
                                }
                                None => break,
                            }
                        }
                        Err(err) => {
                            if !is_shutting_down() {
                                log_error!("failed to read message: {}", err);
                            }
                        }
                    }
                }
            }
        }
    });

    (receiver, handle)
}

pub(crate) fn create_output(
    rt: Arc<Runtime>,
    mut stream: impl AsyncWrite + Send + Unpin + 'static,
) -> (UnboundedSender<rpc::Message>, JoinHandle<()>) {
    let (sender, mut receiver) = unbounded::<rpc::Message>();
    let handle = rt.spawn(async move {
        while let Some(message) = receiver.next().await {
            if let Err(err) = write_message(&mut stream, message).await {
                if !is_shutting_down() {
                    log_error!("failed to send message: {}", err)
                }
            };
        }
    });
    (sender, handle)
}

async fn read_message<R: AsyncBufRead + Unpin>(
    mut input: R,
) -> Result<Option<rpc::Message>, anyhow::Error> {
    let mut size = 0;
    let mut buf = String::new();
    let mut shutdown_chan = SHUTDOWN_CHAN.get().unwrap().subscribe();

    // Parse headers
    loop {
        buf.clear();

        tokio::select! {
            _ = shutdown_chan.recv() => return Ok(None),
            count = input.read_line(&mut buf) => {
                if count? == 0 {
                    return Ok(None);

                }
            }
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
                Ok(s) => s + 2, // For "\r\n" at the end
                Err(err) => {
                    return Err(anyhow!("invalid content-length: {}", err));
                }
            }
        }
    }

    // Parse the message itself.
    let mut buf = buf.into_bytes();
    buf.resize(size, 0);

    tokio::select! {
        _ = shutdown_chan.recv() => return Ok(None),
        ok = input.read_exact(&mut buf) => ok?
    };

    Ok(Some(serde_json::from_slice(&buf)?))
}
