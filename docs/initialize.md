# Module: initialize

**File:** `\crates\taplo-lsp\src\handlers\initialize.rs`

## Overview

Module `initialize` contains 2 analyzed entities.

```
use std::sync::Arc;

use super::{semantic_tokens, update_configuration};
use crate::config::InitConfig;
use crate::world::WorkspaceState;
use crate::World;
use lsp_async_stub::{rpc::Error, Context, Params};
use lsp_types::{
    CompletionOptions, DocumentLinkOptions, FoldingRangeProviderCapability,
    HoverProviderCapability, InitializedParams, OneOf, RenameOptions, SemanticTokensFullOptions,
    SemanticTokensLegend, SemanticTokensOptions, SemanticTokensServerCapabilities,
    ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
    WorkDoneProgressOptions, WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities,
};
use lsp_types::{InitializeParams, InitializeResult};
use taplo_common::environment::Environment;

#[tracing::instrument(skip_all)]
pub async fn initialize<E: Environment>(
    context: Context<World<E>>,
    params: Params<InitializeParams>,
) -> Result<InitializeResult, Error> {
    let p = params.required()?;

    if let Some(init_opts) = p.initialization_options {
        match serde_json::from_value::<InitConfig>(init_opts) {
            Ok(c) => context.init_config.store(Arc::new(c)),
            Err(error) => {
                tracing::error!(%error, "invalid initialization options");
            }
        }
    }

    if let Some(workspaces) = p.workspace_folders {
        let mut wss = context.workspaces.write().await;
        let init_config = context.init_config.load();

        for workspace in workspaces {
            let ws = wss
                .entry(workspace.uri.clone())
                .or_insert(WorkspaceState::new(context.env.clone(), workspace.uri));

            ws.schemas
                .cache()
                .set_cache_path(init_config.cache_path.clone());

            if let Err(error) = ws.initialize(context.clone(), &context.env).await {
                tracing::error!(?error, "failed to initialize workspace");
            }
        }
    }

    Ok(InitializeResult {
        capabilitie
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `initialize.initialize` | function | No description |
| `initialize.initialized` | function | No description |

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 1
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
