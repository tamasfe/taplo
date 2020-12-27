#![allow(unused_unsafe)]
use crate::config::{Config, CONFIG_FILE_NAMES};
use anyhow::anyhow;
use hex::ToHex;
use js_sys::Uint8Array;
use once_cell::sync::Lazy;
use reqwest::{Client, header};
use schemars::schema::RootSchema;
use schemars::schema_for;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use taplo::schema::{CachedSchema, SchemaIndex, BUILTIN_SCHEME};
use wasm_bindgen::prelude::*;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(concat!("Taplo CLI v", env!("CARGO_PKG_VERSION"), " (", env!("BUILD_TARGET"), ")")),
    );
    Client::builder().default_headers(headers).build().unwrap()
});

pub(crate) fn colored_output() -> Option<bool> {
    Some(unsafe { js_is_tty() })
}

fn js_err_to_anyhow(err: JsValue) -> anyhow::Error {
    anyhow!("{:?}", err)
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = global, js_name = readStdin, catch)]
    fn js_read_stdin() -> Result<String, JsValue>;

    #[wasm_bindgen(js_namespace = process, js_name = cwd)]
    fn js_current_dir() -> String;

    #[wasm_bindgen(js_namespace = global, js_name = isATty)]
    fn js_is_tty() -> bool;

    #[wasm_bindgen(js_namespace = global, js_name = isWindows)]
    fn js_is_windows() -> bool;

    #[wasm_bindgen(js_namespace = global, js_name = sendMessage)]
    fn js_send_message(message: JsValue);

    #[wasm_bindgen(js_namespace = global, js_name = readFile, catch)]
    async fn js_read_file(path: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = global, js_name = writeFile, catch)]
    async fn js_write_file(path: &str, data: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(js_namespace = global, js_name = fileExists)]
    fn js_file_exists(path: &str) -> bool;

    #[wasm_bindgen(js_namespace = global, js_name = mkdir, catch)]
    fn js_mkdir(path: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(js_namespace = global, js_name = needsUpdate, catch)]
    fn js_needs_update(path: &str, new_date_ms: u64) -> Result<bool, JsValue>;

    #[wasm_bindgen(js_namespace = global, js_name = isAbsolutePath)]
    fn js_is_absolute_path(path: &str) -> bool;

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

pub(crate) fn get_paths_by_glob(pattern: &str) -> Result<Vec<PathBuf>, anyhow::Error> {
    unsafe {
        js_glob_paths(pattern)
            .map_err(js_err_to_anyhow)
            .map(|v| v.into_serde().unwrap())
    }
}

pub(crate) async fn load_config(path: Option<&str>) -> Result<Config, anyhow::Error> {
    let cwd = PathBuf::from(unsafe { js_current_dir() });

    if let Some(p) = path {
        return toml::from_slice(&read_file(p).await?).map_err(Into::into);
    }

    for name in CONFIG_FILE_NAMES {
        let config_path = cwd.join(name);

        unsafe {
            if js_file_exists(config_path.to_str().unwrap()) {
                return toml::from_slice(&read_file(config_path.to_str().unwrap()).await?)
                    .map_err(Into::into);
            }
        }
    }

    Ok(Config::default())
}

pub(crate) async fn get_schema(
    mut path: &str,
    cache_path: Option<impl AsRef<Path>>,
) -> Result<(RootSchema, Option<anyhow::Error>), anyhow::Error> {
    if path.starts_with(&format!("{}://", BUILTIN_SCHEME)) {
        if path == "taplo://taplo.toml" {
            Ok((schema_for!(Config), None))
        } else {
            Err(anyhow!("invalid builtin schema: {}", path))
        }
    } else if path.starts_with("http://") || path.starts_with("https://") {
        let mut hasher = Sha256::new();
        hasher.update(path.as_bytes());
        let url_hash = hasher.finalize().encode_hex::<String>();

        if let Some(cache_path) = cache_path.as_ref() {
            let file_path = cache_path
                .as_ref()
                .join("schemas")
                .join(&url_hash)
                .with_extension("json");
            let fp = file_path.to_str().unwrap();

            if file_exists(fp) {
                let schema_bytes = read_file(fp).await?;
                let cached_schema: CachedSchema = serde_json::from_slice(&schema_bytes)?;
                return Ok((cached_schema.schema, None));
            }
        }

        let res = HTTP_CLIENT.get(path).send().await?;
        let schema: RootSchema = res.json().await.map_err::<anyhow::Error, _>(Into::into)?;

        let p = path.to_string();
        let s = schema.clone();

        // We also cache it here.
        if let Some(cache_path) = cache_path.as_ref() {
            mkdir(cache_path.as_ref().join("schemas").to_str().unwrap())?;
            if let Err(err) = write_file(
                cache_path
                    .as_ref()
                    .join("schemas")
                    .join(&url_hash)
                    .with_extension("json")
                    .to_str()
                    .unwrap(),
                &serde_json::to_vec(&CachedSchema {
                    url: Some(p),
                    schema: s,
                })
                .unwrap(),
            )
            .await
            {
                return Ok((schema, Some(anyhow!("failed to cache schema: {}", err))));
            };
        }

        Ok((schema, None))
    } else if path.starts_with("file://") {
        path = path.trim_start_matches("file://");
        Ok((serde_json::from_slice(&read_file(path).await?)?, None))
    } else {
        Ok((serde_json::from_slice(&read_file(path).await?)?, None))
    }
}

