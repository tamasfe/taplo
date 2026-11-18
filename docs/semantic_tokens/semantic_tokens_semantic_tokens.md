# semantic_tokens.semantic_tokens

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\semantic_tokens.rs` (lines 22-47)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn semantic_tokens<E: Environment>(
    context: Context<World<E>>,
    params: Params<SemanticTokensParams>,
) -> Result<Option<SemanticTokensResult>, Error> {
    let p = params.required()?;

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&p.text_document.uri);

    if !ws.config.syntax.semantic_tokens {
        return Ok(None);
    }

    let doc = match ws.document(&p.text_document.uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: create_tokens(doc.dom.syntax().unwrap().as_node().unwrap(), &doc.mapper),
    })))
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) async fn semantic_tokens<E: Environment>(
    context: Context<World<E>>,
    params: Params<SemanticTokensParams>,
) -> Result<Option<SemanticTokensResult>, Error> {
    let p = params.required()?;

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&p.text_document.uri);

    if !ws.config.syntax.semantic_tokens {
        return Ok(None);
    }

    let doc = match ws.document(&p.text_document.uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: create_tokens(doc.dom.syntax().unwrap().as_node().unwrap(), &doc.mapper),
    })))
}
```

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 1
- Features: 2

## Callers

None

## Callees

None
