# documents.document_close

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\documents.rs` (lines 161-177)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn document_close<E: Environment>(
    context: Context<World<E>>,
    params: Params<DidCloseTextDocumentParams>,
) {
    let Some(p) = params.optional() else { return };

    let mut workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document_mut(&p.text_document.uri);

    ws.documents.remove(&p.text_document.uri);
    drop(workspaces);

    context.env.spawn_local(diagnostics::clear_diagnostics(
        context.clone(),
        p.text_document.uri,
    ));
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) async fn document_close<E: Environment>(
    context: Context<World<E>>,
    params: Params<DidCloseTextDocumentParams>,
) {
    let Some(p) = params.optional() else { return };

    let mut workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document_mut(&p.text_document.uri);

    ws.documents.remove(&p.text_document.uri);
    drop(workspaces);

    context.env.spawn_local(diagnostics::clear_diagnostics(
        context.clone(),
        p.text_document.uri,
    ));
}
```

## Git History

- Commits: 4
- Authors: 2
- Bug fixes: 2
- Features: 2

## Callers

None

## Callees

None
