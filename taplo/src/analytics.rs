//! This module contains various methods and utilities for analyzing the DOM and/or the syntax tree.

use std::iter::FromIterator;

use crate::{
    dom::{self, NodeSyntax, TextRanges},
    syntax::SyntaxToken,
    syntax::{
        SyntaxElement,
        SyntaxKind::{self, *},
    },
    util::StrExt,
};
use rowan::{TextRange, TextSize};
use smallvec::SmallVec;

macro_rules! impl_is_node_ref {
    ($($method_name:ident() -> $variant:ident;)*) => {
        impl NodeRef<'_> {
            $(
                pub fn $method_name(&self) -> bool {
                    matches!(self, NodeRef::$variant(_))
                }
            )*
        }
    }
}

impl dom::RootNode {
    pub fn query_position(&self, position: TextSize) -> PositionQueryResult {
        PositionQueryResult {
            before: position
                .checked_sub(1.into())
                .map(|position| self.q_position_impl(position)),
            after: self.q_position_impl(position),
        }
    }

    /// Returns an iterator over all the nodes of the DOM.
    pub fn iter<'i>(&'i self) -> impl Iterator<Item = (dom::Path, NodeRef<'i>)> + 'i {
        let mut nodes = SmallVec::with_capacity(1 + self.entries().len());
        collect::collect_root(dom::Path::new(), self, &mut nodes);
        nodes.into_iter()
    }
}

