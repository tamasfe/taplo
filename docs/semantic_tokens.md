# Module: semantic_tokens

**File:** `\crates\taplo-lsp\src\handlers\semantic_tokens.rs`

## Overview

Module `semantic_tokens` contains 6 analyzed entities.

```
use crate::World;
use lsp_async_stub::{
    rpc::Error,
    util::{relative_range, LspExt, Mapper},
    Context, Params,
};
use lsp_types::{
    Range, SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens,
    SemanticTokensParams, SemanticTokensResult,
};
use taplo::{
    dom::node::DomNode,
    syntax::{
        SyntaxElement,
        SyntaxKind::{ARRAY, IDENT, INLINE_TABLE},
        SyntaxNode, SyntaxToken,
    },
};
use taplo_common::environment::Environment;

#[tracing::instrument(skip_all)]
pub(crate) async fn semantic_tokens<E: Environment>(
    context: Context<World<E>>,
    params: Params<SemanticTokensParams>,
) -> Result<Option<SemanticTokensResult>, Error> {
    let p = params.required()?;

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&p.text_document.uri);

    if !ws.config.syntax.semantic_tokens {
        return Ok(None);
    }

    let doc = match ws.document(&p.text_document.uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: create_tokens(doc.dom.syntax().unwrap().as_node().unwrap(), &doc.mapper),
    })))
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum TokenType {
    TomlArrayKey,
    TomlTableKey,
}

impl TokenType {
    pub const LEGEND: &'static [SemanticTokenType] = &[
        SemanticTokenType::new("tomlArrayKey"),
        SemanticTokenType::new("tomlTableKey"),
    ];
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum TokenModifier {
    ReadOnly,
}

impl TokenModifier {
    pub const MODIFIERS: &'static [SemanticTokenModifier] = &[SemanticTokenModifier::READONLY];
}

#[tracing::instrument(skip_all)]
pub fn create_tokens(syntax: &SyntaxNode, mapper: &Mapper) -> Vec<SemanticToken> {
    let mut builder = Semant
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `semantic_tokens.build` | function | No description |
| `semantic_tokens.add_token` | function | No description |
| `semantic_tokens.new` | function | No description |
| `semantic_tokens.create_tokens` | function | No description |
| `semantic_tokens.semantic_tokens` | function | No description |
| `semantic_tokens.SemanticTokensBuilder` | struct | No description |

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 1
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
