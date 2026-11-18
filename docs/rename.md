# Module: rename

**File:** `\crates\taplo-lsp\src\handlers\rename.rs`

## Overview

Module `rename` contains 2 analyzed entities.

```
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
            .int
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `rename.prepare_rename` | function | No description |
| `rename.rename` | function | No description |

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 0
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
