# Module: completion

**File:** `\crates\taplo-lsp\src\handlers\completion.rs`

## Overview

Module `completion` contains 6 analyzed entities.

```
use lsp_async_stub::{
    rpc::Error,
    util::{LspExt, Position},
    Context, Params,
};
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse, CompletionTextEdit,
    Documentation, InsertTextFormat, MarkupContent, Range, TextEdit,
};
use serde_json::Value;
use std::borrow::Cow;
use std::fmt::Write as _;
use taplo::dom::{node::TableKind, Keys, Node};
use taplo_common::{
    environment::Environment,
    schema::{ext::schema_ext_of, ValueExt},
};

use crate::{
    query::{lookup_keys, Query},
    world::World,
};

#[tracing::instrument(skip_all)]
pub async fn completion<E: Environment>(
    context: Context<World<E>>,
    params: Params<CompletionParams>,
) -> Result<Option<CompletionResponse>, Error> {
    let p = params.required()?;

    let document_uri = p.text_document_position.text_document.uri;

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&document_uri);

    // All completions are tied to schemas.
    if !ws.config.schema.enabled {
        return Ok(None);
    }

    let doc = match ws.document(&document_uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    let Some(schema_association) = ws.schemas.associations().association_for(&document_uri) else {
        return Ok(None);
    };

    let position = p.text_document_position.position;
    let Some(offset) = doc.mapper.offset(Position::from_lsp(position)) else {
        tracing::error!(?position, "document position not found");
        return Ok(None);
    };

    let query = Query::at(&doc.dom, offset);

    let value = match serde_json::to_value(&doc.dom) {
        Ok(v) => v,
        Err(error) => {
            tracing::warn!(%error, "unable to serialize DOM");
            Value::Null
        }
    };

    if query.in_table_header() {
        let key_count = query.header_keys().len();

        
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `completion.documentation` | function | No description |
| `completion.default_value_snippet` | function | No description |
| `completion.new_entry_snippet` | function | No description |
| `completion.add_value_completions` | function | No description |
| `completion.empty_value_snippet` | function | No description |
| `completion.completion` | function | No description |

## Git History

- Commits: 7
- Authors: 2
- Bug fixes: 3
- Features: 3

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
