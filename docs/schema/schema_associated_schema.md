# schema.associated_schema

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\schema.rs` (lines 122-141)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn associated_schema<E: Environment>(
    context: Context<World<E>>,
    params: Params<AssociatedSchemaParams>,
) -> Result<AssociatedSchemaResponse, Error> {
    let p = params.required()?;

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&p.document_uri);

    Ok(AssociatedSchemaResponse {
        schema: ws
            .schemas
            .associations()
            .association_for(&p.document_uri)
            .map(|s| SchemaInfo {
                url: s.url,
                meta: s.meta,
            }),
    })
}
```

## Docstring

_No docstring provided._

## Source

```
pub async fn associated_schema<E: Environment>(
    context: Context<World<E>>,
    params: Params<AssociatedSchemaParams>,
) -> Result<AssociatedSchemaResponse, Error> {
    let p = params.required()?;

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&p.document_uri);

    Ok(AssociatedSchemaResponse {
        schema: ws
            .schemas
            .associations()
            .association_for(&p.document_uri)
            .map(|s| SchemaInfo {
                url: s.url,
                meta: s.meta,
            }),
    })
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
