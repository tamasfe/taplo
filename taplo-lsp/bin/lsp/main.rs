#![recursion_limit = "256"]

use clap::{App, AppSettings, Arg};
use lsp_async_stub::rpc;
use once_cell::sync::OnceCell;
use std::{
    process::exit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use taplo_lsp::{log_error, log_info};
use tokio::sync::broadcast;

static CTRL_C_PRESSED: AtomicBool = AtomicBool::new(false);
static SHUTTING_DOWN: AtomicBool = AtomicBool::new(false);

// Manual shutdown message for handling CTRL+C and other scenarios.
static SHUTDOWN_CHAN: OnceCell<broadcast::Sender<rpc::Message>> = OnceCell::new();

fn shutdown(msg: rpc::Message) {
    SHUTTING_DOWN.store(true, Ordering::SeqCst);
    SHUTDOWN_CHAN.get().unwrap().send(msg).unwrap();
}

fn is_shutting_down() -> bool {
    SHUTTING_DOWN.load(Ordering::SeqCst)
}

mod common;
mod stdio;
mod tcp;

fn main() {
    let app = App::new("Taplo LSP")
        .author("kkiyama117 (https://github.com/kkiyama117)\ntamasfe (https://github.com/tamasfe)")
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
        shutdown(
            rpc::Request::<()>::new()
                .with_method("shutdown")
                .into_message(),
        );
    })
    .unwrap();

    match matches.subcommand() {
        Some(("run", _opts)) => {
            exit(run_lsp(IoKind::Stdio));
        }
        Some(("listen", opts)) => {
            let port: usize = match opts.value_of("port").unwrap().parse() {
                Ok(v) => v,
                Err(err) => {
                    log_error!("invalid port: {}", err);
                    exit(1);
                }
            };

            let addr = opts.value_of("address").unwrap();

            exit(run_lsp(IoKind::Tcp { addr, port }))
        }
        _ => unreachable!(),
    }
}

enum IoKind<'args> {
    Stdio,
    Tcp { addr: &'args str, port: usize },
}

// async fn create_output_chan(kind:)

fn run_lsp(kind: IoKind<'_>) -> i32 {
    let rt = Arc::new(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap(),
    );

    let server = taplo_lsp::create_server();
    let world = taplo_lsp::create_world();

    let exit_code = match kind {
        IoKind::Stdio => stdio::run(rt, server, world),
        IoKind::Tcp { addr, port } => tcp::run(rt, server, world, addr, port),
    };

    log_info!("exiting...");

    exit_code
}
