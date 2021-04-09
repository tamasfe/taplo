use anyhow::anyhow;
use futures::{
    channel::mpsc::unbounded,
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
};
use futures::{SinkExt, StreamExt};
use lsp_async_stub::{rpc, Server};
use std::{io::BufReader, sync::Arc};
use taplo_lsp::{log_error, log_info, World};
use tokio::{runtime::Runtime, task::JoinHandle};

use crate::{common::write_message, is_shutting_down, shutdown, SHUTDOWN_CHAN};

pub(crate) fn run(rt: Arc<Runtime>, server: Server<World>, world: World) -> i32 {
    let mut input = create_input(rt.clone());
    let (output, output_handle) = create_output(rt.clone());

    let exit_code = rt.block_on(async move {
        let mut shutdown_chan = SHUTDOWN_CHAN.get().unwrap().subscribe();

        log_info!("processing messages from stdin...");

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

        if server.is_shutting_down().await {
            0
        } else {
            1
        }
    });

    let _ = rt.block_on(output_handle);

    exit_code
}

pub(crate) fn create_input(rt: Arc<Runtime>) -> UnboundedReceiver<rpc::Message> {
    let (mut sender, receiver) = unbounded::<rpc::Message>();

    std::thread::spawn(move || {
        let mut stdin = BufReader::new(std::io::stdin());

        loop {
            match read_message(&mut stdin) {
                Ok(msg) => match msg {
                    Some(msg) => {
                        rt.block_on(sender.send(msg)).unwrap();
                        rt.block_on(sender.flush()).unwrap();
                    }
                    None => return,
                },
                Err(err) => {
                    if !is_shutting_down() {
                        log_error!("failed to read message: {}", err);
                    }
                }
            }
        }
    });

    receiver
}

pub(crate) fn create_output(rt: Arc<Runtime>) -> (UnboundedSender<rpc::Message>, JoinHandle<()>) {
    let (sender, mut receiver) = unbounded::<rpc::Message>();
    let handle = rt.spawn(async move {
        let mut out = tokio::io::stdout();

        while let Some(message) = receiver.next().await {
            if let Err(err) = write_message(&mut out, message).await {
                if !is_shutting_down() {
                    log_error!("{}", err)
                }
            };
        }
    });
    (sender, handle)
}

fn read_message<R: std::io::BufRead>(mut input: R) -> Result<Option<rpc::Message>, anyhow::Error> {
    let mut size = 0;
    let mut buf = String::new();

    // Parse headers
    loop {
        buf.clear();

        if input.read_line(&mut buf)? == 0 {
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

    input.read_exact(&mut buf)?;

    Ok(Some(serde_json::from_slice(&buf)?))
}