impl dom::RootNode {
    fn q_position_impl(&self, position: TextSize) -> PositionInfo {
        let root_syntax = self.syntax();

        let mut nodes: Vec<NodeRef> = Vec::new();
        let mut path = dom::Path::new();
        for (p, node) in self.iter() {
            match node {
                NodeRef::Root(n) => {
                    if n.syntax().text_range().contains(position) {
                        nodes.push(node);
                    }
                }
                NodeRef::Table(n) => {
                    if n.text_ranges().iter().any(|r| r.contains(position)) {
                        nodes.push(node);
                        path = p;
                    }
                }
                NodeRef::Entry(n) => {
                    if n.text_ranges().iter().any(|r| r.contains(position)) {
                        nodes.push(node);
                    }
                }
                NodeRef::Array(n) => {
                    if n.text_ranges().iter().any(|r| r.contains(position)) {
                        nodes.push(node);
                    }
                }
                NodeRef::Value(n) => {
                    if n.text_ranges().iter().any(|r| r.contains(position)) {
                        nodes.push(node);

                        match n {
                            dom::ValueNode::Array(a) => {
                                if !a.is_array_of_tables() {
                                    path = p;
                                }
                            }
                            dom::ValueNode::Table(t) => {
                                if t.is_inline() {
                                    path = p;
                                }
                            }
                            _ => {
                                path = p;
                            }
                        }
                    }
                }
                NodeRef::Key(n) => {
                    if n.text_ranges().iter().any(|r| r.contains(position)) {
                        nodes.push(node);
                        path = p;
                    }
                }
            }
        }

        if nodes.is_empty() {
            // Always has root.
            nodes.push(NodeRef::Root(self));
        }

        let syntax = SyntaxInfo::query(position, &root_syntax);

        if let Some(el) = &syntax.element {
            if el.kind() == PERIOD {
                let path_fragment = dom::Path::from_iter(
                    el.parent()
                        .unwrap()
                        .children_with_tokens()
                        .filter(|t| t.kind() == IDENT)
                        .map(|t| t.as_token().unwrap().text().to_string())
                        .map(|s| s.as_str().strip_quotes().to_string()),
                );

                // FIXME: this might not be 100% correct
                if path_fragment != path {
                    path = path.extend(path_fragment);
                }
            } else if let Some(entry) = el.parent() {
                if entry.kind() == ENTRY {
                    let key = entry.children().next().unwrap();

                    let path_fragment = dom::Path::from_iter(
                        key.children_with_tokens()
                            .filter(|t| t.kind() == IDENT)
                            .map(|t| t.as_token().unwrap().text().to_string())
                            .map(|s| s.as_str().strip_quotes().to_string()),
                    );

                    // FIXME: this might not be 100% correct
                    if path_fragment != path {
                        path = path.extend(path_fragment);
                    }
                }
            }
        }

        PositionInfo {
            path,
            nodes,
            syntax,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum NodeRef<'n> {
    Root(&'n dom::RootNode),
    Table(&'n dom::TableNode),
    Entry(&'n dom::EntryNode),
    Key(&'n dom::KeyNode),
    Value(&'n dom::ValueNode),
    Array(&'n dom::ArrayNode),
}

impl_is_node_ref! {
    is_root() -> Root;
    is_table() -> Table;
    is_entry() -> Entry;
    is_key() -> Key;
    is_value() -> Value;
    is_array() -> Array;
}

impl NodeRef<'_> {
    pub fn text_ranges(&self) -> TextRanges {
        match self {
            NodeRef::Root(v) => v.text_ranges(),
            NodeRef::Table(v) => v.text_ranges(),
            NodeRef::Entry(v) => v.text_ranges(),
            NodeRef::Key(v) => v.text_ranges(),
            NodeRef::Value(v) => v.text_ranges(),
            NodeRef::Array(v) => v.text_ranges(),
        }
    }

    pub fn into_node(self) -> dom::Node {
        match self {
            NodeRef::Root(v) => dom::Node::from(v.clone()),
            NodeRef::Table(v) => dom::Node::from(v.clone()),
            NodeRef::Entry(v) => dom::Node::from(v.clone()),
            NodeRef::Key(v) => dom::Node::from(v.clone()),
            NodeRef::Value(v) => dom::Node::from(v.clone()),
            NodeRef::Array(v) => dom::Node::from(v.clone()),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn name(&self) -> &str {
        match self {
            NodeRef::Root(_) => "root",
            NodeRef::Table(_) => "table",
            NodeRef::Entry(_) => "entry",
            NodeRef::Key(_) => "key",
            NodeRef::Value(_) => "value",
            NodeRef::Array(_) => "array",
        }
    }
}

#[derive(Debug)]
pub struct PositionInfo<'q> {
    /// The path to the deepest node at the position.
    pub path: dom::Path,
    /// All the overlapping nodes at the position,
    /// root is the first, and the deepest node is at the end.
    pub nodes: Vec<NodeRef<'q>>,
    /// The syntax information after the position.
    pub syntax: SyntaxInfo,
}

/// Value returned from a DOM query.
#[derive(Debug)]
pub struct PositionQueryResult<'q> {
    /// Information before the position cursor.
    /// [`None`] if the position is the start of the file.
    pub before: Option<PositionInfo<'q>>,
    /// Information after the position cursor.
    pub after: PositionInfo<'q>,
}

impl PositionQueryResult<'_> {
    pub fn is_completable(&self) -> bool {
        if self.is_before_header() || self.is_after_header() {
            return false;
        }

        if self
            .before
            .as_ref()
            .map(|p| p.syntax.syntax_kinds.iter().any(|k| *k == COMMENT))
            .unwrap_or(false)
        {
            return false;
        }

        true
    }

    pub fn is_inside_header(&self) -> bool {
        self.is_completable()
            && self
                .after
                .syntax
                .syntax_kinds
                .iter()
                .any(|kind| *kind == TABLE_ARRAY_HEADER || *kind == TABLE_HEADER)
    }

    pub fn is_empty_header(&self) -> bool {
        self.is_completable()
            && self.is_inside_header()
            && self
                .after
                .syntax
                .text
                .as_ref()
                .map(|t| t == "]")
                .unwrap_or(false)
            && self
                .before
                .as_ref()
                .and_then(|b| b.syntax.text.as_ref().map(|t| t == "["))
                .unwrap_or(false)
    }

    fn is_before_header(&self) -> bool {
        self.after
            .syntax
            .syntax_kinds
            .iter()
            .any(|k| *k == TABLE_HEADER || *k == TABLE_ARRAY_HEADER)
            && self
                .after
                .syntax
                .element
                .as_ref()
                .and_then(|el| el.as_token().map(|t| t.text() == "["))
                .unwrap_or(false)
    }

    fn is_after_header(&self) -> bool {
        self.before
            .as_ref()
            .map(|p| {
                p.syntax
                    .syntax_kinds
                    .iter()
                    .any(|k| *k == TABLE_HEADER || *k == TABLE_ARRAY_HEADER)
                    && p.syntax
                        .element
                        .as_ref()
                        .and_then(|el| el.as_token().map(|t| t.kind() == BRACKET_END))
                        .unwrap_or(false)
            })
            .unwrap_or(false)
    }
}

/// Information about a position query only based on the syntax tree.
///
/// The information might also contain incomplete and invalid nodes,
/// even if its syntax kind is valid.
#[derive(Debug)]
pub struct SyntaxInfo {
    /// The range of the relevant area.
    ///
    /// Note that this is not the range of the syntax token,
    /// but rather a helper for keys and values.
    pub range: Option<TextRange>,
    /// The text inside of relevant area.
    pub text: Option<String>,
    /// The syntax element if any.
    pub element: Option<SyntaxElement>,
    /// The syntax kids relevant to the position
    /// (e.g. [`SyntaxKind::KEY`] and [`SyntaxKind::TABLE_HEADER`] for inside table headers).
    pub syntax_kinds: Vec<SyntaxKind>,
    /// The path of a single dotted key.
    pub key_path: Option<dom::Path>,
}

impl SyntaxInfo {
    pub fn is_kind(&self, kind: SyntaxKind) -> bool {
        self.element
            .as_ref()
            .map(|s| s.kind() == kind)
            .unwrap_or(false)
    }

    /// First non-whitespace (or newline) syntax token before the element.
    pub fn first_token_before(&self) -> Option<(TextRange, SyntaxToken)> {
        self.element.as_ref().and_then(|el| {
            el.as_token().and_then(|token| {
                let mut token_before = token.prev_token();

                while let Some(tok) = token_before {
                    if tok.kind() != WHITESPACE || tok.kind() != NEWLINE {
                        return Some((tok.text_range(), tok));
                    }

                    token_before = tok.prev_token();
                }

                None
            })
        })
    }

    fn query(position: TextSize, root: &SyntaxElement) -> Self {
        let mut range: Option<TextRange> = Default::default();
        let mut text: Option<String> = Default::default();
        let mut element: Option<SyntaxElement> = Default::default();
        let mut syntax_kinds: Vec<SyntaxKind> = Default::default();
        let mut key_path: Option<dom::Path> = Default::default();

        let mut last: Option<SyntaxElement> = None;
        for n in root.as_node().unwrap().descendants_with_tokens() {
            if n.text_range().contains(position) {
                syntax_kinds.push(n.kind());
                last = Some(n);
            }
        }

        if let Some(last) = last {
            text = Some(last.to_string());

            // Header key of the table
            if (last.kind() == PERIOD || last.kind() == IDENT)
                && syntax_kinds
                    .iter()
                    .any(|k| *k == TABLE_HEADER || *k == TABLE_ARRAY_HEADER)
            {
                let key = last.parent().unwrap();

                // Keys without leading and trailing whitespace.
                let mut keys_range: Option<TextRange> = None;

                key_path = Some(dom::Path::from_iter(
                    key.children_with_tokens()
                        .filter(|t| t.kind() == IDENT || t.kind() == PERIOD)
                        .map(|ident| {
                            let token = ident.into_token().unwrap();

                            keys_range = match keys_range {
                                Some(r) => Some(r.cover(token.text_range())),
                                None => Some(token.text_range()),
                            };

                            token
                        })
                        .filter(|t| t.kind() == IDENT)
                        .map(|t| t.text().to_string())
                        .map(|s| s.as_str().strip_quotes().to_string()),
                ));
                range = keys_range;
                text = Some(key.text().to_string());
            }

            if syntax_kinds.iter().any(|k| *k == ENTRY) {
                if last.kind() == PERIOD || last.kind() == IDENT {
                    let key = last.parent().unwrap();

                    // Keys without leading and trailing whitespace.
                    let mut keys_range: Option<TextRange> = None;

                    key_path = Some(dom::Path::from_iter(
                        key.children_with_tokens()
                            .filter(|t| t.kind() == IDENT || t.kind() == PERIOD)
                            .map(|ident| {
                                let token = ident.into_token().unwrap();

                                keys_range = match keys_range {
                                    Some(r) => Some(r.cover(token.text_range())),
                                    None => Some(token.text_range()),
                                };

                                token
                            })
                            .filter(|t| t.kind() == IDENT)
                            .map(|t| t.text().to_string())
                            .map(|s| s.as_str().strip_quotes().to_string()),
                    ));
                    range = keys_range;
                    text = Some(key.text().to_string());
                } else if let Some(mut entry) = last.parent() {
                    if entry.kind() == VALUE {
                        entry = entry.parent().unwrap();
                    }

                    if entry.kind() == ENTRY {
                        if let Some(eq) = entry.children_with_tokens().find(|t| t.kind() == EQ) {
                            range = Some(TextRange::new(
                                eq.text_range().end(),
                                entry
                                    .children_with_tokens()
                                    .last()
                                    .map(|t| t.text_range().end())
                                    .unwrap_or_else(|| eq.text_range().end()),
                            ));
                        }
                    }

                    // FIXME: text could be useful here as well
                }
            }

            element = Some(last);
        }

        Self {
            range,
            text,
            element,
            syntax_kinds,
            key_path,
        }
    }
}

mod collect {
    use super::*;

    #[allow(dead_code)]
    pub fn collect_node<'a>(
        path: dom::Path,
        node: &'a dom::Node,
        nodes: &mut SmallVec<[(dom::Path, NodeRef<'a>); 20]>,
    ) {
        match node {
            dom::Node::Root(n) => collect_root(path, n, nodes),
            dom::Node::Table(n) => collect_table(path, n, nodes),
            dom::Node::Entry(n) => collect_entry(path, n, nodes),
            dom::Node::Value(n) => collect_value(path, n, nodes),
            dom::Node::Key(n) => collect_key(path, n, nodes),
            dom::Node::Array(n) => collect_array(path, n, nodes),
        }
    }

    pub fn collect_entry<'a>(
        path: dom::Path,
        node: &'a dom::EntryNode,
        nodes: &mut SmallVec<[(dom::Path, NodeRef<'a>); 20]>,
    ) {
        nodes.push((path.clone(), NodeRef::Entry(node)));

        collect_value(path.clone(), node.value(), nodes);
        collect_key(path, node.key(), nodes);
    }

    pub fn collect_key<'a>(
        path: dom::Path,
        node: &'a dom::KeyNode,
        nodes: &mut SmallVec<[(dom::Path, NodeRef<'a>); 20]>,
    ) {
        nodes.push((path, NodeRef::Key(node)));
    }

    pub fn collect_value<'a>(
        path: dom::Path,
        node: &'a dom::ValueNode,
        nodes: &mut SmallVec<[(dom::Path, NodeRef<'a>); 20]>,
    ) {
        nodes.push((path.clone(), NodeRef::Value(node)));
        match node {
            dom::ValueNode::Array(v) => collect_array(path, v, nodes),
            dom::ValueNode::Table(t) => collect_table(path, t, nodes),
            _ => {}
        }
    }

    pub fn collect_root<'a>(
        path: dom::Path,
        node: &'a dom::RootNode,
        nodes: &mut SmallVec<[(dom::Path, NodeRef<'a>); 20]>,
    ) {
        nodes.push((path.clone(), NodeRef::Root(node)));

        for (k, entry) in node.entries().iter() {
            collect_entry(
                path.clone().join(k.full_key_string_stripped()),
                entry,
                nodes,
            )
        }
    }

    pub fn collect_table<'a>(
        path: dom::Path,
        node: &'a dom::TableNode,
        nodes: &mut SmallVec<[(dom::Path, NodeRef<'a>); 20]>,
    ) {
        nodes.push((path.clone(), NodeRef::Table(node)));

        for (k, entry) in node.entries().iter() {
            collect_entry(
                path.clone().join(k.full_key_string_stripped()),
                entry,
                nodes,
            )
        }
    }

    pub fn collect_array<'a>(
        path: dom::Path,
        node: &'a dom::ArrayNode,
        nodes: &mut SmallVec<[(dom::Path, NodeRef<'a>); 20]>,
    ) {
        nodes.push((path.clone(), NodeRef::Array(node)));

        for (i, value) in node.items().iter().enumerate() {
            collect_value(path.clone().join(i), value, nodes)
        }
    }
}
