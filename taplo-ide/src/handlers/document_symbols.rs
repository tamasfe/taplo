use crate::Document;
use lsp_types::DocumentSymbol;

use lsp_types::SymbolKind;

use taplo::{
    dom::{EntryNode, ValueNode},
    util::coords::Mapper,
};

pub(crate) fn create_symbols(doc: &Document) -> Vec<DocumentSymbol> {
    let mapper = &doc.mapper;
    let mut symbols: Vec<DocumentSymbol> = Vec::new();

    for entry in doc.parse.clone().into_dom().entries().iter() {
        symbols.extend(symbols_for_entry(entry, mapper, None).into_iter());
    }

    symbols
}

fn symbols_for_entry(
    entry: &EntryNode,
    mapper: &Mapper,
    prefix: Option<String>,
) -> Vec<DocumentSymbol> {
    let mut p = prefix.clone().unwrap_or_default();
    if !p.is_empty() {
        p += "."
    }

    let mut symbols = Vec::new();

    match entry.value() {
        ValueNode::Bool(_) => symbols.push(DocumentSymbol {
            name: p + &entry
                .key()
                .keys()
                .into_iter()
                .map(ensure_non_empty_key)
                .last()
                .unwrap(),
            kind: SymbolKind::Boolean,
            range: mapper
                .range(
                    entry
                        .key()
                        .text_range()
                        .clone()
                        .cover(entry.value().text_range()),
                )
                .unwrap(),
            selection_range: mapper.range(entry.key().text_range()).unwrap(),
            detail: None,
            deprecated: None,
            children: None,
        }),
        ValueNode::String(_) => symbols.push(DocumentSymbol {
            name: p + &entry
                .key()
                .keys()
                .into_iter()
                .map(ensure_non_empty_key)
                .last()
                .unwrap(),
            kind: SymbolKind::String,
            range: mapper
                .range(
                    entry
                        .key()
                        .text_range()
                        .clone()
                        .cover(entry.value().text_range()),
                )
                .unwrap(),
            selection_range: mapper.range(entry.key().text_range()).unwrap(),
            detail: None,
            deprecated: None,
            children: None,
        }),
        ValueNode::Integer(_) => symbols.push(DocumentSymbol {
            name: p + &entry
                .key()
                .keys()
                .into_iter()
                .map(ensure_non_empty_key)
                .last()
                .unwrap(),
            kind: SymbolKind::Number,
            range: mapper
                .range(
                    entry
                        .key()
                        .text_range()
                        .clone()
                        .cover(entry.value().text_range()),
                )
                .unwrap(),
            selection_range: mapper.range(entry.key().text_range()).unwrap(),
            detail: None,
            deprecated: None,
            children: None,
        }),
        ValueNode::Float(_) => symbols.push(DocumentSymbol {
            name: p + &entry
                .key()
                .keys()
                .into_iter()
                .map(ensure_non_empty_key)
                .last()
                .unwrap(),
            kind: SymbolKind::Number,
            range: mapper
                .range(
                    entry
                        .key()
                        .text_range()
                        .clone()
                        .cover(entry.value().text_range()),
                )
                .unwrap(),
            selection_range: mapper.range(entry.key().text_range()).unwrap(),
            detail: None,
            deprecated: None,
            children: None,
        }),
        ValueNode::Date(_) => symbols.push(DocumentSymbol {
            name: p + &entry
                .key()
                .keys()
                .into_iter()
                .map(ensure_non_empty_key)
                .last()
                .unwrap(),
            kind: SymbolKind::Field,
            range: mapper
                .range(
                    entry
                        .key()
                        .text_range()
                        .clone()
                        .cover(entry.value().text_range()),
                )
                .unwrap(),
            selection_range: mapper.range(entry.key().text_range()).unwrap(),
            detail: None,
            deprecated: None,
            children: None,
        }),
        ValueNode::Array(_) => symbols.push(DocumentSymbol {
            name: p + &entry
                .key()
                .keys()
                .into_iter()
                .map(ensure_non_empty_key)
                .last()
                .unwrap(),
            kind: SymbolKind::Array,
            range: mapper
                .range(
                    entry
                        .key()
                        .text_range()
                        .clone()
                        .cover(entry.value().text_range()),
                )
                .unwrap(),
            selection_range: mapper.range(entry.key().text_range()).unwrap(),
            detail: None,
            deprecated: None,
            children: None,
        }),
        ValueNode::Table(t) => {
            let range = if t.entries().is_empty() {
                mapper.range(entry.key().text_range()).unwrap()
            } else {
                mapper
                    .range(
                        entry
                            .key()
                            .text_range()
                            .cover(t.entries().iter().last().unwrap().text_range()),
                    )
                    .unwrap()
            };

            let mut child_symbols = Vec::new();

            for c in t.entries().iter() {
                child_symbols.extend(symbols_for_entry(c, mapper, prefix.clone()).into_iter());
            }

            symbols.push(DocumentSymbol {
                name: p + &entry
                    .key()
                    .keys()
                    .into_iter()
                    .map(ensure_non_empty_key)
                    .last()
                    .unwrap(),
                kind: SymbolKind::Object,
                range,
                selection_range: mapper.range(entry.key().text_range()).unwrap(),
                detail: None,
                deprecated: None,
                children: Some(child_symbols),
            });
        }
        _ => {}
    }

    symbols
}

fn ensure_non_empty_key(s: String) -> String {
    if s.is_empty() {
        r#"''"#.into()
    } else {
        s
    }
}
