# folding_ranges.folding_ranges

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\folding_ranges.rs` (lines 22-42)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn folding_ranges<E: Environment>(
    context: Context<World<E>>,
    params: Params<FoldingRangeParams>,
) -> Result<Option<Vec<FoldingRange>>, Error> {
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

    Ok(Some(create_folding_ranges(
        doc.dom.syntax().unwrap().as_node().unwrap(),
        &doc.mapper,
    )))
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) async fn folding_ranges<E: Environment>(
    context: Context<World<E>>,
    params: Params<FoldingRangeParams>,
) -> Result<Option<Vec<FoldingRange>>, Error> {
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

    Ok(Some(create_folding_ranges(
        doc.dom.syntax().unwrap().as_node().unwrap(),
        &doc.mapper,
    )))
}
```

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
