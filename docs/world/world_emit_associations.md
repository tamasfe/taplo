# world.emit_associations

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\world.rs` (lines 263-291)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) async fn emit_associations(&self, mut context: Context<World<E>>) {
        for document_url in self.documents.keys() {
            if let Some(assoc) = self.schemas.associations().association_for(document_url) {
                if let Err(error) = context
                    .write_notification::<DidChangeSchemaAssociation, _>(Some(
                        DidChangeSchemaAssociationParams {
                            document_uri: document_url.clone(),
                            schema_uri: Some(assoc.url.clone()),
                            meta: Some(assoc.meta.clone()),
                        },
                    ))
                    .await
                {
                    tracing::error!(%error, "failed to write notification");
                }
            } else if let Err(error) = context
                .write_notification::<DidChangeSchemaAssociation, _>(Some(
                    DidChangeSchemaAssociationParams {
                        document_uri: document_url.clone(),
                        schema_uri: None,
                        meta: None,
                    },
                ))
                .await
            {
                tracing::error!(%error, "failed to write notification");
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub(crate) async fn emit_associations(&self, mut context: Context<World<E>>) {
        for document_url in self.documents.keys() {
            if let Some(assoc) = self.schemas.associations().association_for(document_url) {
                if let Err(error) = context
                    .write_notification::<DidChangeSchemaAssociation, _>(Some(
                        DidChangeSchemaAssociationParams {
                            document_uri: document_url.clone(),
                            schema_uri: Some(assoc.url.clone()),
                            meta: Some(assoc.meta.clone()),
                        },
                    ))
                    .await
                {
                    tracing::error!(%error, "failed to write notification");
                }
            } else if let Err(error) = context
                .write_notification::<DidChangeSchemaAssociation, _>(Some(
                    DidChangeSchemaAssociationParams {
                        document_uri: document_url.clone(),
                        schema_uri: None,
                        meta: None,
                    },
                ))
                .await
            {
                tracing::error!(%error, "failed to write notification");
            }
        }
    }
```

## Git History

- Commits: 13
- Authors: 3
- Bug fixes: 4
- Features: 5

## Callers

None

## Callees

None
