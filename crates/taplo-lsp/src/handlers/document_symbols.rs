use crate::world::{DocumentState, World};
use lsp_async_stub::{
    rpc::Error,
    util::{LspExt, Mapper},
    Context, Params,
};
use lsp_types::{DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, SymbolKind};
use taplo::{dom::Node, rowan::TextRange, util::join_ranges};
use taplo_common::environment::Environment;

#[tracing::instrument(skip_all)]
pub(crate) async fn document_symbols<E: Environment>(
    context: Context<World<E>>,
    params: Params<DocumentSymbolParams>,
) -> Result<Option<DocumentSymbolResponse>, Error> {
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

    Ok(Some(DocumentSymbolResponse::Nested(create_symbols(doc))))
}

pub(crate) fn create_symbols(doc: &DocumentState) -> Vec<DocumentSymbol> {
    let mapper = &doc.mapper;
    let mut symbols: Vec<DocumentSymbol> = Vec::new();

    let dom = doc.dom.clone();

    let root_table = dom.as_table().unwrap();
    let entries = root_table.entries().read();

    for (key, entry) in entries.iter() {
        symbols_for_value(
            ensure_non_empty_key(key.value().to_string()),
            None,
            entry,
            mapper,
            &mut symbols,
        );
    }

    symbols
}

#[allow(deprecated)]
fn symbols_for_value(
    name: String,
    key_range: Option<TextRange>,
    node: &Node,
    mapper: &Mapper,
    symbols: &mut Vec<DocumentSymbol>,
) {
    let own_range = mapper.range(join_ranges(node.text_ranges(true))).unwrap();

    let range = if let Some(key_r) = key_range {
        mapper
            .range(key_r.cover(join_ranges(node.text_ranges(true))))
            .unwrap()
    } else {
        own_range
    };

    let selection_range = key_range.map_or(own_range, |r| mapper.range(r).unwrap());

    match node {
        Node::Bool(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::BOOLEAN,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        Node::Str(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::STRING,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        Node::Integer(_) | Node::Float(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::NUMBER,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        Node::Date(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::FIELD,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        Node::Array(arr) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::ARRAY,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: {
                let mut child_symbols = Vec::with_capacity(arr.items().read().len());
                let items = arr.items().read();

                for (i, c) in items.iter().enumerate() {
                    symbols_for_value(i.to_string(), None, c, mapper, &mut child_symbols);
                }

                Some(child_symbols)
            },
        }),
        Node::Table(t) => {
            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::OBJECT,
                range: range.into_lsp(),
                selection_range: selection_range.into_lsp(),
                detail: None,
                deprecated: None,
                tags: Default::default(),
                children: {
                    let mut child_symbols = Vec::with_capacity(t.entries().read().len());
                    let entries = t.entries().read();
                    for (key, entry) in entries.iter() {
                        symbols_for_value(
                            ensure_non_empty_key(key.value().to_string()),
                            None,
                            entry,
                            mapper,
                            &mut child_symbols,
                        );
                    }

                    Some(child_symbols)
                },
            });
        }
        Node::Invalid(_) => {}
    }
}

fn ensure_non_empty_key(s: String) -> String {
    if s.is_empty() {
        r"''".into()
    } else {
        s
    }
}
