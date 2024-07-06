use crate::query::{lookup_keys, Query};
use crate::world::World;
use lsp_async_stub::rpc::Error;
use lsp_async_stub::util::{LspExt, Position};
use lsp_async_stub::{Context, Params};
use lsp_types::{
    PrepareRenameResponse, RenameParams, TextDocumentPositionParams, TextEdit, WorkspaceEdit,
};
use std::collections::HashMap;
use taplo::dom::rewrite::Rewrite;
use taplo::dom::{KeyOrIndex, Keys};
use taplo::syntax::SyntaxKind;
use taplo_common::environment::Environment;

#[tracing::instrument(skip_all)]
pub async fn prepare_rename<E: Environment>(
    context: Context<World<E>>,
    params: Params<TextDocumentPositionParams>,
) -> Result<Option<PrepareRenameResponse>, Error> {
    let p = params.required()?;
    let document_uri = p.text_document.uri;

    let workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document(&document_uri);
    let doc = match ws.document(&document_uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    let position = p.position;
    let Some(offset) = doc.mapper.offset(Position::from_lsp(position)) else {
        tracing::error!(?position, "document position not found");
        return Ok(None);
    };

    let query = Query::at(&doc.dom, offset);

    let position_info = match query.before.clone().and_then(|p| {
        if p.syntax.kind() == SyntaxKind::IDENT {
            Some(p)
        } else {
            None
        }
    }) {
        Some(before) => before,
        None => match query.after.and_then(|p| {
            if p.syntax.kind() == SyntaxKind::IDENT {
                Some(p)
            } else {
                None
            }
        }) {
            Some(after) => after,
            None => return Ok(None),
        },
    };

    Ok(Some(PrepareRenameResponse::Range(
        doc.mapper
            .range(position_info.syntax.text_range())
            .unwrap()
            .into_lsp(),
    )))
}

#[tracing::instrument(skip_all)]
pub async fn rename<E: Environment>(
    context: Context<World<E>>,
    params: Params<RenameParams>,
) -> Result<Option<WorkspaceEdit>, Error> {
    let p = params.required()?;
    let document_uri = p.text_document_position.text_document.uri;

    let workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document(&document_uri);
    let doc = match ws.document(&document_uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    let position = p.text_document_position.position;
    let Some(offset) = doc.mapper.offset(Position::from_lsp(position)) else {
        tracing::error!(?position, "document position not found");
        return Ok(None);
    };

    let query = Query::at(&doc.dom, offset);

    let position_info = match query.before.clone().and_then(|p| {
        if p.syntax.kind() == SyntaxKind::IDENT {
            Some(p)
        } else {
            None
        }
    }) {
        Some(before) => before,
        None => match query.after.clone().and_then(|p| {
            if p.syntax.kind() == SyntaxKind::IDENT {
                Some(p)
            } else {
                None
            }
        }) {
            Some(after) => after,
            None => return Ok(None),
        },
    };

    let mut rewrite = Rewrite::new(doc.dom.clone()).unwrap();

    let keys = match &position_info.dom_node {
        Some(d) => &d.0,
        None => return Ok(None),
    };

    let mut keys = keys.clone();

    if let Some(header_key) = query.header_key() {
        let key_idx = header_key
            .descendants_with_tokens()
            .filter(|t| t.kind() == SyntaxKind::IDENT)
            .position(|t| t.as_token().unwrap() == &position_info.syntax)
            .unwrap();

        keys = lookup_keys(
            doc.dom.clone(),
            &Keys::new(keys.into_iter().take(key_idx + 1)),
        );
    }

    // We're interested in the array itself, not its item type.
    if let Some(KeyOrIndex::Index(_)) = keys.iter().last() {
        keys = keys.skip_right(1);
    }

    rewrite.rename_keys(keys.dotted(), &p.new_name).unwrap();

    Ok(Some(WorkspaceEdit {
        changes: Some(HashMap::from([(
            document_uri,
            rewrite
                .patches()
                .iter()
                .filter_map(|patch| match &patch.kind {
                    taplo::dom::rewrite::PendingPatchKind::Replace(replace) => Some(TextEdit {
                        range: doc.mapper.range(patch.range).unwrap().into_lsp(),
                        new_text: replace.to_string(),
                    }),
                    _ => None,
                })
                .collect(),
        )])),
        ..Default::default()
    }))
}
