#![allow(unused_macros)]
// extern WASM calls are wrapped in unsafe,
// but they don't technically have to be.
#![deny(unused_unsafe)]

use anyhow::anyhow;
use external::{file_exists, is_absolute_path, mkdir, read_file, spawn, write_file};
use hex::ToHex;
use indexmap::IndexMap;
use lsp_async_stub::{Context, Server};
use lsp_types::{notification, request, Url};
use parking_lot::RwLock;
use schemars::{schema::RootSchema, schema_for};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, hash::Hash, path::Path, path::PathBuf, sync::Arc};
use taplo::{
    analytics::Directive,
    parser::Parse,
    schema::{CachedSchema, BUILTIN_SCHEME},
    util::coords::Mapper,
};

#[cfg(not(target_arch = "wasm32"))]
#[path = "external/native/mod.rs"]
#[macro_use]
pub mod external;

#[cfg(target_arch = "wasm32")]
#[path = "external/wasm32/mod.rs"]
#[macro_use]
pub mod external;

#[cfg(target_arch = "wasm32")]
use external::UrlExt;

mod handlers;
mod msg_ext;
mod utils;

#[derive(Debug, Clone)]
pub struct Document {
    parse: Parse,
    mapper: Mapper,
}

/// Regex with hash and Eq
struct HashRegex(pub regex::Regex);

impl Hash for HashRegex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.as_str().hash(state)
    }
}

impl PartialEq for HashRegex {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for HashRegex {}

impl From<regex::Regex> for HashRegex {
    fn from(re: regex::Regex) -> Self {
        Self(re)
    }
}

#[derive(Default)]
pub struct WorldState {
    cache_path: Option<PathBuf>,
    workspace_uri: Option<Url>,
    documents: HashMap<lsp_types::Url, Document>,
    schema_associations: IndexMap<HashRegex, String>,
    index_schema_associations: IndexMap<HashRegex, String>,
    http_client: reqwest::Client,
    configuration: Configuration,
    taplo_config: Option<taplo_cli::config::Config>,
}

impl WorldState {
    fn get_config_formatter_options(
        &self,
        uri: &Url,
        mut default_opts: taplo::formatter::Options,
    ) -> (
        taplo::formatter::Options,
        Vec<(String, taplo::formatter::OptionsIncomplete)>,
    ) {
        let mut incomplete = Vec::new();

        if let Some(c) = &self.taplo_config {
            if let Ok(p) = uri.to_file_path() {
                let p = self
                    .workspace_uri
                    .as_ref()
                    .and_then(|ws| ws.to_file_path().ok())
                    .and_then(|ws| pathdiff::diff_paths(Path::new(&p), ws))
                    .unwrap_or(p);

                match c.get_formatter_options(p.to_str(), Some(default_opts.clone())) {
                    Ok((opts, inc)) => {
                        default_opts = opts;
                        incomplete.extend(inc);
                    }
                    Err(err) => {
                        log_warn!("invalid config: {}", err);
                    }
                }
            }
        }

        (default_opts, incomplete)
    }

    /// For the given `Url`, if it exists in the map of known documents,
    /// try to parse the schema path from a toml comment.
    ///
    /// Example:
    ///
    /// ```toml
    /// #:schema ./foo/bar
    /// # ...rest of toml file
    /// ```
    ///
    /// returns `"/.foo/bar"`.
    ///
    /// If the file does not contain such a schema comment, we look into the taplo config,
    /// which assigns file regexes (relative to the `workspace_uri`) to schema files.
    ///
    /// If nothing is found, returns `None`.
    fn get_schema_name(&self, uri: &Url) -> Option<String> {
        if let Some(doc) = self.documents.get(uri) {
            for directive in Directive::collect_from_syntax(doc.parse.clone().into_syntax()) {
                if directive.value.starts_with("schema") {
                    return directive
                        .value
                        .split_whitespace()
                        .nth(1)
                        .map(|s| s.to_string());
                }
            }
        }

        if let Some(c) = &self.taplo_config {
            if let Ok(p) = uri.to_file_path() {
                let p = self
                    .workspace_uri
                    .as_ref()
                    .and_then(|ws| ws.to_file_path().ok())
                    .and_then(|ws| pathdiff::diff_paths(Path::new(&p), ws))
                    .unwrap_or(p);

                if let Some(p) = p.to_str() {
                    match c.get_schema_path(p) {
                        Ok(p) => {
                            if p.is_some() {
                                return p;
                            }
                        }
                        Err(err) => {
                            log_warn!("invalid config: {}", err);
                        }
                    }
                }
            }
        }

        let s = uri.as_str();

        for (re, name) in self.schema_associations.iter() {
            if re.0.is_match(s) {
                return Some(name.clone());
            }
        }

        for (re, name) in self.index_schema_associations.iter() {
            if re.0.is_match(s) {
                return Some(name.clone());
            }
        }

        None
    }

    fn workspace_path(&self) -> Option<PathBuf> {
        match &self.workspace_uri {
            Some(uri) => uri.to_file_path().ok(),
            None => None,
        }
    }

