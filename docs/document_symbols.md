# Module: document_symbols

**File:** `\crates\taplo-lsp\src\handlers\document_symbols.rs`

## Overview

Module `document_symbols` contains 4 analyzed entities.

```
use crate::world::{DocumentState, World};
use lsp_async_stub::{
    rpc::Error,
    util::{LspExt, Mapper},
    Context, Params,
};
use lsp_types::{DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, SymbolKind};
use taplo::{dom::Node, rowan::TextRange, util::join_ranges};
use taplo_common::environment::Environment;

#[tracing::instrument(skip_all)]
pub(crate) async fn document_symbols<E: Environment>(
    context: Context<World<E>>,
    params: Params<DocumentSymbolParams>,
) -> Result<Option<DocumentSymbolResponse>, Error> {
    let p = params.required()?;

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&p.text_document.uri);
    let doc = match ws.document(&p.text_document.uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    Ok(Some(DocumentSymbolResponse::Nested(create_symbols(doc))))
}

pub(crate) fn create_symbols(doc: &DocumentState) -> Vec<DocumentSymbol> {
    let mapper = &doc.mapper;
    let mut symbols: Vec<DocumentSymbol> = Vec::new();

    let dom = doc.dom.clone();

    let root_table = dom.as_table().unwrap();
    let entries = root_table.entries().read();

    for (key, entry) in entries.iter() {
        symbols_for_value(
            ensure_non_empty_key(key.value().to_string()),
            None,
            entry,
            mapper,
            &mut symbols,
        );
    }

    symbols
}

#[allow(deprecated)]
fn symbols_for_value(
    name: String,
    key_range: Option<TextRange>,
    node: &Node,
    mapper: &Mapper,
    symbols: &mut Vec<DocumentSymbol>,
) {
    let own_range = mapper.range(join_ranges(node.text_ranges(true))).unwrap();

    let range = if let Some(key_r) = key_range {
        mapper
            .range(key_r.cover(join_ranges(node.text_ranges(true))))
            .unwrap()
    } else {
        own_range
    };

    let selection_range = key_range.
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `document_symbols.document_symbols` | function | No description |
| `document_symbols.symbols_for_value` | function | No description |
| `document_symbols.create_symbols` | function | No description |
| `document_symbols.ensure_non_empty_key` | function | No description |

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 0
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
