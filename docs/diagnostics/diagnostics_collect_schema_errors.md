# diagnostics.collect_schema_errors

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\diagnostics.rs` (lines 281-324)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn collect_schema_errors<E: Environment>(
    ws: &WorkspaceState<E>,
    doc: &DocumentState,
    dom: &Node,
    document_url: &Url,
    diags: &mut Vec<Diagnostic>,
) {
    if !ws.config.schema.enabled {
        return;
    }

    if let Some(schema_association) = ws.schemas.associations().association_for(document_url) {
        tracing::debug!(
            schema.url = %schema_association.url,
            schema.name = schema_association.meta["name"].as_str().unwrap_or(""),
            schema.source = schema_association.meta["source"].as_str().unwrap_or(""),
            "using schema"
        );

        let errors = match ws.schemas.validate_root(&schema_association.url, dom).await {
            Ok(errors) => errors,
            Err(error) => {
                tracing::error!(?error, "schema validation failed");
                return;
            }
        };

        for error in errors {
            let range = doc
                .mapper
                .range(error.text_ranges().next().unwrap())
                .unwrap()
                .into_lsp();

            diags.push(Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("Even Better TOML".into()),
                message: error.error.to_string(),
                ..Default::default()
            });
        }
    }
}
```

## Docstring

_No docstring provided._

## Source

```
async fn collect_schema_errors<E: Environment>(
    ws: &WorkspaceState<E>,
    doc: &DocumentState,
    dom: &Node,
    document_url: &Url,
    diags: &mut Vec<Diagnostic>,
) {
    if !ws.config.schema.enabled {
        return;
    }

    if let Some(schema_association) = ws.schemas.associations().association_for(document_url) {
        tracing::debug!(
            schema.url = %schema_association.url,
            schema.name = schema_association.meta["name"].as_str().unwrap_or(""),
            schema.source = schema_association.meta["source"].as_str().unwrap_or(""),
            "using schema"
        );

        let errors = match ws.schemas.validate_root(&schema_association.url, dom).await {
            Ok(errors) => errors,
            Err(error) => {
                tracing::error!(?error, "schema validation failed");
                return;
            }
        };

        for error in errors {
            let range = doc
                .mapper
                .range(error.text_ranges().next().unwrap())
                .unwrap()
                .into_lsp();

            diags.push(Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("Even Better TOML".into()),
                message: error.error.to_string(),
                ..Default::default()
            });
        }
    }
}
```

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
