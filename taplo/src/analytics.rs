//! This module contains various methods and utilities for analyzing the DOM and/or the syntax tree.

use crate::{
    dom::{self, NodeSyntax, TextRanges},
    syntax::{
        SyntaxElement,
        SyntaxKind::{self, *},
    },
};
use rowan::{TextRange, TextSize};
use smallvec::SmallVec;

macro_rules! impl_is_node_ref {
    ($($method_name:ident() -> $variant:ident;)*) => {
        impl NodeRef<'_> {
            $(
                pub fn $method_name(&self) -> bool {
                    match self {
                        NodeRef::$variant(_) => true,
                        _ => false
                    }
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

        PositionInfo {
            path,
            nodes,
            syntax: SyntaxInfo::query(position, &root_syntax),
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

        true
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
    pub range: Option<TextRange>,
    /// The text inside of relevant area.
    pub text: Option<String>,
    /// The syntax element if any.
    pub element: Option<SyntaxElement>,
    /// The syntax kids relevant to the position
    /// (e.g. [`SyntaxKind::KEY`] and [`SyntaxKind::TABLE_HEADER`] for inside table headers).
    pub syntax_kinds: Vec<SyntaxKind>,
}

impl SyntaxInfo {
    pub fn is_kind(&self, kind: SyntaxKind) -> bool {
        self.element
            .as_ref()
            .map(|s| s.kind() == kind)
            .unwrap_or(false)
    }

    fn query(position: TextSize, root: &SyntaxElement) -> Self {
        let mut range: Option<TextRange> = Default::default();
        let mut text: Option<String> = Default::default();
        let mut element: Option<SyntaxElement> = Default::default();
        let mut syntax_kinds: Vec<SyntaxKind> = Default::default();
        let mut expected_kind: Option<SyntaxKind> = Default::default();

        let mut last: Option<SyntaxElement> = None;
        for n in root.as_node().unwrap().descendants_with_tokens() {
            if n.text_range().contains(position) {
                syntax_kinds.push(n.kind());
                last = Some(n);
            }
        }

        if let Some(last) = last {
            range = Some(last.text_range());
            text = Some(last.to_string());

            if syntax_kinds
                .iter()
                .any(|k| *k == TABLE_HEADER || *k == TABLE_ARRAY_HEADER)
            {
                expected_kind = Some(KEY);
            }

            // Header key of the table
            if (last.kind() == PERIOD || last.kind() == IDENT)
                && syntax_kinds
                    .iter()
                    .any(|k| *k == TABLE_HEADER || *k == TABLE_ARRAY_HEADER)
            {
                let key = last.parent().unwrap();
                range = Some(key.text_range());
                text = Some(key.text().to_string());
                expected_kind = Some(KEY);
            }

            if (last.kind() == PERIOD || last.kind() == IDENT)
                && syntax_kinds.iter().any(|k| *k == ENTRY)
            {
                let key = last.parent().unwrap();
                range = Some(key.text_range());
                text = Some(key.text().to_string());
                expected_kind = Some(KEY);
            }

            element = Some(last);
        }

        Self {
            range,
            text,
            element,
            syntax_kinds,
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
