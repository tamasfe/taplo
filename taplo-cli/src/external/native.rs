//! This module contains functions that call into external APIs.
//!
//! The sole purpose of this is to make the tool work in a NodeJS WASM context.

use crate::{
    config::{Config, CONFIG_FILE_NAMES},
    glob_match_options,
};
use glob::glob_with;
use io::{stdin, Read};
use pretty_lint::{colored::Colorize, Severity};
use std::{env, io::Write};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use once_cell::sync::Lazy;

static ERROR_STATUS: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

pub(crate) fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

pub(crate) fn get_stdin_source() -> Result<String, io::Error> {
    let mut src = String::new();
    stdin().read_to_string(&mut src)?;
    Ok(src)
}

pub(crate) fn read_file(path: &Path) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

pub(crate) fn write_file(path: &Path, s: String) -> Result<(), io::Error> {
    let mut f = fs::File::create(path)?;
    f.write_all(s.as_bytes())
}

pub(crate) fn get_paths_by_glob(
    pattern: &str,
) -> Result<(Vec<PathBuf>, Vec<io::Error>), io::Error> {
    let mut sources = Vec::with_capacity(16);
    let mut file_errors = Vec::with_capacity(16);

    let paths = glob_with(pattern, glob_match_options())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    for p_res in paths {
        match p_res.map_err(|e| e.into_error()) {
            Ok(p) => sources.push(p),
            Err(e) => {
                file_errors.push(e);
            }
        }
    }

    Ok((sources, file_errors))
}

pub(crate) fn eprint_line(message: &str) {
    println!("{}", message);
}

pub(crate) fn print_stdout(message: &str) {
    print!("{}", message);
}

pub(crate) fn print_message(severity: Severity, name: &str, message: &str) {
    match severity {
        Severity::Error => {
            eprintln!("{}: {}", name.red().bold(), message.bold());
            set_error(true);
        }
        Severity::Warning => eprintln!("{}: {}", name.yellow().bold(), message.bold()),
        Severity::Info => eprintln!("{}: {}", name.cyan().bold(), message.bold()),
        Severity::Success => eprintln!("{}: {}", name.green().bold(), message.bold()),
    }
}

pub(crate) fn load_config(path: Option<&str>) -> Result<Config, io::Error> {
    let cwd = env::current_dir()?;

    if let Some(p) = path {
        return toml::from_str(&fs::read_to_string(p)?)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err));
    }

    for name in CONFIG_FILE_NAMES {
        let config_path = cwd.join(name);

        if Path::exists(&config_path) {
            return toml::from_str(&fs::read_to_string(config_path)?)
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
