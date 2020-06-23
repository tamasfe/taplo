use crate::Document;
use dom::RootNode;
use lsp_types::Position;
use rowan::{TextRange, TextSize};
use taplo::{
    dom::{self, Common},
    syntax::{SyntaxKind, SyntaxNode},
};

#[derive(Debug, Clone)]
pub(crate) enum Key {
    Index(usize),
    Property(String),
}

#[derive(Debug)]
pub(crate) struct PositionInfo {
    pub(crate) keys: Vec<Key>,
    pub(crate) node: Option<dom::Node>,
    pub(crate) key_only: bool,
    pub(crate) table_header: bool,
    pub(crate) table_array_header: bool,
    pub(crate) inside_comment: bool,
    pub(crate) ident_range: Option<TextRange>,
    pub(crate) not_completable: bool,
    pub(crate) doc: Document,
    pub(crate) dom: RootNode,
    pub(crate) position: Position,
    pub(crate) offset: TextSize,
}

impl PositionInfo {
    pub(crate) fn new(doc: Document, position: Position) -> Self {
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
