use rowan::{GreenNodeBuilder, NodeOrToken, TextRange};

use crate::syntax::SyntaxNode;

pub fn add_all(node: SyntaxNode, builder: &mut GreenNodeBuilder) {
    builder.start_node(node.kind().into());

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => add_all(n, builder),
            NodeOrToken::Token(t) => builder.token(t.kind().into(), t.text()),
        }
    }

    builder.finish_node()
}

pub fn join_ranges<I: IntoIterator<Item = TextRange>>(ranges: I) -> TextRange {
    ranges
        .into_iter()
        .fold(None, |ranges, range| match ranges {
            Some(r) => Some(range.cover(r)),
            None => Some(range),
        })
        .unwrap()
}
