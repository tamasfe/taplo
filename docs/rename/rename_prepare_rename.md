# rename.prepare_rename

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\rename.rs` (lines 16-67)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn prepare_rename<E: Environment>(
    context: Context<World<E>>,
    params: Params<TextDocumentPositionParams>,
) -> Result<Option<PrepareRenameResponse>, Error> {
    let p = params.required()?;
    let document_uri = p.text_document.uri;

    let workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document(&document_uri);
    let doc = match ws.document(&document_uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    let position = p.position;
    let Some(offset) = doc.mapper.offset(Position::from_lsp(position)) else {
        tracing::error!(?position, "document position not found");
        return Ok(None);
    };

    let query = Query::at(&doc.dom, offset);

    let position_info = match query.before.clone().and_then(|p| {
        if p.syntax.kind() == SyntaxKind::IDENT {
            Some(p)
        } else {
            None
        }
    }) {
        Some(before) => before,
        None => match query.after.and_then(|p| {
            if p.syntax.kind() == SyntaxKind::IDENT {
                Some(p)
            } else {
                None
            }
        }) {
            Some(after) => after,
            None => return Ok(None),
        },
    };

    Ok(Some(PrepareRenameResponse::Range(
        doc.mapper
            .range(position_info.syntax.text_range())
            .unwrap()
            .into_lsp(),
    )))
}
```

## Docstring

_No docstring provided._

## Source

```
pub async fn prepare_rename<E: Environment>(
    context: Context<World<E>>,
    params: Params<TextDocumentPositionParams>,
) -> Result<Option<PrepareRenameResponse>, Error> {
    let p = params.required()?;
    let document_uri = p.text_document.uri;

    let workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document(&document_uri);
    let doc = match ws.document(&document_uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    let position = p.position;
    let Some(offset) = doc.mapper.offset(Position::from_lsp(position)) else {
        tracing::error!(?position, "document position not found");
        return Ok(None);
    };

    let query = Query::at(&doc.dom, offset);

    let position_info = match query.before.clone().and_then(|p| {
        if p.syntax.kind() == SyntaxKind::IDENT {
            Some(p)
        } else {
            None
        }
    }) {
        Some(before) => before,
        None => match query.after.and_then(|p| {
            if p.syntax.kind() == SyntaxKind::IDENT {
                Some(p)
            } else {
                None
            }
        }) {
            Some(after) => after,
            None => return Ok(None),
        },
    };

    Ok(Some(PrepareRenameResponse::Range(
        doc.mapper
            .range(position_info.syntax.text_range())
            .unwrap()
            .into_lsp(),
    )))
}
```

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