    /// Get the schema for a given file and schema path/url.
    async fn get_schema(
        // File to get the schema for.
        for_url: &Url,
        // Path of the schema file, as either
        //
        // - a `taplo://` url
        // - an absolute `file://` path
        // - an absolute path (same as `file://`)
        // - a `http://` or `https://`
        // - or a relative path, which is resolved relative to the `for_url` (this should work for both local files and remote files)
        mut path: &str,
        mut context: Context<World>,
    ) -> Result<RootSchema, anyhow::Error> {
        // resolve taplo://
        if path.starts_with(&format!("{}://", BUILTIN_SCHEME)) {
            if path == "taplo://taplo.toml" {
                Ok(schema_for!(taplo_cli::config::Config))
            } else {
                Err(anyhow!("invalid builtin schema: {}", path))
            }
        // resolve http://, https://
        } else if path.starts_with("http://") || path.starts_with("https://") {
            let w = context.world().read();

            let mut hasher = Sha256::new();
            hasher.update(path.as_bytes());
            let url_hash = hasher.finalize().encode_hex::<String>();

            if let Some(cache_path) = &w.cache_path {
                let file_path = cache_path
                    .join("schemas")
                    .join(&url_hash)
                    .with_extension("json");
                let fp = file_path.to_str().unwrap();

                if file_exists(fp) {
                    let schema_bytes = read_file(fp).await?;
                    let cached_schema: CachedSchema = serde_json::from_slice(&schema_bytes)?;
                    return Ok(cached_schema.schema);
                }
            }

            let client = w.http_client.clone();
            drop(w);

            let res = client.get(path).send().await?;
            let schema: RootSchema = res.json().await.map_err::<anyhow::Error, _>(Into::into)?;

            let w = context.world().read();

            let p = path.to_string();
            let s = schema.clone();

            // We also cache it here.
            if let Some(cache_path) = w.cache_path.clone() {
                match mkdir(cache_path.join("schemas").to_str().unwrap()) {
                    Ok(_) => {
                        spawn(async move {
                            if let Err(err) = write_file(
                                cache_path
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
                                log_error!("failed to cache schema: {}", err);
                            };
                        });
                    }
                    Err(err) => {
                        log_error!("failed to cache schema: {}", err);
                    }
                }
            }

            Ok(schema)
        // resolve file://
        } else if path.starts_with("file://") {
            path = path.trim_start_matches("file://");
            serde_json::from_slice(&read_file(path).await?).map_err(Into::into)
        // same as file:// for absolute paths
        } else if is_absolute_path(path) {
            serde_json::from_slice(&read_file(path).await?).map_err(Into::into)
        // resolve relative paths, relative to the `for_url`.
        } else {
            // This should in theory work for any type of url, so if `for_url` is
            // `http://foo.bar/baz.toml`
            // then `./schema.json` ould resolve to
            // `http://foo.bar/schema.json`
            // which sounds like something one would want.
            // However, implementing this will take some more refactoring, so for now we error out.
            if for_url.scheme() != "file" {
                return Err(anyhow!(
                    "File {} is trying to load relative schema {}, but we only support loading relative schemas from local files right now, not {}",
                    for_url,
                    path,
                    for_url.scheme()
                ));
            }
            match for_url.join(path) {
                Ok(schema) => serde_json::from_slice(
                    &read_file(
                        schema
                            .to_file_path()
                            .unwrap_or_else(|_| panic!("{} has to be a file path here", schema))
                            // this should be utf-8 safe since it came from an URL
                            .to_str()
                            .unwrap(),
                    )
                    .await?,
                )
                .map_err(Into::into),
                Err(err) => Err(anyhow!(
                    "Cannot resolve relative schema {}, coming from file {}. Error: {}",
                    path,
                    for_url,
                    err
                )),
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaConfiguration {
    pub enabled: Option<bool>,
    pub associations: Option<HashMap<String, String>>,
    pub repository_enabled: Option<bool>,
    pub repository_url: Option<String>,
    pub links: Option<bool>,
}

// This is not exhaustive
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    taplo_config: Option<String>,
    taplo_config_enabled: Option<bool>,
    schema: SchemaConfiguration,
    semantic_tokens: Option<bool>,
    cache_path: Option<String>,
    formatter: taplo::formatter::OptionsIncompleteCamel,
}

pub type World = Arc<RwLock<WorldState>>;

pub fn create_server() -> Server<World> {
    Server::new()
        .on_request::<request::Initialize, _>(handlers::initialize)
        .on_request::<request::FoldingRangeRequest, _>(handlers::folding_ranges)
        .on_request::<request::DocumentSymbolRequest, _>(handlers::document_symbols)
        .on_request::<request::Formatting, _>(handlers::format)
        .on_request::<request::Completion, _>(handlers::completion)
        .on_request::<request::HoverRequest, _>(handlers::hover)
        .on_request::<request::DocumentLinkRequest, _>(handlers::links)
        .on_request::<request::SemanticTokensFullRequest, _>(handlers::semantic_tokens)
        .on_request::<request::CodeActionRequest, _>(handlers::code_action)
        .on_request::<msg_ext::TomlToJsonRequest, _>(handlers::toml_to_json)
        .on_request::<msg_ext::JsonToTomlRequest, _>(handlers::json_to_toml)
        .on_request::<msg_ext::SyntaxTreeRequest, _>(handlers::syntax_tree)
        .on_notification::<notification::DidOpenTextDocument, _>(handlers::document_open)
        .on_notification::<notification::DidChangeTextDocument, _>(handlers::document_change)
        .on_notification::<notification::DidCloseTextDocument, _>(handlers::document_close)
        .on_notification::<notification::DidChangeConfiguration, _>(handlers::configuration_change)
        .on_notification::<msg_ext::CachePath, _>(handlers::cache_path)
        .build()
}

pub fn create_world() -> World {
    Arc::new(RwLock::new(WorldState::default()))
}
