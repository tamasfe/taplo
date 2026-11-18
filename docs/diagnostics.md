# Module: diagnostics

**File:** `\crates\taplo-lsp\src\diagnostics.rs`

## Overview

Module `diagnostics` contains 5 analyzed entities.

```
use crate::world::{DocumentState, WorkspaceState, World};
use lsp_async_stub::{util::LspExt, Context, RequestWriter};
use lsp_types::{
    notification, Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location,
    PublishDiagnosticsParams, Url,
};
use taplo::dom::Node;
use taplo_common::environment::Environment;

#[tracing::instrument(skip_all)]
pub(crate) async fn publish_diagnostics<E: Environment>(
    mut context: Context<World<E>>,
    ws_url: Url,
    document_url: Url,
) {
    let mut diags = Vec::new();

    let workspaces = context.workspaces.read().await;
    let Some(ws) = workspaces.get(&ws_url) else {
        tracing::warn!(%document_url, "workspace not found");
        return;
    };
    let Some(doc) = ws.documents.get(&document_url) else {
        return;
    };

    collect_syntax_errors(doc, &mut diags);
    drop(workspaces);

    context
        .write_notification::<notification::PublishDiagnostics, _>(Some(PublishDiagnosticsParams {
            uri: document_url.clone(),
            diagnostics: diags.clone(),
            version: None,
        }))
        .await
        .unwrap_or_else(|err| tracing::error!("{err}"));

    if !diags.is_empty() {
        return;
    }

    let workspaces = context.workspaces.read().await;
    let Some(ws) = workspaces.get(&ws_url) else {
        tracing::warn!(%document_url, "workspace not found");
        return;
    };
    let Some(doc) = ws.documents.get(&document_url) else {
        return;
    };

    let dom = doc.dom.clone();

    collect_dom_errors(doc, &dom, &document_url, &mut diags);
    drop(workspaces);

    context
        .write_notification::<notification::PublishDiagnostics, _>(Some(PublishDiagnosticsParams {
            uri: document_url.clone(),
            diagnostics: diags.clone(),
            version: None,
        }))
        .await
        .unwrap_or_else(|err| tracing::error!("{err}"));

    if !diags.is_empty() {
        return;
    }

    let workspaces = context.worksp
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `diagnostics.clear_diagnostics` | function | No description |
| `diagnostics.collect_syntax_errors` | function | No description |
| `diagnostics.collect_schema_errors` | function | No description |
| `diagnostics.publish_diagnostics` | function | No description |
| `diagnostics.collect_dom_errors` | function | No description |

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 0
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
