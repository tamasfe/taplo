#![allow(unused)]
use lsp_async_stub::{
    rpc::Error,
    util::{LspExt, Position},
    Context, Params,
};
use lsp_types::{CodeAction, CodeActionParams, CodeActionResponse};
use taplo::dom::{node::TableKind, Keys, Node};
use taplo_common::environment::Environment;

use crate::{query::Query, world::World};

pub async fn code_action<E: Environment>(
    context: Context<World<E>>,
    params: Params<CodeActionParams>,
) -> Result<Option<CodeActionResponse>, Error> {
    let params = params.required()?;
    let document_uri = &params.text_document.uri;

    #[cfg(feature = "cargo_toml")]
    {
        if !document_uri.path().ends_with("Cargo.toml") {
            return Ok(None);
        }

        let workspaces = context.workspaces.read().await;
        let ws = workspaces.by_document(document_uri);

        let doc = match ws.document(document_uri) {
            Ok(d) => d,
            Err(error) => {
                tracing::debug!(%error, "failed to get document from workspace");
                return Ok(None);
            }
        };

        let position = params.range.start;
        let Some(offset) = doc.mapper.offset(Position::from_lsp(position)) else {
            tracing::error!(?position, "document position not found");
            return Ok(None);
        };

        let query = Query::at(&doc.dom, offset);

        let (path, node) = query
            .dom_node()
            .cloned()
            .or_else(|| {
                doc.dom
                    .flat_iter()
                    .rev()
                    .find(|n| matches!(&n.1, Node::Table(t) if t.kind() == TableKind::Regular))
            })
            .unwrap_or_else(|| (Keys::empty(), doc.dom.clone()));

        return super::cargo::code_action(params, path, node, doc).await;
    }
    return Ok(None);
}
