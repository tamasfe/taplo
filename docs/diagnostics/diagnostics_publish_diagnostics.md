# diagnostics.publish_diagnostics

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\diagnostics.rs` (lines 11-90)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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

    let workspaces = context.workspaces.read().await;
    let Some(ws) = workspaces.get(&ws_url) else {
        tracing::warn!(%document_url, "workspace not found");
        return;
    };
    let Some(doc) = ws.documents.get(&document_url) else {
        return;
    };

    collect_schema_errors(ws, doc, &dom, &document_url, &mut diags).await;
    drop(workspaces);

    context
        .write_notification::<notification::PublishDiagnostics, _>(Some(PublishDiagnosticsParams {
            uri: document_url.clone(),
            diagnostics: diags.clone(),
            version: None,
        }))
        .await
        .unwrap_or_else(|err| tracing::error!("{err}"));
}
```

## Docstring

_No docstring provided._

## Source

```
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

    let workspaces = context.workspaces.read().await;
    let Some(ws) = workspaces.get(&ws_url) else {
        tracing::warn!(%document_url, "workspace not found");
        return;
    };
    let Some(doc) = ws.documents.get(&document_url) else {
        return;
    };

    collect_schema_errors(ws, doc, &dom, &document_url, &mut diags).await;
    drop(workspaces);

    context
        .write_notification::<notification::PublishDiagnostics, _>(Some(PublishDiagnosticsParams {
            uri: document_url.clone(),
            diagnostics: diags.clone(),
            version: None,
        }))
        .await
        .unwrap_or_else(|err| tracing::error!("{err}"));
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
