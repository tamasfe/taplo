#![allow(unused_unsafe)]
use crate::config::{Config, CONFIG_FILE_NAMES};
use once_cell::sync::Lazy;
use pretty_lint::{colored::Colorize, Severity};
use std::{
    env,
    sync::atomic::{AtomicBool, Ordering},
};
use std::{
    io,
    path::{Path, PathBuf},
};
use wasm_bindgen::prelude::*;

static ERROR_STATUS: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

#[derive(Debug)]
pub struct GenericError(String);

impl core::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for GenericError {}

fn to_io_error(err: JsValue) -> io::Error {
    io::Error::new(
        io::ErrorKind::Other,
        Box::new(GenericError(format!("{:?}", err))),
    )
}

#[wasm_bindgen]
pub(crate) struct GlobPaths {
    pub(crate) paths: Vec<PathBuf>,
    pub(crate) errors: Vec<JsValue>,
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = global, js_name = writeStderr)]
    fn js_write_stderr(s: &str);

    #[wasm_bindgen(js_namespace = global, js_name = writeStdout)]
    fn js_write_stdout(s: &str);

    #[wasm_bindgen(js_namespace = global, js_name = isWindows)]
    fn js_is_windows() -> bool;

    #[wasm_bindgen(js_namespace = global, js_name = stdinSource, catch)]
    fn js_get_stdin_source() -> Result<String, JsValue>;

    #[wasm_bindgen(js_namespace = global, js_name = readFile, catch)]
    fn js_read_file(path: &str) -> Result<String, JsValue>;

    #[wasm_bindgen(js_namespace = global, js_name = writeFile, catch)]
    fn js_write_file(path: &str, s: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(js_namespace = global, js_name = globPaths, catch)]
    fn js_get_paths_by_glob(pattern: &str) -> Result<GlobPaths, JsValue>;
}

pub(crate) fn is_windows() -> bool {
    unsafe { js_is_windows() }
}

pub(crate) fn get_stdin_source() -> Result<String, io::Error> {
    unsafe { js_get_stdin_source().map_err(to_io_error) }
}

pub(crate) fn read_file(path: &Path) -> Result<String, io::Error> {
    unsafe { js_read_file(path.to_str().unwrap()).map_err(to_io_error) }
}

pub(crate) fn write_file(path: &Path, s: String) -> Result<(), io::Error> {
    unsafe { js_write_file(path.to_str().unwrap(), &s).map_err(to_io_error) }
}

pub(crate) fn get_paths_by_glob(
    pattern: &str,
) -> Result<(Vec<PathBuf>, Vec<io::Error>), io::Error> {
    unsafe {
        js_get_paths_by_glob(pattern)
            .map_err(to_io_error)
            .map(|glob_paths| {
                (
                    glob_paths.paths,
                    glob_paths.errors.into_iter().map(to_io_error).collect(),
                )
            })
    }
}

pub(crate) fn eprint_line(message: &str) {
    unsafe {
        js_write_stderr(message);
        js_write_stderr("\n");
    }
}

pub(crate) fn print_stdout(message: &str) {
    unsafe {
        js_write_stdout(message);
    }
}

pub(crate) fn print_message(severity: Severity, name: &str, message: &str) {
    match severity {
        Severity::Error => {
            eprint_line(&format!("{}: {}", name.red().bold(), message.bold()));
            set_error(true);
        }
        Severity::Warning => eprint_line(&format!("{}: {}", name.yellow().bold(), message.bold())),
        Severity::Info => eprint_line(&format!("{}: {}", name.cyan().bold(), message.bold())),
        Severity::Success => eprint_line(&format!("{}: {}", name.green().bold(), message.bold())),
    }
}

pub(crate) fn load_config(path: Option<&str>) -> Result<Config, io::Error> {
    let cwd = env::current_dir()?;

    if let Some(p) = path {
        return toml::from_str(&read_file(&Path::new(p))?)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err));
    }

    for name in CONFIG_FILE_NAMES {
        let config_path = cwd.join(name);

        if Path::exists(&config_path) {
            return toml::from_str(&read_file(&config_path)?)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        }
    }

    Ok(Config::default())
}

/// For exit conditions.
pub(crate) fn is_error() -> bool {
    ERROR_STATUS.load(Ordering::SeqCst)
}

/// For exit conditions.
pub(crate) fn set_error(err: bool) {
    ERROR_STATUS.store(err, Ordering::SeqCst);
}
