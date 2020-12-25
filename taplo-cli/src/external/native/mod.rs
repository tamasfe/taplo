//! This module contains functions that call into external APIs.
//!
//! The sole purpose of this is to make the tool work in a NodeJS WASM context.

use crate::{
    config::{Config, CONFIG_FILE_NAMES},
    print_message,
};
use anyhow::anyhow;
use glob::glob_with;
use io::{stdin, Read};
use once_cell::sync::Lazy;
use pretty_lint::Severity;
use reqwest::Client;
use schemars::{schema::RootSchema, schema_for};
use std::{env, io::Write};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use taplo::schema::BUILTIN_SCHEME;
use tokio_compat_02::FutureExt;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub(crate) const fn colored_output() -> Option<bool> {
    None
}

pub(crate) const fn glob_match_options() -> glob::MatchOptions {
    glob::MatchOptions {
        case_sensitive: cfg!(not(windows)),
        require_literal_leading_dot: false,
        require_literal_separator: false,
    }
}

pub(crate) async fn get_schema(mut path: &str) -> Result<RootSchema, anyhow::Error> {
    if path.starts_with(&format!("{}://", BUILTIN_SCHEME)) {
        if path == "taplo://taplo.toml" {
            Ok(schema_for!(Config))
        } else {
            Err(anyhow!("invalid builtin schema: {}", path))
        }
    } else if path.starts_with("http://") || path.starts_with("https://") {
        print_message(
            Severity::Info,
            "info",
            &format!("using remote schema at {}", path),
        );
        let res = HTTP_CLIENT.get(path).send().compat().await?;
        res.json().compat().await.map_err(Into::into)
    } else {
        if path.starts_with("file://") {
            path = path.trim_start_matches("file://");
        }

        serde_json::from_str(&read_file(&Path::new(path))?).map_err(Into::into)
    }
}

pub(crate) fn read_stdin() -> Result<String, anyhow::Error> {
    let mut src = String::new();
    stdin().read_to_string(&mut src)?;
    Ok(src)
}

pub(crate) fn read_file(path: &Path) -> Result<String, anyhow::Error> {
    fs::read_to_string(path).map_err(Into::into)
}

pub(crate) fn write_file(path: &Path, s: &[u8]) -> Result<(), anyhow::Error> {
    let mut f = fs::File::create(path)?;
    f.write_all(s).map_err(Into::into)
}

pub(crate) fn get_paths_by_glob(pattern: &str) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut sources = Vec::with_capacity(16);

    let paths = glob_with(pattern, glob_match_options())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    for p_res in paths {
        match p_res.map_err(|e| e.into_error()) {
            Ok(p) => sources.push(p),
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(sources)
}

pub(crate) fn load_config(path: Option<&str>) -> Result<Config, anyhow::Error> {
    let cwd = env::current_dir()?;

    if let Some(p) = path {
        return toml::from_str(&fs::read_to_string(p)?).map_err(Into::into);
    }

    for name in CONFIG_FILE_NAMES {
        let config_path = cwd.join(name);

        if Path::exists(&config_path) {
            return toml::from_str(&fs::read_to_string(config_path)?).map_err(Into::into);
        }
    }

    Ok(Config::default())
}
