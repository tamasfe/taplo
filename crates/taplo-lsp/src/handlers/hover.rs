use crate::{
    query::{lookup_keys, Query},
    world::World,
};
use itertools::Itertools;
use lsp_async_stub::{
    rpc::Error,
    util::{LspExt, Position},
    Context, Params,
};
use lsp_types::{Hover, HoverContents, HoverParams, MarkupContent, MarkupKind};
use serde_json::Value;
use taplo::{
    dom::{KeyOrIndex, Keys},
    syntax::SyntaxKind::{
        self, BOOL, DATE, DATE_TIME_LOCAL, DATE_TIME_OFFSET, IDENT, INTEGER, INTEGER_BIN,
        INTEGER_HEX, INTEGER_OCT, MULTI_LINE_STRING, MULTI_LINE_STRING_LITERAL, STRING,
        STRING_LITERAL, TIME,
    },
};
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
    let doc = match ws.document(&document_uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    let position = p.text_document_position_params.position;
    let Some(offset) = doc.mapper.offset(Position::from_lsp(position)) else {
        tracing::error!(?position, "document position not found");
        return Ok(None);
    };

    let query = Query::at(&doc.dom, offset);

    let position_info = match query.before.clone().and_then(|p| {
        if p.syntax.kind() == IDENT || is_primitive(p.syntax.kind()) {
            Some(p)
        } else {
            None
        }
    }) {
        Some(before) => before,
        None => match query.after.clone().and_then(|p| {
            if p.syntax.kind() == IDENT || is_primitive(p.syntax.kind()) {
                Some(p)
            } else {
                None
            }
        }) {
            Some(after) => after,
            None => return Ok(None),
        },
    };

    if let Some(schema_association) = ws.schemas.associations().association_for(&document_uri) {
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

        let Some((keys, _)) = &position_info.dom_node else {
            return Ok(None);
        };

        let links_in_hover = !ws.config.schema.links;

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

        let Some(node) = doc.dom.path(&keys) else {
            return Ok(None);
        };

        if position_info.syntax.kind() == SyntaxKind::IDENT {
            keys = lookup_keys(doc.dom.clone(), &keys);

            // We're interested in the array itself, not its item type.
            if let Some(KeyOrIndex::Index(_)) = keys.iter().last() {
                keys = keys.skip_right(1);
            }

            let schemas = match ws
                .schemas
                .schemas_at_path(&schema_association.url, &value, &keys)
                .await
            {
                Ok(s) => s,
                Err(error) => {
                    tracing::error!(?error, "schema resolution failed");
                    return Ok(None);
                }
            };

            let content = schemas
                .iter()
                .map(|(_, schema)| {
                    let ext = schema_ext_of(schema).unwrap_or_default();
                    let ext_docs = ext.docs.unwrap_or_default();
                    let ext_links = ext.links.unwrap_or_default();

                    let mut s = String::new();
                    if let Some(docs) = ext_docs.main {
                        s += &docs;
                    } else if let Some(desc) = schema["description"].as_str() {
                        s += desc;
                    }

                    let link_title = schema["title"].as_str().unwrap_or("...");

                    if links_in_hover {
                        if let Some(link) = &ext_links.key {
                            s = format!("[{link_title}]({link})\n\n{s}");
                        }
                    }

                    s
                })
                .join("\n\n");

            if content.is_empty() {
                return Ok(None);
            }

            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: content,
                }),
                range: Some(
                    doc.mapper
                        .range(position_info.syntax.text_range())
                        .unwrap()
                        .into_lsp(),
                ),
            }));
        } else if is_primitive(position_info.syntax.kind()) {
            let schemas = match ws
                .schemas
                .schemas_at_path(&schema_association.url, &value, &keys)
                .await
            {
                Ok(s) => s,
                Err(error) => {
                    tracing::error!(?error, "schema resolution failed");
                    return Ok(None);
                }
            };

            let value = match serde_json::to_value(node) {
                Ok(v) => v,
                Err(error) => {
                    tracing::warn!(%error, "failed to turn DOM into JSON");
                    Value::Null
                }
            };

            let content = schemas
                .iter()
                .map(|(_, schema)| {
                    let ext = schema_ext_of(schema).unwrap_or_default();
                    let ext_docs = ext.docs.unwrap_or_default();
                    let enum_docs = ext_docs.enum_values.unwrap_or_default();

                    let ext_links = ext.links.unwrap_or_default();
                    let enum_links = ext_links.enum_values.unwrap_or_default();

                    if !enum_docs.is_empty() {
                        if let Some(enum_values) = schema["enum"].as_array() {
                            for (idx, val) in enum_values.iter().enumerate() {
                                if val == &value {
                                    if let Some(enum_docs) = enum_docs.get(idx).cloned().flatten() {
                                        if links_in_hover {
                                            let link_title =
                                                schema["title"].as_str().unwrap_or("...");

                                            if let Some(enum_link) =
                                                enum_links.get(idx).and_then(Option::as_ref)
                                            {
                                                return format!(
                                                    "[{link_title}]({enum_link})\n\n{enum_docs}"
                                                );
                                            }
                                        }

                                        return enum_docs;
                                    }
                                }
                            }
                        }
                    }

                    if let (Some(docs), Some(default_value)) =
                        (ext_docs.default_value, schema.get("default"))
                    {
                        if &value == default_value {
                            return docs;
                        }
                    }

                    if let (Some(docs), Some(const_value)) =
                        (ext_docs.const_value, schema.get("const"))
                    {
                        if &value == const_value {
                            return docs;
                        }
                    }

                    if let Some(docs) = ext_docs.main {
                        docs
                    } else if let Some(desc) = schema["description"].as_str() {
                        desc.to_string()
                    } else if let Some(title) = schema["title"].as_str() {
                        title.to_string()
                    } else {
                        String::new()
                    }
                })
                .join("\n");

            if content.is_empty() {
                return Ok(None);
            }

            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: content,
                }),
                range: Some(
                    doc.mapper
                        .range(position_info.syntax.text_range())
                        .unwrap()
                        .into_lsp(),
                ),
            }));
        }
    }

    Ok(None)
}

fn is_primitive(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        BOOL | DATE
            | DATE_TIME_LOCAL
            | DATE_TIME_OFFSET
            | TIME
            | STRING
            | MULTI_LINE_STRING
            | STRING_LITERAL
            | MULTI_LINE_STRING_LITERAL
            | INTEGER
            | INTEGER_HEX
            | INTEGER_OCT
            | INTEGER_BIN
    )
}
