#![recursion_limit = "256"]

use anyhow::anyhow;
use clap::{App, AppSettings, Arg};
use futures::{
    channel::mpsc::unbounded,
    channel::mpsc::{UnboundedReceiver, UnboundedSender},
};
use futures::{SinkExt, StreamExt};
use lsp_async_stub::rpc;
use once_cell::sync::OnceCell;
use std::{
    io::BufReader,
    process::exit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    unimplemented,
};
use taplo_lsp::{log_debug, log_error, log_info};
use tokio::{prelude::*, select, sync::broadcast};

static CTRL_C_PRESSED: AtomicBool = AtomicBool::new(false);

// Manual shutdown message for handling CTRL+C and other scenarios.
static SHUTDOWN_CHAN: OnceCell<broadcast::Sender<rpc::Message>> = OnceCell::new();

fn main() {
    let app = App::new("Taplo LSP")
        .author("kkiyama117 (https://github.com/kkiyama117), tamasfe (https://github.com/tamasfe)")
        .bin_name("taplo-lsp")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A language server for TOML")
        .long_about("A language server TOML (https://github.com/tamasfe/taplo).")
        .subcommand(
            App::new("run").about("Runs the server using the standard i/o for communication"),
        )
        .subcommand(
            App::new("listen")
                .about("Runs the server using a TCP socket for communication")
                .arg(
                    Arg::new("address")
                        .about("The address to listen on")
                        .default_value("localhost"),
                )
                .arg(Arg::new("port").about("Port to use").default_value("5000")),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp);

    let matches = app.get_matches();

    let (ctrlc_send, _) = broadcast::channel(1);
    SHUTDOWN_CHAN.set(ctrlc_send).unwrap();

    ctrlc::set_handler(|| {
        if CTRL_C_PRESSED.load(Ordering::SeqCst) {
            exit(1)
        }

        log_info!("shutting down gracefully... (press CTRL+C again to force)");
        CTRL_C_PRESSED.store(true, Ordering::SeqCst);
        SHUTDOWN_CHAN
            .get()
            .unwrap()
            .send(
                rpc::Request::<()>::new()
                    .with_method("shutdown")
                    .into_message(),
            )
            .unwrap();
    })
    .unwrap();

    match matches.subcommand() {
        Some(("run", _opts)) => {
            exit(run_lsp());
        }
        Some(("listen", _opts)) => unimplemented!(),
        _ => unreachable!(),
    }
}

fn run_lsp() -> i32 {
    let rt = Arc::new(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap(),
    );

    let server = taplo_lsp::create_server();
    let world = taplo_lsp::create_world();

    let (messages_sender, mut messages_chan) = unbounded::<rpc::Message>();
    let (msg_send, msg_recv) = unbounded::<rpc::Message>();

    stdin_background(messages_sender, rt.clone());
    let stdout_task = rt.spawn(write_stdout(msg_recv));

    let exit_code = rt.block_on(async move {
        let mut shutdown_chan = SHUTDOWN_CHAN.get().unwrap().subscribe();

        log_info!("processing messages from stdin...");

        loop {
            select! {
                shutdown_msg = shutdown_chan.recv() => {
                    if let Err(e) = server.handle_message(
                        world.clone(),
                        shutdown_msg.unwrap(),
                        msg_send.clone().sink_map_err(|e| panic!(e))
                    ).await {
                        log_error!("{}", e);
                    };
                    break;
                }
                msg = messages_chan.next() => {
                    match msg {
                        Some(msg) => {
                            if msg.method.as_ref().map(|m| m == "exit").unwrap_or(false) {
                                break;
                            }

                            let res = tokio::spawn(server.handle_message(
                                world.clone(),
                                msg,
                                msg_send.clone().sink_map_err(|e| panic!(e)),
                            ))
                            .await
                            .unwrap();

                            if let Err(e) = res {
                                log_error!("{}", e);
                            }
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

    if let Err(err) = rt.block_on(stdout_task) {
        log_debug!("{}", err)
    };

    exit_code
}

async fn write_message<T: AsyncWrite + Unpin>(
    out: &mut T,
    msg: rpc::Message,
) -> anyhow::Result<()> {
    let msg = serde_json::to_vec(&msg)?;

    out.write_all(format!("Content-Length: {}\r\n\r\n", msg.len()).as_bytes())
        .await?;
    out.write_all(&msg).await?;
    out.write_all("\r\n".as_bytes()).await?;

    out.flush().await?;

    Ok(())
}

/// This future will continuously write messages to stdout
/// until the channel is alive.
async fn write_stdout(mut messages: UnboundedReceiver<rpc::Message>) {
    let mut out = io::stdout();

    while let Some(message) = messages.next().await {
        if let Err(err) = write_message(&mut out, message).await {
            log_error!("{}", err)
        };
    }
}

fn read_message_sync<R: std::io::BufRead>(
    mut input: R,
) -> Result<Option<rpc::Message>, anyhow::Error> {
    let mut size = 0;
    let mut buf = String::new();

    // Parse headers
    loop {
        buf.clear();

        if input.read_line(&mut buf).unwrap() == 0 {
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
        let header_name = parts.next().unwrap();
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

fn stdin_background(mut messages: UnboundedSender<rpc::Message>, rt: Arc<tokio::runtime::Runtime>) {
    std::thread::spawn(move || {
        let mut stdin = BufReader::new(std::io::stdin());

        loop {
            match read_message_sync(&mut stdin) {
                Ok(msg) => match msg {
                    Some(msg) => {
                        rt.block_on(messages.send(msg)).unwrap();
                        rt.block_on(messages.flush()).unwrap();
                    }
                    None => return,
                },
                Err(err) => {
                    log_error!("input error: {}", err);
                }
            }
        }
    });
}
