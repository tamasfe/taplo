//! This module contains functions that call into external APIs.
//!
//! The sole purpose of this is to make the tool work in a NodeJS WASM context.

use crate::config::{Config, CONFIG_FILE_NAMES};
use anyhow::anyhow;
use futures::future::join_all;
use glob::glob_with;
use hex::ToHex;
use io::{stdin, Read};
use once_cell::sync::Lazy;
use reqwest::{header, Client};
use schemars::{schema::RootSchema, schema_for};
use sha2::{Digest, Sha256};
use std::{env, time::UNIX_EPOCH};
use std::{
    io,
    path::{Path, PathBuf},
};
use taplo::schema::{CachedSchema, SchemaIndex, BUILTIN_SCHEME};
use tokio::task::JoinHandle;
use tokio_compat_02::FutureExt;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(concat!("Taplo CLI v", env!("CARGO_PKG_VERSION"), " (", env!("BUILD_TARGET"), ")")),
    );
    Client::builder().default_headers(headers).build().unwrap()
});

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

        let res = HTTP_CLIENT.get(path).send().compat().await?;
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
    match HTTP_CLIENT.get(index_url).send().compat().await {
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

pub(crate) async fn update_schemas(
    index: &SchemaIndex,
    cache_path: impl AsRef<Path>,
) -> (usize, Vec<anyhow::Error>) {
    let mut errors = Vec::new();

    let mut updated: usize = 0;
    let mut schema_downloads: Vec<JoinHandle<Result<(), anyhow::Error>>> = Vec::new();

    for schema in &index.schemas {
        if let Some(updated) = &schema.updated {
            match time::OffsetDateTime::parse(updated, time::Format::Rfc3339) {
                Ok(updated) => {
                    let mut hasher = Sha256::new();
                    hasher.update(schema.url.as_bytes());
                    let url_hash = hasher.finalize().encode_hex::<String>();

                    let file_path = cache_path
                        .as_ref()
                        .join("schemas")
                        .join(&url_hash)
                        .with_extension("json");
                    let fp = file_path.to_str().unwrap();

                    if let Ok(true) = needs_update(fp, (updated.unix_timestamp() * 1000) as u64) {
                        let path = schema.url.clone();
                        let cache_path = PathBuf::from(cache_path.as_ref());

                        schema_downloads.push(tokio::spawn(async move {
                            let res = HTTP_CLIENT.get(&path).send().compat().await.unwrap();
                            let schema: RootSchema = res
                                .json()
                                .await
                                .map_err::<anyhow::Error, _>(Into::into)
                                .unwrap();

                            mkdir(cache_path.join("schemas").to_str().unwrap())?;

                            write_file(
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
                            .await?;

                            Ok(())
                        }));
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

    for result in join_all(schema_downloads).await {
        match result.unwrap() {
            Ok(_) => {
                updated += 1;
            }
            Err(err) => errors.push(err),
        }
    }

    (updated, errors)
}

pub(crate) fn read_stdin() -> Result<String, anyhow::Error> {
    let mut src = String::new();
    stdin().read_to_string(&mut src)?;
    Ok(src)
}

pub(crate) async fn read_file(p: &str) -> Result<Vec<u8>, anyhow::Error> {
    Ok(tokio::fs::read(p).await?)
}

pub(crate) async fn write_file(p: &str, data: &[u8]) -> Result<(), anyhow::Error> {
    Ok(tokio::fs::write(p, data).await?)
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

pub(crate) async fn load_config(path: Option<&str>) -> Result<Config, anyhow::Error> {
    let cwd = env::current_dir()?;

    if let Some(p) = path {
        return toml::from_slice(&read_file(p).await?).map_err(Into::into);
    }

    for name in CONFIG_FILE_NAMES {
        let config_path = cwd.join(name);

        if Path::exists(&config_path) {
            return toml::from_slice(&read_file(config_path.to_str().unwrap()).await?)
                .map_err(Into::into);
        }
    }

    Ok(Config::default())
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
