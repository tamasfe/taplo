use crate::{
    diagnostics::publish_diagnostics,
    lsp_ext::{
        notification::AssociateSchemaParams,
        request::{
            AssociatedSchemaParams, AssociatedSchemaResponse, ListSchemasParams,
            ListSchemasResponse, SchemaInfo,
        },
    },
    world::World,
};
use lsp_async_stub::{rpc::Error, Context, Params};
use serde_json::json;
use taplo_common::{
    environment::Environment,
    schema::associations::{priority, source, AssociationRule, SchemaAssociation},
};

#[tracing::instrument(skip_all)]
pub async fn list_schemas<E: Environment>(
    context: Context<World<E>>,
    params: Params<ListSchemasParams>,
) -> Result<ListSchemasResponse, Error> {
    let p = params.required()?;

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&p.document_uri);

    let associations = ws.schemas.associations().read();

    Ok(ListSchemasResponse {
        schemas: associations
            .iter()
            .filter(|(rule, _)| !matches!(rule, AssociationRule::Url(..)))
            .map(|(_, s)| SchemaInfo {
                url: s.url.clone(),
                meta: s.meta.clone(),
            })
            .collect(),
    })
}

#[tracing::instrument(skip_all)]
pub async fn associate_schema<E: Environment>(
    context: Context<World<E>>,
    params: Params<AssociateSchemaParams>,
) {
    let p = match params.required() {
        Ok(p) => p,
        Err(_) => return,
    };

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&p.document_uri);

    ws.schemas
        .associations()
        .retain(|(rule, assoc)| match rule {
            AssociationRule::Url(u) => {
                !(*u == p.document_uri && assoc.meta["source"] == source::MANUAL)
            }
            _ => true,
        });

    ws.schemas.associations().add(
        AssociationRule::Url(p.document_uri.clone()),
        SchemaAssociation {
            priority: priority::MAX,
            url: p.schema_uri,
            meta: {
                let mut meta = p.meta.unwrap_or_default();
                if !meta.is_object() {
                    meta = json!({});
                }

                meta["source"] = source::MANUAL.into();
                meta
            },
        },
    );

    ws.emit_associations(context.clone()).await;
    let ws_root = ws.root.clone();
    drop(workspaces);
    publish_diagnostics(context, ws_root, p.document_uri).await;
}

#[tracing::instrument(skip_all)]
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
