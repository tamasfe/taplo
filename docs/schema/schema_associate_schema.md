# schema.associate_schema

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\schema.rs` (lines 44-119)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn associate_schema<E: Environment>(
    context: Context<World<E>>,
    params: Params<AssociateSchemaParams>,
) {
    let Ok(p) = params.required() else {
        return;
    };

    let workspaces = context.workspaces.read().await;

    let assoc = SchemaAssociation {
        priority: p.priority.unwrap_or(priority::MAX),
        url: p.schema_uri,
        meta: {
            let mut meta = p.meta.unwrap_or_default();
            if !meta.is_object() {
                meta = json!({});
            }

            meta["source"] = source::MANUAL.into();
            meta
        },
    };

    for (_, ws) in workspaces.iter() {
        // FIXME: there is no way to remove these.
        match &p.rule {
            notification::AssociationRule::Glob(glob) => {
                let rule = match AssociationRule::glob(glob) {
                    Ok(re) => re,
                    Err(err) => {
                        tracing::error!(
                        error = %err,
                        schema_uri = %assoc.url,
                        "invalid pattern for schema");
                        return;
                    }
                };

                ws.schemas.associations().add(rule, assoc.clone());
            }
            notification::AssociationRule::Regex(regex) => {
                let rule = match AssociationRule::regex(regex) {
                    Ok(re) => re,
                    Err(err) => {
                        tracing::error!(
                        error = %err,
                        schema_uri = %assoc.url,
                        "invalid pattern for schema");
                        return;
                    }
                };

                ws.schemas.associations().add(rule, assoc.clone());
            }
            notification::AssociationRule::Url(document_uri) => {
                ws.schemas
                    .associations()
                    .retain(|(rule, assoc)| match rule {
                        AssociationRule::Url(u) => {
                            !(u == document_uri && assoc.meta["source"] == source::MANUAL)
                        }
                        _ => true,
                    });

                ws.schemas
                    .associations()
                    .add(AssociationRule::Url(document_uri.clone()), assoc.clone());

                let ws_root = ws.root.clone();
                publish_diagnostics(context.clone(), ws_root, document_uri.clone()).await;
            }
        }
        ws.emit_associations(context.clone()).await;
    }
}
```

## Docstring

_No docstring provided._

## Source

```
pub async fn associate_schema<E: Environment>(
    context: Context<World<E>>,
    params: Params<AssociateSchemaParams>,
) {
    let Ok(p) = params.required() else {
        return;
    };

    let workspaces = context.workspaces.read().await;

    let assoc = SchemaAssociation {
        priority: p.priority.unwrap_or(priority::MAX),
        url: p.schema_uri,
        meta: {
            let mut meta = p.meta.unwrap_or_default();
            if !meta.is_object() {
                meta = json!({});
            }

            meta["source"] = source::MANUAL.into();
            meta
        },
    };

    for (_, ws) in workspaces.iter() {
        // FIXME: there is no way to remove these.
        match &p.rule {
            notification::AssociationRule::Glob(glob) => {
                let rule = match AssociationRule::glob(glob) {
                    Ok(re) => re,
                    Err(err) => {
                        tracing::error!(
                        error = %err,
                        schema_uri = %assoc.url,
                        "invalid pattern for schema");
                        return;
                    }
                };

                ws.schemas.associations().add(rule, assoc.clone());
            }
            notification::AssociationRule::Regex(regex) => {
                let rule = match AssociationRule::regex(regex) {
                    Ok(re) => re,
                    Err(err) => {
                        tracing::error!(
                        error = %err,
                        schema_uri = %assoc.url,
                        "invalid pattern for schema");
                        return;
                    }
                };

                ws.schemas.associations().add(rule, assoc.clone());
            }
            notification::AssociationRule::Url(document_uri) => {
                ws.schemas
                    .associations()
                    .retain(|(rule, assoc)| match rule {
                        AssociationRule::Url(u) => {
                            !(u == document_uri && assoc.meta["source"] == source::MANUAL)
                        }
                        _ => true,
                    });

                ws.schemas
                    .associations()
                    .add(AssociationRule::Url(document_uri.clone()), assoc.clone());

                let ws_root = ws.root.clone();
                publish_diagnostics(context.clone(), ws_root, document_uri.clone()).await;
            }
        }
        ws.emit_associations(context.clone()).await;
    }
}
```

## Git History

- Commits: 3
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
