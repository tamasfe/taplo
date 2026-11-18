# Module: hover

**File:** `\crates\taplo-lsp\src\handlers\hover.rs`

## Overview

Module `hover` contains 2 analyzed entities.

```
use crate::{
    query::{lookup_keys, Query},
    world::World,
};
use itertools::Itertools;
use lsp_async_stub::{
    rpc::Error,
    util::{LspExt, Position},
    Context, Params,
};
use lsp_types::{Hover, HoverContents, HoverParams, MarkupContent, MarkupKind};
use serde_json::Value;
use taplo::{
    dom::{KeyOrIndex, Keys},
    syntax::SyntaxKind::{
        self, BOOL, DATE, DATE_TIME_LOCAL, DATE_TIME_OFFSET, IDENT, INTEGER, INTEGER_BIN,
        INTEGER_HEX, INTEGER_OCT, MULTI_LINE_STRING, MULTI_LINE_STRING_LITERAL, STRING,
        STRING_LITERAL, TIME,
    },
};
use taplo_common::{environment::Environment, schema::ext::schema_ext_of};

#[tracing::instrument(skip_all)]
pub(crate) async fn hover<E: Environment>(
    context: Context<World<E>>,
    params: Params<HoverParams>,
) -> Result<Option<Hover>, Error> {
    let p = params.required()?;

    let document_uri = p.text_document_position_params.text_document.uri;

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&document_uri);
    let doc = match ws.document(&document_uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    let position = p.text_document_position_params.position;
    let Some(offset) = doc.mapper.offset(Position::from_lsp(position)) else {
        tracing::error!(?position, "document position not found");
        return Ok(None);
    };

    let query = Query::at(&doc.dom, offset);

    let position_info = match query.before.clone().and_then(|p| {
        if p.syntax.kind() == IDENT || is_primitive(p.syntax.kind()) {
            Some(p)
        } else {
            None
        }
    }) {
        Some(before) => before,
        None => match query.after.clone().and_then(|p| {
            if p.syntax.kind() == IDENT || is_primitive(p.syntax.kind()) {
                Some(p)
            } else {
                None
            }
    
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `hover.hover` | function | No description |
| `hover.is_primitive` | function | No description |

## Git History

- Commits: 7
- Authors: 4
- Bug fixes: 3
- Features: 3

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
