use crate::Document;
use dom::RootNode;
use lsp_types::Position;
use rowan::{TextRange, TextSize};
use taplo::{
    dom::{self, NodeSyntax},
    syntax::{SyntaxKind, SyntaxNode},
    util::SyntaxExt,
};

#[derive(Debug, Clone)]
pub enum Key {
    Index(usize),
    Property(String),
}

impl core::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Index(i) => i.fmt(f),
            Key::Property(p) => p.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct PositionInfo {
    pub keys: Vec<Key>,
    pub node: Option<dom::Node>,
    pub key_only: bool,
    pub table_header: bool,
    pub table_array_header: bool,
    pub inside_comment: bool,
    pub inside_array: bool,
    pub ident_range: Option<TextRange>,
    pub not_completable: bool,
    pub doc: Document,
    pub dom: RootNode,
    pub position: Position,
    pub offset: TextSize,
}

impl PositionInfo {
    pub fn new(doc: Document, position: Position) -> Self {
        let mut info = PositionInfo {
            offset: doc
                .mapper
                .offset(position)
                .unwrap_or_else(|| doc.parse.green_node.text_len()),
            dom: doc.parse.clone().into_dom(),
            doc,
            key_only: false,
            table_header: false,
            table_array_header: false,
            inside_comment: false,
            inside_array: false,
            node: None,
            not_completable: false,
            ident_range: None,
            keys: Vec::new(),
            position,
        };

        get_position_info(&info.dom.clone().into(), &mut info);

        info.ident_range = info.node.as_ref().and_then(|n| ident_range(n, info.offset));

        // Empty table headers ("[]") and empty table array headers ("[[]]") aren't
        // in the DOM.
        for n in info.doc.parse.clone().into_syntax().descendants() {
            if n.text_range().contains(info.offset)
                && (n.kind() == SyntaxKind::TABLE_HEADER
                    || n.kind() == SyntaxKind::TABLE_ARRAY_HEADER)
            {
                if has_error(&n) {
                    info.not_completable = true;
                }

                if n.kind() == SyntaxKind::TABLE_HEADER {
                    info.table_header = true;

                    // cursor is before the first ident
                    if n.text_range().len() > TextSize::from(2)
                        && info.offset == last_starting_bracket(&n).start()
                    {
                        info.not_completable = true;
                    }
                } else {
                    info.table_array_header = true;

                    // cursor is before the first ident
                    if n.text_range().len() > TextSize::from(4)
                        && info.offset == last_starting_bracket(&n).start()
                    {
                        info.not_completable = true;
                    }
                }

                // No completion outside the headers
                if !inside_header_brackets(&n, info.offset) {
                    info.not_completable = true;
                }

                info.key_only = true;
            }
        }

        for token in info
            .doc
            .parse
            .clone()
            .into_syntax()
            .descendants_with_tokens()
        {
            if token.kind() == SyntaxKind::COMMENT && token.text_range().contains(info.offset) {
                info.inside_comment = true;
                info.not_completable = true;
                break;
            }

            if let SyntaxKind::TABLE_HEADER | SyntaxKind::TABLE_ARRAY_HEADER | SyntaxKind::ENTRY =
                token.kind()
            {
                // Entry at the start of the line
                if token.text_range().start() == info.offset + TextSize::from(1) {
                    info.not_completable = true;
                    break;
                }
            }
        }

        info
    }
}

fn has_error(node: &SyntaxNode) -> bool {
    node.descendants_with_tokens()
        .any(|n| n.kind() == SyntaxKind::ERROR)
}

fn last_starting_bracket(node: &SyntaxNode) -> TextRange {
    node.descendants_with_tokens()
        .filter(|d| d.kind() == SyntaxKind::BRACKET_START)
        .last()
        .map(|n| n.text_range())
        .unwrap_or_default()
}

fn first_ending_bracket(node: &SyntaxNode) -> TextRange {
    node.descendants_with_tokens()
        .find(|d| d.kind() == SyntaxKind::BRACKET_END)
        .map(|n| n.text_range())
        .unwrap_or_default()
}

fn inside_header_brackets(node: &SyntaxNode, offset: TextSize) -> bool {
    let start = last_starting_bracket(node).start();
    let end = first_ending_bracket(node).start();

    offset >= start && offset < end
}

