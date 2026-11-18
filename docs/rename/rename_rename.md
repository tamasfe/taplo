# rename.rename

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\rename.rs` (lines 70-161)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn rename<E: Environment>(
    context: Context<World<E>>,
    params: Params<RenameParams>,
) -> Result<Option<WorkspaceEdit>, Error> {
    let p = params.required()?;
    let document_uri = p.text_document_position.text_document.uri;

    let workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document(&document_uri);
    let doc = match ws.document(&document_uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    let position = p.text_document_position.position;
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
        None => match query.after.clone().and_then(|p| {
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

    let mut rewrite = Rewrite::new(doc.dom.clone()).unwrap();

    let keys = match &position_info.dom_node {
        Some(d) => &d.0,
        None => return Ok(None),
    };

    let mut keys = keys.clone();

    if let Some(header_key) = query.header_key() {
        let key_idx = header_key
            .descendants_with_tokens()
            .filter(|t| t.kind() == SyntaxKind::IDENT)
            .position(|t| t.as_token().unwrap() == &position_info.syntax)
            .unwrap();

        keys = lookup_keys(
            doc.dom.clone(),
            &Keys::new(keys.into_iter().take(key_idx + 1)),
        );
    }

    // We're interested in the array itself, not its item type.
    if let Some(KeyOrIndex::Index(_)) = keys.iter().last() {
        keys = keys.skip_right(1);
    }

    rewrite.rename_keys(keys.dotted(), &p.new_name).unwrap();

    Ok(Some(WorkspaceEdit {
        changes: Some(HashMap::from([(
            document_uri,
            rewrite
                .patches()
                .iter()
                .filter_map(|patch| match &patch.kind {
                    taplo::dom::rewrite::PendingPatchKind::Replace(replace) => Some(TextEdit {
                        range: doc.mapper.range(patch.range).unwrap().into_lsp(),
                        new_text: replace.to_string(),
                    }),
                    _ => None,
                })
                .collect(),
        )])),
        ..Default::default()
    }))
}
```

## Docstring

_No docstring provided._

## Source

```
pub async fn rename<E: Environment>(
    context: Context<World<E>>,
    params: Params<RenameParams>,
) -> Result<Option<WorkspaceEdit>, Error> {
    let p = params.required()?;
    let document_uri = p.text_document_position.text_document.uri;

    let workspaces = context.workspaces.write().await;
    let ws = workspaces.by_document(&document_uri);
    let doc = match ws.document(&document_uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    let position = p.text_document_position.position;
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
        None => match query.after.clone().and_then(|p| {
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

    let mut rewrite = Rewrite::new(doc.dom.clone()).unwrap();

    let keys = match &position_info.dom_node {
        Some(d) => &d.0,
        None => return Ok(None),
    };

    let mut keys = keys.clone();

    if let Some(header_key) = query.header_key() {
        let key_idx = header_key
            .descendants_with_tokens()
            .filter(|t| t.kind() == SyntaxKind::IDENT)
            .position(|t| t.as_token().unwrap() == &position_info.syntax)
            .unwrap();

        keys = lookup_keys(
            doc.dom.clone(),
            &Keys::new(keys.into_iter().take(key_idx + 1)),
        );
    }

    // We're interested in the array itself, not its item type.
    if let Some(KeyOrIndex::Index(_)) = keys.iter().last() {
        keys = keys.skip_right(1);
    }

    rewrite.rename_keys(keys.dotted(), &p.new_name).unwrap();

    Ok(Some(WorkspaceEdit {
        changes: Some(HashMap::from([(
            document_uri,
            rewrite
                .patches()
                .iter()
                .filter_map(|patch| match &patch.kind {
                    taplo::dom::rewrite::PendingPatchKind::Replace(replace) => Some(TextEdit {
                        range: doc.mapper.range(patch.range).unwrap().into_lsp(),
                        new_text: replace.to_string(),
                    }),
                    _ => None,
                })
                .collect(),
        )])),
        ..Default::default()
    }))
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
