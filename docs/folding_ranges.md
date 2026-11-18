# Module: folding_ranges

**File:** `\crates\taplo-lsp\src\handlers\folding_ranges.rs`

## Overview

Module `folding_ranges` contains 2 analyzed entities.

```
#![allow(clippy::cast_possible_truncation)]

use lsp_async_stub::{rpc::Error, util::Mapper, Context, Params};
use lsp_types::{FoldingRange, FoldingRangeKind, FoldingRangeParams};
use taplo::{
    dom::node::DomNode,
    rowan::TextRange,
    syntax::{
        SyntaxElement,
        SyntaxKind::{
            ARRAY, COMMENT, MULTI_LINE_STRING, MULTI_LINE_STRING_LITERAL, NEWLINE,
            TABLE_ARRAY_HEADER, TABLE_HEADER, WHITESPACE,
        },
        SyntaxNode,
    },
};
use taplo_common::environment::Environment;

use crate::world::World;

#[tracing::instrument(skip_all)]
pub(crate) async fn folding_ranges<E: Environment>(
    context: Context<World<E>>,
    params: Params<FoldingRangeParams>,
) -> Result<Option<Vec<FoldingRange>>, Error> {
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

    Ok(Some(create_folding_ranges(
        doc.dom.syntax().unwrap().as_node().unwrap(),
        &doc.mapper,
    )))
}

#[tracing::instrument(skip_all)]
pub fn create_folding_ranges(syntax: &SyntaxNode, mapper: &Mapper) -> Vec<FoldingRange> {
    let mut folding_ranges = Vec::with_capacity(20);

    let mut comments_start: Option<TextRange> = None;
    let mut last_comment: Option<TextRange> = None;
    let mut was_comment: bool = false;

    let mut header_starts: Vec<(String, TextRange)> = Vec::new();

    let mut last_non_header: Option<TextRange> = None;

    for element in syntax.children_with_tokens() {
        let mut is_comment = false;

        match element.kind() {
            TABLE_ARRAY_HEADER | TABLE_HEADER => {
                let key = element
                    .as_node()
                    .unwrap()
                    .first_child()
       
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `folding_ranges.folding_ranges` | function | No description |
| `folding_ranges.create_folding_ranges` | function | No description |

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 0
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
