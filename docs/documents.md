# Module: documents

**File:** `\crates\taplo-lsp\src\handlers\documents.rs`

## Overview

Module `documents` contains 4 analyzed entities.

```
use lsp_async_stub::{util::Mapper, Context, Params, RequestWriter};
use lsp_types::{
    notification, Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
    PublishDiagnosticsParams,
};
use taplo_common::{
    environment::Environment,
    schema::associations::{source, AssociationRule},
};

use crate::{
    diagnostics,
    world::{DocumentState, World},
};

#[tracing::instrument(skip_all)]
pub(crate) async fn document_open<E: Environment>(
    mut context: Context<World<E>>,
    params: Params<DidOpenTextDocumentParams>,
) {
    let Some(p) = params.optional() else { return };

    let mut workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document_mut(&p.text_document.uri);

    if let Some(pth) = context.env.to_file_path_normalized(&p.text_document.uri) {
        if !ws.taplo_config.is_included(&pth) {
            drop(workspaces);
            context
                .write_notification::<notification::PublishDiagnostics, _>(Some(
                    PublishDiagnosticsParams {
                        uri: p.text_document.uri.clone(),
                        diagnostics: vec![Diagnostic {
                            range: Default::default(),
                            severity: Some(DiagnosticSeverity::HINT),
                            code: None,
                            code_description: None,
                            source: Some("Even Better TOML".into()),
                            message: "this document has been excluded".into(),
                            related_information: None,
                            tags: None,
                            data: None,
                        }],
                        version: None,
                    },
                ))
                .await
                .unwrap_or_else(|err| tracing::error!("{err}"));
            return;
        }
    }

    let parse = taplo::parser::parse(
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `documents.document_open` | function | No description |
| `documents.document_save` | function | No description |
| `documents.document_close` | function | No description |
| `documents.document_change` | function | No description |

## Git History

- Commits: 4
- Authors: 2
- Bug fixes: 2
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
