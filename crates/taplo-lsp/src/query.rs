//! Cursor queries of a TOML document.

use taplo::{
    dom::{
        node::{DomNode, Key},
        KeyOrIndex, Keys, Node,
    },
    rowan::{TextRange, TextSize},
    syntax::{SyntaxNode, SyntaxToken},
    util::join_ranges,
};

#[derive(Debug, Default)]
pub struct Query {
    /// Before the cursor.
    pub before: Option<PositionInfo>,
    /// After the cursor.
    pub after: Option<PositionInfo>,
}

impl Query {
    /// Query a DOM root with the given cursor offset.
    /// Returns [`None`] if the position is out of range.
    ///
    /// # Panics
    ///
    /// Panics if the DOM was not entirely constructed from a syntax tree (e.g. if a node has no associated syntax element).
    /// Also panics if the given DOM node is not root.
    ///
    /// Also the given offset must be within the tree.
    pub fn at(root: &Node, offset: TextSize) -> Self {
        let syntax = root.syntax().cloned().unwrap().into_node().unwrap();

        Query {
            before: if offset == TextSize::from(0) {
                None
            } else {
                Self::query_left(
                    root,
                    &syntax,
                    offset.checked_sub(TextSize::from(1)).unwrap(),
                )
            },
            after: if offset >= syntax.text_range().end() {
                None
            } else {
                Self::query_right(root, &syntax, offset)
            },
        }
    }

    fn query_left(root: &Node, syntax: &SyntaxNode, offset: TextSize) -> Option<PositionInfo> {
        let syntax = match syntax.token_at_offset(offset) {
            taplo::rowan::TokenAtOffset::None => return None,
            taplo::rowan::TokenAtOffset::Single(s) => s,
            taplo::rowan::TokenAtOffset::Between(left, _) => left,
        };

        root.flat_iter()
            .filter(|(k, n)| full_range(k, n).contains(offset))
            .max_by_key(|(k, _)| k.len())
            .or_else(|| {
                root.flat_iter()
                    .take_while(|(k, n)| full_range(k, n).end() < offset)
                    .last()
            })
            .map(|dom_node| PositionInfo { dom_node, syntax })
    }

    fn query_right(root: &Node, syntax: &SyntaxNode, offset: TextSize) -> Option<PositionInfo> {
        let syntax = match syntax.token_at_offset(offset) {
            taplo::rowan::TokenAtOffset::None => return None,
            taplo::rowan::TokenAtOffset::Single(s) => s,
            taplo::rowan::TokenAtOffset::Between(_, right) => right,
        };

        root.flat_iter()
            .filter(|(k, n)| full_range(k, n).contains(offset))
            .max_by_key(|(k, _)| k.len())
            .or_else(|| {
                root.flat_iter()
                    .skip_while(|(k, n)| full_range(k, n).end() < offset)
                    .last()
            })
            .map(|dom_node| PositionInfo { dom_node, syntax })
    }
}

#[derive(Debug)]
pub struct PositionInfo {
    /// The narrowest syntax element that contains the position.
    pub syntax: SyntaxToken,
    /// The nearest node in the DOM away from the cursor.
    pub dom_node: (Keys, Node),
}

fn full_range(keys: &Keys, node: &Node) -> TextRange {
    join_ranges(
        keys.iter()
            .filter_map(KeyOrIndex::as_key)
            .map(Key::text_ranges)
            .flatten()
            .chain(node.text_ranges()),
    )
}
