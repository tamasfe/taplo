#![allow(unused_macros)]
// extern WASM calls are wrapped in unsafe,
// but they don't technically have to be.
#![allow(unused_unsafe)]

use futures::lock::Mutex as AsyncMutex;
use indexmap::IndexMap;
use lsp_async_stub::Server;
use lsp_types::{notification, request, Url};
use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash, path::Path, path::PathBuf, sync::Arc};
use taplo::{parser::Parse, schema::BUILTIN_SCHEMAS, util::coords::Mapper};

#[cfg(test)]
mod tests;

#[cfg(not(target_arch = "wasm32"))]
#[path = "external/native/mod.rs"]
#[macro_use]
pub mod external;

#[cfg(target_arch = "wasm32")]
#[path = "external/wasm32/mod.rs"]
#[macro_use]
pub mod external;

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
    workspace_uri: Option<Url>,
    documents: HashMap<lsp_types::Url, Document>,
    schemas: HashMap<String, RootSchema>,
    schema_associations: IndexMap<HashRegex, String>,
    http_client: reqwest::Client,
    configuration: Configuration,
    taplo_config: Option<taplo_cli::config::Config>,
}

impl WorldState {
    fn register_built_in_schemas(&mut self) {
        for (name, schema) in &*BUILTIN_SCHEMAS {
            self.schemas.insert(name.clone(), schema.clone());
        }
    }

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
            if let Some(ws) = &self.workspace_uri {
                if let Some(p) = pathdiff::diff_paths(Path::new(uri.path()), ws.path()) {
                    if let Some(p) = p.to_str() {
                        match c.get_formatter_options(Some(p), Some(default_opts.clone())) {
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
            }

            let p = uri.path();

            match c.get_formatter_options(Some(p), Some(default_opts.clone())) {
                Ok((opts, inc)) => {
                    default_opts = opts;
                    incomplete.extend(inc);
                }
                Err(err) => {
                    log_warn!("invalid config: {}", err);
                }
            }
        }

        (default_opts, incomplete)
    }

    fn get_schema_name(&self, uri: &Url) -> Option<String> {
        if let Some(c) = &self.taplo_config {
            if let Some(ws) = &self.workspace_uri {
                if let Some(p) = pathdiff::diff_paths(Path::new(uri.path()), ws.path()) {
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

            let p = uri.path();

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

        let s = uri.as_str();

        for (re, name) in self.schema_associations.iter() {
            if re.0.is_match(s) {
                return Some(name.clone());
            }
        }

        None
    }

    fn get_schema(&self, name: &str) -> Option<&RootSchema> {
        self.schemas.get(name)
    }

    fn get_schema_by_uri(&self, uri: &Url) -> Option<&RootSchema> {
        for (re, name) in &self.schema_associations {
            if re.0.is_match(uri.as_str()) {
                return self.get_schema(name);
            }
        }

        None
    }

    fn workspace_path(&self) -> Option<PathBuf> {
        match &self.workspace_uri {
            Some(uri) => Some(PathBuf::from(uri.path())),
            None => None,
        }
    }

    fn workspace_absolute<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
        let workspace = &self.workspace_uri;

        match workspace {
            Some(uri) => Some(Path::new(uri.path()).join(path)),
            None => None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaConfiguration {
    pub enabled: Option<bool>,
    pub associations: Option<HashMap<String, String>>,
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
    formatter: taplo::formatter::OptionsIncompleteCamel,
}

pub type World = Arc<AsyncMutex<WorldState>>;

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
        .on_request::<msg_ext::TomlToJsonRequest, _>(handlers::toml_to_json)
        .on_request::<msg_ext::SyntaxTreeRequest, _>(handlers::syntax_tree)
        .on_notification::<notification::DidOpenTextDocument, _>(handlers::document_open)
        .on_notification::<notification::DidChangeTextDocument, _>(handlers::document_change)
        .on_notification::<notification::DidCloseTextDocument, _>(handlers::document_close)
        .on_notification::<notification::DidChangeConfiguration, _>(handlers::configuration_change)
        .on_notification::<msg_ext::ConfigFileChanged, _>(handlers::config_file_changed)
        .build()
}

pub fn create_world() -> World {
    let mut w = WorldState::default();
    w.register_built_in_schemas();
    Arc::new(AsyncMutex::new(WorldState::default()))
}
