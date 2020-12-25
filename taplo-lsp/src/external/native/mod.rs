use futures::Future;
use std::{path::Path, time::UNIX_EPOCH};

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

pub(crate) async fn write_file(p: &str, data: &[u8]) -> Result<(), anyhow::Error> {
    Ok(tokio::fs::write(p, data).await?)
}

pub(crate) fn file_exists(p: &str) -> bool {
    Path::new(p).exists()
}

pub(crate) fn mkdir(p: &str) -> Result<(), anyhow::Error> {
    std::fs::create_dir_all(p)?;
    Ok(())
}

pub(crate) fn needs_update(p: &str, new_date_ms: u64) -> Result<bool, anyhow::Error> {
    Ok(std::fs::metadata(p)?
        .modified()?
        .duration_since(UNIX_EPOCH)?
        .as_millis()
        < new_date_ms as u128)
}
