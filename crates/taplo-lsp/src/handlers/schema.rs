use crate::{
    diagnostics::publish_diagnostics,
    lsp_ext::{
        notification::{self, AssociateSchemaParams},
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