fn get_position_info(node: &dom::Node, info: &mut PositionInfo) {
    match node {
        dom::Node::Root(r) => {
            info.node = Some(r.clone().into());
            for (_, entry) in r.entries().iter() {
                if entry.syntax().text_range().contains(info.offset) {
                    info.keys.push(Key::Property(entry.key().full_key_string()));

                    get_position_info(&entry.clone().into(), info);
                    break;
                }
            }
        }
        dom::Node::Table(t) => {
            info.node = Some(t.clone().into());
            for (_, entry) in t.entries().iter() {
                if entry.syntax().text_range().contains(info.offset) {
                    info.keys.push(Key::Property(entry.key().full_key_string()));

                    get_position_info(&entry.clone().into(), info);
                    break;
                }
            }
        }
        dom::Node::Entry(e) => {
            if e.key()
                .text_ranges()
                .iter()
                .any(|e| e.contains(info.offset))
            {
                // if e.token_eq_text_range().is_some() {
                //     info.key_only = true;
                // }
                get_position_info(&e.key().clone().into(), info);
            } else if e.value().syntax().text_range().contains(info.offset) {
                get_position_info(&e.value().clone().into(), info);
            } else {
                // Everything after the eq is considered a value
                // if let Some(eq) = e.token_eq_text_range() {
                //     if info.offset >= eq.start() {
                //         get_position_info(&e.value().clone().into(), info);
                //     } else {
                //         info.key_only = true;
                //         get_position_info(&e.key().clone().into(), info);
                //     }
                // } else
                get_position_info(&e.key().clone().into(), info);
            }
        }
        dom::Node::Key(k) => {
            info.node = Some(k.clone().into());
        }
        dom::Node::Value(v) => match v {
            dom::ValueNode::Array(a) => {
                get_position_info(&a.clone().into(), info);
            }
            dom::ValueNode::Table(t) => {
                get_position_info(&t.clone().into(), info);
            }
            _ => {
                info.node = Some(v.clone().into());
            }
        },
        dom::Node::Array(arr) => {
            info.node = Some(arr.clone().into());
            let mut value_found = false;
            for (i, value) in arr.items().iter().enumerate() {
                if value.syntax().text_range().contains(info.offset) {
                    info.keys.push(Key::Index(i));
                    get_position_info(&value.clone().into(), info);
                    value_found = true;
                    break;
                }
            }
            if !value_found && !arr.is_array_of_tables() {
                if let Some(t) = arr
                    .syntax()
                    .as_node()
                    .unwrap()
                    .find(SyntaxKind::BRACKET_START)
                {
                    if t.text_range().start() <= info.offset {
                        info.inside_array = true;
                    }
                }
            }
        }
    }
}

fn ident_range(node: &dom::Node, offset: TextSize) -> Option<TextRange> {
    let syntax = node.syntax();

    let n = match syntax.as_node() {
        Some(n) => n,
        None => return None,
    };

    for tok in n.descendants_with_tokens() {
        if let SyntaxKind::IDENT = tok.kind() {
            if tok.text_range().contains(offset) {
                return Some(tok.text_range());
            }
        }
    }

    None
}

#[derive(Debug)]
pub struct KeyInfo {
    pub key: Option<dom::KeyNode>,
    pub parent_keys: Vec<Key>,
    pub value: Option<dom::ValueNode>,
}

pub fn collect_for_schema(node: &dom::Node, parent_keys: Vec<Key>) -> Vec<KeyInfo> {
    let mut keys = Vec::new();

    match node {
        dom::Node::Root(r) => {
            for (_, entry) in r.entries().iter() {
                keys.push(KeyInfo {
                    key: entry.key().clone().into(),
                    parent_keys: parent_keys.clone(),
                    value: entry.value().clone().into(),
                });

                let mut next_keys = parent_keys.clone();
                next_keys.push(Key::Property(entry.key().full_key_string()));
                keys.extend(collect_for_schema(&entry.value().clone().into(), next_keys));
            }
        }
        dom::Node::Table(t) => {
            for (_, entry) in t.entries().iter() {
                keys.push(KeyInfo {
                    key: entry.key().clone().into(),
                    parent_keys: parent_keys.clone(),
                    value: entry.value().clone().into(),
                });

                let mut next_keys = parent_keys.clone();
                next_keys.push(Key::Property(entry.key().full_key_string()));
                keys.extend(collect_for_schema(&entry.value().clone().into(), next_keys));
            }
        }
        dom::Node::Value(v) => match v {
            dom::ValueNode::Array(arr) => {
                keys.extend(collect_for_schema(&arr.clone().into(), parent_keys));
            }
            dom::ValueNode::Table(t) => {
                keys.extend(collect_for_schema(&t.clone().into(), parent_keys));
            }
            _ => {}
        },
        dom::Node::Array(arr) => {
            for (idx, item) in arr.items().iter().enumerate() {
                let mut next_keys = parent_keys.clone();
                next_keys.push(Key::Index(idx));

                keys.push(KeyInfo {
                    key: None,
                    parent_keys: parent_keys.clone(),
                    value: item.clone().into(),
                });

                keys.extend(collect_for_schema(&item.clone().into(), next_keys));
            }
        }
        dom::Node::Entry(_) | dom::Node::Key(_) => unimplemented!(),
    }

    keys
}
