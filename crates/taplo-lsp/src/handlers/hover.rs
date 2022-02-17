use crate::{query::Query, world::World};
use itertools::Itertools;
use lsp_async_stub::{
    rpc::Error,
    util::{LspExt, Position},
    Context, Params,
};
use lsp_types::{Hover, HoverContents, HoverParams, MarkupContent, MarkupKind};
use taplo::syntax::SyntaxKind;
use taplo_common::{environment::Environment, schema::ext::schema_ext_of};

#[tracing::instrument(skip_all)]
pub(crate) async fn hover<E: Environment>(
    context: Context<World<E>>,
    params: Params<HoverParams>,
) -> Result<Option<Hover>, Error> {
    let p = params.required()?;

    let document_uri = p.text_document_position_params.text_document.uri;

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&document_uri);
    let doc = ws.document(&document_uri)?;

    let position = p.text_document_position_params.position;
    let offset = match doc.mapper.offset(Position::from_lsp(position)) {
        Some(ofs) => ofs,
        None => {
            tracing::error!(?position, "document position not found");
            return Ok(None);
        }
    };

    let query = Query::at(&doc.dom, offset);

    let position_info = match query.before.or(query.after) {
        Some(p) => p,
        None => {
            return Ok(None);
        }
    };

    if position_info.syntax.kind() != SyntaxKind::IDENT {
        return Ok(None);
    }

    if let Some(schema_association) = ws
        .schemas
        .associations()
        .association_for(document_uri.as_str())
    {
        tracing::debug!(
            schema.url = %schema_association.url,
            schema.name = schema_association.meta["name"].as_str().unwrap_or(""),
            schema.source = schema_association.meta["source"].as_str().unwrap_or(""),
            "using schema"
        );

        let value = match serde_json::to_value(&doc.dom) {
            Ok(v) => v,
            Err(error) => {
                tracing::warn!(%error, "cannot turn DOM into JSON");
                return Ok(None);
            }
        };

        // TODO
        // match ws
        //     .schemas
        //     .schemas_at_path(&schema_association.url, &value, &position_info.dom_node.0)
        //     .await
        // {
        //     Ok(schemas) => {
        //         let content = schemas
        //             .iter()
        //             .map(|(_, schema)| {
        //                 let mut s = String::new();
        //                 if let Some(ext) = schema_ext_of(&*schema) {
        //                     if let Some(link) = ext.links.and_then(|l| l.key) {
        //                         s += "*[more information](";
        //                         s += &link;
        //                         s += ")*\n\n";
        //                     }

        //                     if let Some(docs) = ext.docs.and_then(|d| d.main) {
        //                         s += &docs;
        //                     } else if let Some(desc) = schema["description"].as_str() {
        //                         s += desc;
        //                     }
        //                 } else if let Some(desc) = schema["description"].as_str() {
        //                     s += desc;
        //                 }

        //                 s
        //             })
        //             .join("\n");

        //         if content.is_empty() {
        //             return Ok(None);
        //         }

        //         return Ok(Some(Hover {
        //             contents: HoverContents::Markup(MarkupContent {
        //                 kind: MarkupKind::Markdown,
        //                 value: content,
        //             }),
        //             range: Some(
        //                 doc.mapper
        //                     .range(position_info.syntax.text_range())
        //                     .unwrap()
        //                     .into_lsp(),
        //             ),
        //         }));
        //     }
        //     Err(error) => {
        //         tracing::warn!(?error, "schema resolution failed");
        //     }
        // }
    }

    Ok(None)
}
