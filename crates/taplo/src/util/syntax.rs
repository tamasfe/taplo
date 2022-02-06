use rowan::{GreenNodeBuilder, NodeOrToken};

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
