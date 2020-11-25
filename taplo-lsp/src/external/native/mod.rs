use futures::Future;
use std::path::Path;

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
