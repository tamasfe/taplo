use crate::Document;
use dom::RootNode;
use lsp_types::Position;
use rowan::{TextRange, TextSize};
use taplo::{
    dom::{self, Common},
    syntax::{SyntaxKind, SyntaxNode},
};

#[derive(Debug, Clone)]
pub enum Key {
    Index(usize),
    Property(String),
}

#[derive(Debug)]
pub struct PositionInfo {
    pub keys: Vec<Key>,
    pub node: Option<dom::Node>,
    pub key_only: bool,
    pub table_header: bool,
    pub table_array_header: bool,
    pub inside_comment: bool,
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
                .unwrap_or(doc.parse.green_node.text_len()),
            dom: doc.parse.clone().into_dom(),
            doc,
            key_only: false,
            table_header: false,
            table_array_header: false,
            inside_comment: false,
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
            if token.kind() == SyntaxKind::COMMENT {
                if token.text_range().contains(info.offset) {
                    info.inside_comment = true;
                    info.not_completable = true;
                    break;
                }
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
            for entry in r.entries().iter() {
                if entry.text_range().contains(info.offset) {
                    info.keys.push(Key::Property(entry.key().full_key_string()));

                    get_position_info(&entry.clone().into(), info);
                    break;
                }
            }
        }
        dom::Node::Table(t) => {
            info.node = Some(t.clone().into());
            for entry in t.entries().iter() {
                if entry.text_range().contains(info.offset) {
                    info.keys.push(Key::Property(entry.key().full_key_string()));

                    get_position_info(&entry.clone().into(), info);
                    break;
                }
            }
        }
        dom::Node::Entry(e) => {
            if e.key().text_range().contains(info.offset) {
                if e.token_eq_text_range().is_some() {
                    info.key_only = true;
                }
                get_position_info(&e.key().clone().into(), info);
            } else if e.value().text_range().contains(info.offset) {
                get_position_info(&e.value().clone().into(), info);
            } else {
                // Everything after the eq is considered a value
                if let Some(eq) = e.token_eq_text_range() {
                    if info.offset >= eq.start() {
                        get_position_info(&e.value().clone().into(), info);
                    } else {
                        info.key_only = true;
                        get_position_info(&e.key().clone().into(), info);
                    }
                } else if e.value().is_valid() {
                    // It's a table header or array of tables header.
                    get_position_info(&e.value().clone().into(), info);
                } else {
                    // There's no eq and it's a regular entry, it's all key only.
                    get_position_info(&e.key().clone().into(), info);
                }
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
            for (i, value) in arr.items().iter().enumerate() {
                if value.text_range().contains(info.offset) {
                    info.keys.push(Key::Index(i));
                    get_position_info(&value.clone().into(), info);
                    break;
                }
            }
        }
    }
}

fn ident_range(node: &dom::Node, offset: TextSize) -> Option<TextRange> {
    if !node.is_valid() {
        return None;
    }

    let syntax = node.syntax();

    let n = match syntax.as_node() {
        Some(n) => n,
        None => return None,
    };

    for tok in n.descendants_with_tokens() {
        match tok.kind() {
            SyntaxKind::IDENT => {
                if tok.text_range().contains(offset) {
                    return Some(tok.text_range());
                }
            }
            _ => {}
        }
    }

    None
}

#[derive(Debug)]
pub struct KeyInfo {
    pub key: dom::KeyNode,
    pub parent_keys: Vec<Key>,
}

pub fn collect_keys(node: &dom::Node, parent_keys: Vec<Key>) -> Vec<KeyInfo> {
    let mut keys = Vec::new();

    match node {
        dom::Node::Root(r) => {
            for entry in r.entries().iter() {
                keys.push(KeyInfo {
                    key: entry.key().clone(),
                    parent_keys: parent_keys.clone(),
                });

                let mut next_keys = parent_keys.clone();
                next_keys.push(Key::Property(entry.key().full_key_string()));
                keys.extend(collect_keys(&entry.value().clone().into(), next_keys));
            }
        }
        dom::Node::Table(t) => {
            for entry in t.entries().iter() {
                keys.push(KeyInfo {
                    key: entry.key().clone(),
                    parent_keys: parent_keys.clone(),
                });

                let mut next_keys = parent_keys.clone();
                next_keys.push(Key::Property(entry.key().full_key_string()));
                keys.extend(collect_keys(&entry.value().clone().into(), next_keys));
            }
        }
        dom::Node::Value(v) => match v {
            dom::ValueNode::Array(arr) => {
                keys.extend(collect_keys(&arr.clone().into(), parent_keys));
            }
            dom::ValueNode::Table(t) => {
                keys.extend(collect_keys(&t.clone().into(), parent_keys));
            }
            _ => {}
        },
        dom::Node::Array(arr) => {
            for (idx, item) in arr.items().iter().enumerate() {
                let mut next_keys = parent_keys.clone();
                next_keys.push(Key::Index(idx));

                keys.extend(collect_keys(&item.clone().into(), next_keys));
            }
        }
        dom::Node::Entry(_) | dom::Node::Key(_) => unimplemented!(),
    }

    keys
}
