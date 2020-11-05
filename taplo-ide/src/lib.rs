#![allow(unused_macros)]
// extern WASM calls are wrapped in unsafe,
// but they don't technically have to be.
#![allow(unused_unsafe)]

use async_trait::async_trait;
use futures::{lock::Mutex as AsyncMutex, Sink};
use indexmap::IndexMap;
use lsp_async_stub::{
    rpc::{self, Message},
    NotificationHandler, RequestHandler, ResponseWriter, Server,
};
use lsp_types::{notification, request, Url};
use once_cell::sync::Lazy;
use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash, io, path::Path, path::PathBuf, sync::Arc, task};
use taplo::{parser::Parse, schema::BUILTIN_SCHEMAS, util::coords::Mapper};
use task::Poll;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

macro_rules! log_info {
    ($($arg:tt)*) => {
        // unsafe: Extern JS call
        unsafe { $crate::log_info(&format!($($arg)*)) }
    };
}

macro_rules! log_warn {
    ($($arg:tt)*) => {
        // unsafe: Extern JS call
        unsafe { $crate::log_warn(&format!($($arg)*)) }
    };
}

macro_rules! log_error {
    ($($arg:tt)*) => {
        // unsafe: Extern JS call
        unsafe { $crate::log_error(&format!($($arg)*)) }
    };
}

macro_rules! log_debug {
    ($($arg:tt)*) => {
        // unsafe: Extern JS call
        if cfg!(debug_assertions) { unsafe { $crate::log_info(&format!($($arg)*)) } }
    };
}

mod handlers;
mod request_ext;
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
    pub fn register_built_in_schemas(&mut self) {
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

type World = Arc<AsyncMutex<WorldState>>;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_info(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn log_error(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    fn log_warn(s: &str);

    #[wasm_bindgen(js_namespace = global, js_name = sendMessage)]
    fn send_message(message: JsValue);

    #[wasm_bindgen(js_namespace = global, js_name = readFile, catch)]
    fn read_file(path: &str) -> Result<Vec<u8>, JsValue>;

    #[wasm_bindgen(js_namespace = global, js_name = fileExists)]
    fn file_exists(path: &str) -> bool;

    #[wasm_bindgen(js_namespace = global, js_name = isAbsolutePath)]
    fn is_absolute_path(path: &str) -> bool;
}

#[wasm_bindgen]
pub async fn initialize() {
    utils::set_panic_hook();
    WORLD.lock().await.register_built_in_schemas();
}

#[wasm_bindgen]
pub fn message(message: JsValue) {
    // log_debug!("message: {:?}", message);
    spawn_local(SERVER.handle_message(WORLD.clone(), message.into_serde().unwrap(), ResWriter));
}

static WORLD: Lazy<World> = Lazy::new(|| Arc::new(AsyncMutex::new(WorldState::default())));

static SERVER: Lazy<Server<World>> = Lazy::new(|| {
    Server::new()
        .handler(RequestHandler::<request::Initialize, _, _>::new(
            handlers::initialize,
        ))
        .handler(
            NotificationHandler::<notification::DidOpenTextDocument, _, _>::new(
                handlers::document_open,
            ),
        )
        .handler(NotificationHandler::<
            notification::DidChangeTextDocument,
            _,
            _,
        >::new(handlers::document_change))
        .handler(NotificationHandler::<
            notification::DidCloseTextDocument,
            _,
            _,
        >::new(handlers::document_close))
        .handler(NotificationHandler::<
            notification::DidChangeConfiguration,
            _,
            _,
        >::new(handlers::configuration_change))
        .handler(RequestHandler::<request::SemanticTokensRequest, _, _>::new(
            handlers::semantic_tokens,
        ))
        .handler(RequestHandler::<request::FoldingRangeRequest, _, _>::new(
            handlers::folding_ranges,
        ))
        .handler(RequestHandler::<request::DocumentSymbolRequest, _, _>::new(
            handlers::document_symbols,
        ))
        .handler(RequestHandler::<request::Formatting, _, _>::new(
            handlers::format,
        ))
        .handler(RequestHandler::<request_ext::TomlToJsonRequest, _, _>::new(
            handlers::toml_to_json,
        ))
        .handler(RequestHandler::<request_ext::SyntaxTreeRequest, _, _>::new(
            handlers::syntax_tree,
        ))
        .handler(RequestHandler::<request::Completion, _, _>::new(
            handlers::completion,
        ))
        .handler(RequestHandler::<request::HoverRequest, _, _>::new(
            handlers::hover,
        ))
        .handler(RequestHandler::<request::DocumentLinkRequest, _, _>::new(
            handlers::links,
        ))
        .handler(
            NotificationHandler::<request_ext::ConfigFileChanged, _, _>::new(
                handlers::config_file_changed,
            ),
        )
        .request_writer(RequestWriter)
        .build()
});

struct RequestWriter;

impl Sink<Message> for RequestWriter {
    type Error = io::Error;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: std::pin::Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        // log_debug!("request: {}", serde_json::to_string(&item).unwrap());
        // unsafe: Extern JS call
        unsafe { send_message(JsValue::from_serde(&item).unwrap()) };
        Ok(())
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

struct ResWriter;

#[async_trait]
impl ResponseWriter for ResWriter {
    async fn write_response<R: Serialize + Send + Sync>(
        mut self,
        response: &rpc::Response<R>,
    ) -> Result<(), io::Error> {
        // log_debug!("response: {}", serde_json::to_string(&response).unwrap());
        // unsafe: Extern JS call
        unsafe { send_message(JsValue::from_serde(response).unwrap()) };
        Ok(())
    }
}
