# Module: formatting

**File:** `\crates\taplo-lsp\src\handlers\formatting.rs`

## Overview

Module `formatting` contains 1 analyzed entities.

```
use lsp_async_stub::{rpc::Error, util::LspExt, Context, Params};
use lsp_types::{DocumentFormattingParams, TextEdit};
use taplo::formatter;
use taplo_common::environment::Environment;

use crate::World;

#[tracing::instrument(skip_all)]
pub(crate) async fn format<E: Environment>(
    context: Context<World<E>>,
    params: Params<DocumentFormattingParams>,
) -> Result<Option<Vec<TextEdit>>, Error> {
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

    let doc_path = context
        .env
        .to_file_path_normalized(&p.text_document.uri)
        .ok_or_else(|| {
            Error::invalid_request().with_data(format!(
                "invalid (non-local) uri for file: {}",
                p.text_document.uri
            ))
        })?;

    let mut format_opts = formatter::Options {
        indent_string: if p.options.insert_spaces {
            " ".repeat(p.options.tab_size as usize)
        } else {
            "\t".into()
        },
        ..Default::default()
    };

    if let Some(v) = p.options.insert_final_newline {
        format_opts.trailing_newline = v;
    }

    format_opts.update_camel(ws.config.formatter.clone());

    ws.taplo_config
        .update_format_options(&doc_path, &mut format_opts);

    let scopes = ws.taplo_config.format_scopes(&doc_path);
    tracing::trace!(
        ?doc_path,
        ?format_opts,
        scopes = ?scopes.clone().collect::<Vec<_>>(),
        all_rules = ?ws.taplo_config.rule,
        matched_rules = ?ws.taplo_config.rules_for(&doc_path).collect::<Vec<_>>(),
    );

    Ok(Some(vec![TextEdit {
        range: doc.mapper.all_range().into_lsp(),
        new_text: taplo::formatter::format_with_path_scop
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `formatting.format` | function | No description |

## Git History

- Commits: 6
- Authors: 2
- Bug fixes: 2
- Features: 3

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
