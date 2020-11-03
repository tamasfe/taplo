#![allow(unused_unsafe)]
use crate::{config::{Config, CONFIG_FILE_NAMES}, print_message};
use anyhow::anyhow;
use once_cell::sync::Lazy;
use pretty_lint::Severity;
use reqwest::Client;
use schemars::schema::RootSchema;
use std::path::{Path, PathBuf};
use taplo::schema::{BUILTIN_SCHEMAS, BUILTIN_SCHEME};
use wasm_bindgen::prelude::*;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub(crate) fn colored_output() -> Option<bool> {
    Some(unsafe { js_is_tty() })
}

pub(crate) async fn get_schema(mut path: &str) -> Result<RootSchema, anyhow::Error> {
    if path.starts_with(&format!("{}://", BUILTIN_SCHEME)) {
        match BUILTIN_SCHEMAS.get(path) {
            Some(s) => Ok(s.clone()),
            None => Err(anyhow!("builtin schema was not found: {}", path)),
        }
    } else if path.starts_with("http://") || path.starts_with("https://") {
        print_message(
            Severity::Info,
            "info",
            &format!("using remote schema at {}", path),
        );
        let res = HTTP_CLIENT.get(path).send().await?;
        res.json().await.map_err(Into::into)
    } else {
        if path.starts_with("file://") {
            path = path.trim_start_matches("file://");
        }

        serde_json::from_str(&read_file(&Path::new(path))?).map_err(Into::into)
    }
}

fn js_err_to_anyhow(err: JsValue) -> anyhow::Error {
    anyhow!("{:?}", err)
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = global, js_name = isATty)]
    fn js_is_tty() -> bool;

    #[wasm_bindgen(js_namespace = global, js_name = isWindows)]
    fn js_is_windows() -> bool;

    #[wasm_bindgen(js_namespace = global, js_name = fileExists)]
    fn js_file_exists(p: &str) -> bool;

    #[wasm_bindgen(js_namespace = process, js_name = cwd)]
    fn js_current_dir() -> String;

    #[wasm_bindgen(js_namespace = global, js_name = readStdin, catch)]
    fn js_read_stdin() -> Result<String, JsValue>;

    #[wasm_bindgen(js_namespace = global, js_name = readFile, catch)]
    fn js_read_file(path: &str) -> Result<String, JsValue>;

    #[wasm_bindgen(js_namespace = global, js_name = writeFile, catch)]
    fn js_write_file(path: &str, s: &[u8]) -> Result<(), JsValue>;

    #[wasm_bindgen(js_namespace = global, js_name = globPaths, catch)]
    fn js_glob_paths(pattern: &str) -> Result<JsValue, JsValue>;
}

pub(crate) fn glob_match_options() -> glob::MatchOptions {
    glob::MatchOptions {
        case_sensitive: unsafe { js_is_windows() },
        require_literal_leading_dot: false,
        require_literal_separator: false,
    }
}

pub(crate) fn read_stdin() -> Result<String, anyhow::Error> {
    unsafe { js_read_stdin().map_err(js_err_to_anyhow) }
}

pub(crate) fn read_file(path: &Path) -> Result<String, anyhow::Error> {
    unsafe { js_read_file(path.to_str().unwrap()).map_err(js_err_to_anyhow) }
}

pub(crate) fn write_file(path: &Path, s: &[u8]) -> Result<(), anyhow::Error> {
    unsafe { js_write_file(path.to_str().unwrap(), &s).map_err(js_err_to_anyhow) }
}

pub(crate) fn get_paths_by_glob(pattern: &str) -> Result<Vec<PathBuf>, anyhow::Error> {
    unsafe {
        js_glob_paths(pattern)
            .map_err(js_err_to_anyhow)
            .map(|v| v.into_serde().unwrap())
    }
}

pub(crate) fn load_config(path: Option<&str>) -> Result<Config, anyhow::Error> {
    let cwd = PathBuf::from(unsafe { js_current_dir() });

    if let Some(p) = path {
        return toml::from_str(&read_file(&Path::new(p))?).map_err(Into::into);
    }

    for name in CONFIG_FILE_NAMES {
        let config_path = cwd.join(name);

        unsafe {
            if js_file_exists(config_path.to_str().unwrap()) {
                return toml::from_str(&read_file(&config_path)?).map_err(Into::into);
            }
        }
    }

    Ok(Config::default())
}