pub(crate) async fn download_schema_index(index_url: &str) -> Result<SchemaIndex, anyhow::Error> {
    match HTTP_CLIENT.get(index_url).send().await {
        Ok(res) => match res.json::<SchemaIndex>().await {
            Ok(idx) => Ok(idx),
            Err(err) => Err(anyhow!("invalid schema index: {}", err)),
        },
        Err(err) => Err(anyhow!("invalid schema index: {}", err)),
    }
}

pub(crate) async fn load_schema_index(
    cache_path: impl AsRef<Path>,
) -> Result<Option<SchemaIndex>, anyhow::Error> {
    let index_path = cache_path
        .as_ref()
        .join("schema_index")
        .with_extension("json");

    if file_exists(index_path.to_str().unwrap()) {
        match read_file(index_path.to_str().unwrap()).await {
            Ok(data) => match serde_json::from_slice(&data) {
                Ok(idx) => Ok(Some(idx)),
                Err(err) => Err(anyhow!("invalid cached schema index: {}", err)),
            },
            Err(err) => Err(anyhow!("failed to load schema index: {}", err)),
        }
    } else {
        Ok(None)
    }
}

// TODO: concurrent downloads
pub(crate) async fn update_schemas(
    index: &SchemaIndex,
    cache_path: impl AsRef<Path>,
) -> (usize, Vec<anyhow::Error>) {
    let mut errors = Vec::new();

    let mut updated: usize = 0;

    for schema in &index.schemas {
        if let Some(updated_str) = &schema.updated {
            match time::OffsetDateTime::parse(updated_str, time::Format::Rfc3339) {
                Ok(updated_ts) => {
                    let mut hasher = Sha256::new();
                    hasher.update(schema.url.as_bytes());
                    let url_hash = hasher.finalize().encode_hex::<String>();

                    let file_path = cache_path
                        .as_ref()
                        .join("schemas")
                        .join(&url_hash)
                        .with_extension("json");
                    let fp = file_path.to_str().unwrap();

                    if let Ok(true) = needs_update(fp, (updated_ts.unix_timestamp() * 1000) as u64)
                    {
                        let path = schema.url.clone();
                        let cache_path = PathBuf::from(cache_path.as_ref());

                        let res = HTTP_CLIENT.get(&path).send().await.unwrap();
                        let schema: RootSchema = res
                            .json()
                            .await
                            .map_err::<anyhow::Error, _>(Into::into)
                            .unwrap();

                        if let Err(err) = mkdir(cache_path.join("schemas").to_str().unwrap()) {
                            errors.push(err);
                            continue;
                        }

                        match write_file(
                            cache_path
                                .join("schemas")
                                .join(&url_hash)
                                .with_extension("json")
                                .to_str()
                                .unwrap(),
                            &serde_json::to_vec(&CachedSchema {
                                url: Some(path),
                                schema,
                            })
                            .unwrap(),
                        )
                        .await
                        {
                            Ok(_) => updated += 1,
                            Err(err) => errors.push(err),
                        }
                    }
                }
                Err(err) => {
                    errors.push(anyhow!(
                        "schema update date is invalid ({}): {}",
                        &schema.url,
                        err
                    ));
                }
            }
        }
    }

    (updated, errors)
}

pub(crate) async fn read_file(p: &str) -> Result<Vec<u8>, anyhow::Error> {
    let res: JsValue = js_read_file(p).await.map_err(|e| anyhow!("{:?}", e))?;
    Ok(Uint8Array::from(res).to_vec())
}

pub(crate) async fn write_file(p: &str, data: &[u8]) -> Result<(), anyhow::Error> {
    js_write_file(p, Uint8Array::from(data).into())
        .await
        .map_err(|e| anyhow!("{:?}", e))?;
    Ok(())
}

pub(crate) fn file_exists(p: &str) -> bool {
    Path::new(p).exists()
}

pub(crate) fn mkdir(p: &str) -> Result<(), anyhow::Error> {
    js_mkdir(p).map_err(|e| anyhow!("{:?}", e))?;
    Ok(())
}

pub(crate) fn needs_update(p: &str, new_date_ms: u64) -> Result<bool, anyhow::Error> {
    js_needs_update(p, new_date_ms).map_err(|e| anyhow!("{:?}", e))
}
