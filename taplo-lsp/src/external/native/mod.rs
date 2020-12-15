use futures::Future;
use lsp_async_stub::Context;
use notify::{RecursiveMode, watcher, Watcher};
use std::{path::Path, sync::mpsc::channel, thread, time::Duration};

use crate::World;

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        eprintln!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        eprintln!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        eprintln!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {  $crate::log_info!($($arg)*)  }
    };
}

pub(crate) fn spawn<F: Future<Output = ()> + Send + 'static>(fut: F) {
    tokio::spawn(fut);
}

pub(crate) fn is_absolute_path(p: &str) -> bool {
    Path::new(p).is_absolute()
}

pub(crate) async fn read_file(p: &str) -> Result<Vec<u8>, anyhow::Error> {
    Ok(tokio::fs::read(p).await?)
}

pub(crate) fn file_exists(p: &str) -> bool {
    Path::new(p).exists()
}

pub(crate) fn watch_config(path: &Path, context: Context<World>) {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(2)).unwrap();

    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(ev) => {
                    spawn(async{
                        context.world().lock().await;
                        // TODO
                    });
                }
                Err(e) => {
                    log_debug!("watch error: {}", e)
                }
            }
        }
    });
}

pub(crate) fn unwatch_config() {}