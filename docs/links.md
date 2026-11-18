# Module: links

**File:** `\crates\taplo-lsp\src\handlers\links.rs`

## Overview

Module `links` contains 1 analyzed entities.

```
use crate::world::World;
use lsp_async_stub::rpc::Error;
use lsp_async_stub::util::LspExt;
use lsp_async_stub::{Context, Params};
use lsp_types::{DocumentLink, DocumentLinkParams, Url};
use taplo::dom::KeyOrIndex;
use taplo_common::environment::Environment;
use taplo_common::schema::ext::schema_ext_of;

#[tracing::instrument(skip_all)]
pub async fn links<E: Environment>(
    context: Context<World<E>>,
    params: Params<DocumentLinkParams>,
) -> Result<Option<Vec<DocumentLink>>, Error> {
    let p = params.required()?;

    let workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document(&p.text_document.uri);

    if !ws.config.schema.enabled || !ws.config.schema.links {
        return Ok(None);
    }

    let doc = match ws.document(&p.text_document.uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    let mut links = Vec::new();

    if let Some(schema_association) = ws
        .schemas
        .associations()
        .association_for(&p.text_document.uri)
    {
        tracing::debug!(
            schema.url = %schema_association.url,
            schema.name = schema_association.meta["name"].as_str().unwrap_or(""),
            schema.source = schema_association.meta["source"].as_str().unwrap_or(""),
            "using schema"
        );

        for (keys, last_key, node) in doc.dom.flat_iter().filter_map(|(k, n)| {
            if let Some(KeyOrIndex::Key(last_key)) = k.iter().last().cloned() {
                Some((k, last_key, n))
            } else {
                None
            }
        }) {
            let value = match serde_json::to_value(&node) {
                Ok(v) => v,
                Err(error) => {
                    tracing::debug!(%error, "invalid TOML value");
                    continue;
                }
            };

            let schemas = match ws
                .schemas
          
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `links.links` | function | No description |

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 0
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
