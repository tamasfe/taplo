use lsp_async_stub::Server;
use lsp_types::{request, notification};
use std::sync::Arc;
use taplo_common::environment::Environment;
use world::{World, WorldState};

mod handlers;
mod msg_ext;
mod diagnostics;

pub mod world;

pub fn create_server<E: Environment>() -> Server<World<E>> {
    Server::new()
        .on_request::<request::Initialize, _>(handlers::initialize)
        .on_request::<request::FoldingRangeRequest, _>(handlers::folding_ranges)
        .on_request::<request::DocumentSymbolRequest, _>(handlers::document_symbols)
        .on_request::<request::Formatting, _>(handlers::format)
        // .on_request::<request::Completion, _>(handlers::completion)
        // .on_request::<request::HoverRequest, _>(handlers::hover)
        // .on_request::<request::DocumentLinkRequest, _>(handlers::links)
        .on_request::<request::SemanticTokensFullRequest, _>(handlers::semantic_tokens)
        // .on_request::<request::CodeActionRequest, _>(handlers::code_action)
        // .on_request::<msg_ext::TomlToJsonRequest, _>(handlers::toml_to_json)
        // .on_request::<msg_ext::JsonToTomlRequest, _>(handlers::json_to_toml)
        .on_notification::<notification::DidOpenTextDocument, _>(handlers::document_open)
        .on_notification::<notification::DidChangeTextDocument, _>(handlers::document_change)
        .on_notification::<notification::DidCloseTextDocument, _>(handlers::document_close)
        // .on_notification::<notification::DidChangeConfiguration, _>(handlers::configuration_change)
        // .on_notification::<msg_ext::CachePath, _>(handlers::cache_path)
        .build()
}

pub fn create_world<E: Environment>(env: E) -> World<E> {
    Arc::new(WorldState::new(env))
}
