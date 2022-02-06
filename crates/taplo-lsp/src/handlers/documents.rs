use lsp_async_stub::{Context, Params, util::Mapper};
use lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
};
use taplo_common::environment::Environment;

use crate::{
    diagnostics,
    world::{DocumentState, World},
};

#[tracing::instrument(level = "debug", skip_all)]
pub(crate) async fn document_open<E: Environment>(
    context: Context<World<E>>,
    params: Params<DidOpenTextDocumentParams>,
) {
    let p = match params.optional() {
        None => return,
        Some(p) => p,
    };

    let parse = taplo::parser::parse(&p.text_document.text);
    let mapper = Mapper::new_utf16(&p.text_document.text, false);

    let mut workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document_mut(&p.text_document.uri);

    ws.documents
        .insert(p.text_document.uri.clone(), DocumentState { parse, mapper });

    let ws_root = ws.root.clone();
    drop(workspaces);

    context.env.spawn_local(diagnostics::publish_diagnostics(
        context.clone(),
        ws_root,
        p.text_document.uri,
    ));
}

#[tracing::instrument(level = "debug", skip_all)]
pub(crate) async fn document_change<E: Environment>(
    context: Context<World<E>>,
    params: Params<DidChangeTextDocumentParams>,
) {
    let mut p = match params.optional() {
        None => return,
        Some(p) => p,
    };

    // We expect one full change
    let change = match p.content_changes.pop() {
        None => return,
        Some(c) => c,
    };

    let parse = taplo::parser::parse(&change.text);
    let mapper = Mapper::new_utf16(&change.text, false);

    let mut workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document_mut(&p.text_document.uri);

    ws.documents
        .insert(p.text_document.uri.clone(), DocumentState { parse, mapper });

    let ws_root = ws.root.clone();
    drop(workspaces);

    context.env.spawn_local(diagnostics::publish_diagnostics(
        context.clone(),
        ws_root,
        p.text_document.uri,
    ));
}

#[tracing::instrument(level = "debug", skip_all)]
pub(crate) async fn document_close<E: Environment>(
    context: Context<World<E>>,
    params: Params<DidCloseTextDocumentParams>,
) {
    let p = match params.optional() {
        None => return,
        Some(p) => p,
    };

    let mut workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document_mut(&p.text_document.uri);

    ws.documents.remove(&p.text_document.uri);
    drop(workspaces);

    context.env.spawn_local(diagnostics::clear_diagnostics(
        context.clone(),
        p.text_document.uri,
    ));
}
