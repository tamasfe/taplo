use crate::Document;
use lsp_types::DocumentSymbol;

use lsp_types::SymbolKind;

use taplo::{
    dom::{Common, KeyNode, ValueNode},
    util::coords::Mapper,
};

pub(crate) fn create_symbols(doc: &Document) -> Vec<DocumentSymbol> {
    let mapper = &doc.mapper;
    let mut symbols: Vec<DocumentSymbol> = Vec::new();

    for entry in doc.parse.clone().into_dom().entries().iter() {
        symbols_for_value(
            KeyOrString::Key(entry.key()),
            entry.value(),
            mapper,
            &mut symbols,
        );
    }

    symbols
}

enum KeyOrString<'a> {
    Key(&'a KeyNode),
    String(String),
}

fn symbols_for_value(
    key: KeyOrString,
    value: &ValueNode,
    mapper: &Mapper,
    symbols: &mut Vec<DocumentSymbol>,
) {
    let range = mapper
        .range(match &key {
            KeyOrString::Key(k) => k.text_range().clone().cover(value.text_range()),
            KeyOrString::String(_) => value.text_range(),
        })
        .unwrap();

    let selection_range = mapper
        .range(match &key {
            KeyOrString::Key(k) => k.text_range(),
            KeyOrString::String(_) => value.text_range(),
        })
        .unwrap();

    let name = match key {
        KeyOrString::Key(k) => k
            .keys_str()
            .last()
            .map(|s| s.to_string())
            .map(ensure_non_empty_key)
            .unwrap_or_else(|| String::from("{error}")),
        KeyOrString::String(s) => s,
    };

    match value {
        ValueNode::Bool(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::Boolean,
            range,
            selection_range,
            detail: None,
            deprecated: None,
            children: None,
        }),
        ValueNode::String(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::String,
            range,
            selection_range,
            detail: None,
            deprecated: None,
            children: None,
        }),
        ValueNode::Integer(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::Number,
            range,
            selection_range,
            detail: None,
            deprecated: None,
            children: None,
        }),
        ValueNode::Float(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::Number,
            range,
            selection_range,
            detail: None,
            deprecated: None,
            children: None,
        }),
        ValueNode::Date(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::Field,
            range,
            selection_range,
            detail: None,
            deprecated: None,
            children: None,
        }),
        ValueNode::Array(arr) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::Array,
            range,
            selection_range,
            detail: None,
            deprecated: None,
            children: {
                let mut child_symbols = Vec::with_capacity(arr.items().len());

                for (i, c) in arr.items().iter().enumerate() {
                    symbols_for_value(
                        KeyOrString::String(i.to_string()),
                        c,
                        mapper,
                        &mut child_symbols,
                    );
                }

                Some(child_symbols)
            },
        }),
        ValueNode::Table(t) => {
            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::Object,
                range,
                selection_range,
                detail: None,
                deprecated: None,
                children: {
                    let mut child_symbols = Vec::with_capacity(t.entries().len());

                    for c in t.entries().iter() {
                        symbols_for_value(
                            KeyOrString::Key(c.key()),
                            c.value(),
                            mapper,
                            &mut child_symbols,
                        );
                    }

                    Some(child_symbols)
                },
            });
        }
        _ => {}
    }
}

fn ensure_non_empty_key(s: String) -> String {
    if s.is_empty() {
        r#"''"#.into()
    } else {
        s
    }
}
