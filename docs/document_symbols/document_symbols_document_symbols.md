# document_symbols.document_symbols

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\document_symbols.rs` (lines 12-29)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn document_symbols<E: Environment>(
    context: Context<World<E>>,
    params: Params<DocumentSymbolParams>,
) -> Result<Option<DocumentSymbolResponse>, Error> {
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

    Ok(Some(DocumentSymbolResponse::Nested(create_symbols(doc))))
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) async fn document_symbols<E: Environment>(
    context: Context<World<E>>,
    params: Params<DocumentSymbolParams>,
) -> Result<Option<DocumentSymbolResponse>, Error> {
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

    Ok(Some(DocumentSymbolResponse::Nested(create_symbols(doc))))
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
