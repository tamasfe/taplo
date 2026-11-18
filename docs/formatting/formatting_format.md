# formatting.format

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\formatting.rs` (lines 9-79)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
        new_text: taplo::formatter::format_with_path_scopes(
            doc.dom.clone(),
            format_opts,
            &doc.parse
                .errors
                .iter()
                .map(|err| err.range)
                .collect::<Vec<_>>(),
            scopes.into_iter(),
        )
        .map_err(|err| {
            tracing::error!(error = %err, "invalid key pattern");
            Error::internal_error().with_data("invalid Taplo configuration")
        })?,
    }]))
}
```

## Docstring

_No docstring provided._

## Source

```
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
        new_text: taplo::formatter::format_with_path_scopes(
            doc.dom.clone(),
            format_opts,
            &doc.parse
                .errors
                .iter()
                .map(|err| err.range)
                .collect::<Vec<_>>(),
            scopes.into_iter(),
        )
        .map_err(|err| {
            tracing::error!(error = %err, "invalid key pattern");
            Error::internal_error().with_data("invalid Taplo configuration")
        })?,
    }]))
}
```

## Git History

- Commits: 6
- Authors: 2
- Bug fixes: 2
- Features: 3

## Callers

None

## Callees

None
