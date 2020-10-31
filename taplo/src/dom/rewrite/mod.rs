// TODO: do conversions in a proper way.
use crate::{dom, syntax::SyntaxKind::*, util::syntax::add_all};
use rowan::GreenNodeBuilder;

use super::NodeSyntax;

#[doc(hidden)]
pub mod builders;
mod impls;

pub mod prelude {
    pub use super::Rewrite;
    pub use super::RewriteBuilder;
    pub use super::RewriteNode;
}

#[derive(Debug, Clone)]
pub enum RewriteNode {
    Old(dom::Node),
    New(Node),
}

impl RewriteNode {
    pub(crate) fn into_tree(self, builder: &mut GreenNodeBuilder) {
        match self {
            RewriteNode::Old(old) => match old.syntax() {
                rowan::NodeOrToken::Node(n) => {
                    add_all(n, builder);
                }
                rowan::NodeOrToken::Token(t) => {
                    builder.token(t.kind().into(), t.text().clone());
                }
            },
            RewriteNode::New(node) => node.into_tree(builder),
        }
    }
}

impl<N: Rewrite + Into<dom::Node>> From<OldOrNew<N>> for RewriteNode {
    fn from(n: OldOrNew<N>) -> Self {
        match n {
            OldOrNew::Old(o) => Self::Old(o.into()),
            OldOrNew::New(n) => Self::New(n.into()),
        }
    }
}

impl<N: Into<dom::Node>> From<N> for RewriteNode {
    fn from(node: N) -> Self {
        Self::Old(node.into())
    }
}

#[derive(Debug, Clone)]
pub enum OldOrNew<N: Rewrite + Into<dom::Node>> {
    Old(N),
    New(<N::Builder as RewriteBuilder>::NewNode),
}

impl<N: Rewrite + Into<dom::Node>> OldOrNew<N> {
    pub(crate) fn into_tree(self, builder: &mut GreenNodeBuilder) {
        match RewriteNode::from(self) {
            RewriteNode::New(node) => node.into_tree(builder),
            _ => {}
        }
    }
}

impl From<String> for OldOrNew<dom::KeyNode> {
    fn from(s: String) -> Self {
        Self::New(KeyNode { keys: s })
    }
}

impl From<&str> for OldOrNew<dom::KeyNode> {
    fn from(s: &str) -> Self {
        Self::New(KeyNode {
            keys: s.to_string(),
        })
    }
}

impl<N: Rewrite + Into<dom::Node>> From<N> for OldOrNew<N> {
    fn from(n: N) -> Self {
        Self::Old(n)
    }
}

impl<N: Into<ValueNode>> From<N> for OldOrNew<dom::ValueNode> {
    fn from(n: N) -> Self {
        Self::New(n.into())
    }
}

impl From<EntryNode> for OldOrNew<dom::EntryNode> {
    fn from(e: EntryNode) -> Self {
        Self::New(e)
    }
}

pub trait RewriteBuilder: dom::private::Sealed {
    type NewNode: Into<Node> + core::fmt::Debug + Clone;

    fn new() -> Self;
    fn build(self) -> Self::NewNode;
}

pub trait Rewrite: dom::private::Sealed {
    type Builder: RewriteBuilder;

    fn rewrite() -> Self::Builder;
}

#[derive(Debug, Clone)]
pub enum Node {
    Entry(EntryNode),
    Key(KeyNode),
    Value(ValueNode),
    Array(ArrayNode),
    Table(TableNode),
    Integer(IntegerNode),
    String(StringNode),
    Bool(BoolNode),
    Float(FloatNode),
    Date(DateNode),
}

impl Node {
    pub(crate) fn into_tree(self, builder: &mut GreenNodeBuilder) {
        match self {
            Node::Entry(val) => val.into_tree(builder),
            Node::Key(val) => val.into_tree(builder),
            Node::Value(val) => val.into_tree(builder),
            Node::Array(val) => val.into_tree(builder),
            Node::Table(val) => val.into_tree(builder),
            Node::Integer(val) => val.into_tree(builder),
            Node::String(val) => val.into_tree(builder),
            Node::Bool(val) => val.into_tree(builder),
            Node::Float(val) => val.into_tree(builder),
            Node::Date(val) => val.into_tree(builder),
        }
    }
}

rewrite_node_from! {
    EntryNode => Entry,
    KeyNode => Key,
    ValueNode => Value,
    ArrayNode => Array,
    TableNode => Table,
    IntegerNode => Integer,
    StringNode => String,
    BoolNode => Bool,
    FloatNode => Float,
    DateNode => Date
}

#[derive(Debug, Clone)]
pub struct EntryNode {
    pub(crate) key: Box<RewriteNode>,
    pub(crate) value: Box<RewriteNode>,
}

impl EntryNode {
    pub(crate) fn into_tree(self, builder: &mut GreenNodeBuilder) {
        builder.start_node(ENTRY.into());

        self.key.into_tree(builder);

        builder.token(WHITESPACE.into(), " ".into());
        builder.token(EQ.into(), "=".into());
        builder.token(WHITESPACE.into(), " ".into());

        self.value.into_tree(builder);

        builder.finish_node();
    }
}

#[derive(Debug, Clone)]
pub struct KeyNode {
    pub(crate) keys: String,
}

impl KeyNode {
    pub(crate) fn into_tree(self, builder: &mut GreenNodeBuilder) {
        let keys = self.keys.split('.').map(|s| {
            if s.is_empty() || s.is_ascii() {
                s.to_string()
            } else {
                format!(r#"'{}'"#, s)
            }
        });

        builder.start_node(KEY.into());

        for (i, key) in keys.enumerate() {
            if i != 0 {
                builder.token(PERIOD.into(), ".".into());
            }

            builder.token(IDENT.into(), key.into());
        }

        builder.finish_node();
    }
}

#[derive(Debug, Clone)]
pub enum ValueNode {
    Bool(OldOrNew<dom::BoolNode>),
    String(OldOrNew<dom::StringNode>),
    Integer(OldOrNew<dom::IntegerNode>),
    Float(OldOrNew<dom::FloatNode>),
    Array(OldOrNew<dom::ArrayNode>),
    Date(OldOrNew<dom::DateNode>),
    Table(OldOrNew<dom::TableNode>),
}

impl ValueNode {
    pub(crate) fn into_tree(self, builder: &mut GreenNodeBuilder) {
        match self {
            ValueNode::Bool(v) => v.into_tree(builder),
            ValueNode::String(v) => v.into_tree(builder),
            ValueNode::Integer(v) => v.into_tree(builder),
            ValueNode::Float(v) => v.into_tree(builder),
            ValueNode::Array(v) => v.into_tree(builder),
            ValueNode::Date(v) => v.into_tree(builder),
            ValueNode::Table(v) => v.into_tree(builder),
        }
    }
}

rewrite_value_node_from!(
    Bool => BoolNode,
    String => StringNode,
    Integer => IntegerNode,
    Float => FloatNode,
    Array => ArrayNode,
    Date => DateNode,
    Table => TableNode,
);

#[derive(Debug, Clone)]
pub struct ArrayNode {
    pub(crate) items: Vec<OldOrNew<dom::ValueNode>>,
}

impl ArrayNode {
    pub(crate) fn into_tree(self, builder: &mut GreenNodeBuilder) {
        builder.start_node(ARRAY.into());

        builder.token(BRACKET_START.into(), "[".into());
        builder.token(WHITESPACE.into(), " ".into());

        for item in self.items {
            item.into_tree(builder);
            builder.token(COMMA.into(), ",".into());
        }

        builder.token(WHITESPACE.into(), " ".into());
        builder.token(BRACKET_END.into(), "]".into());

        builder.finish_node();
    }
}

#[derive(Debug, Clone)]
pub struct TableNode {
    inline: bool,
    part_of_array: bool,
    key: Option<OldOrNew<dom::KeyNode>>,
    pub(crate) entries: Vec<OldOrNew<dom::EntryNode>>,
}

impl TableNode {
    pub(crate) fn into_tree(self, builder: &mut GreenNodeBuilder) {
        if self.inline {
            builder.start_node(INLINE_TABLE.into());
            builder.token(BRACE_START.into(), "{".into());
            builder.token(WHITESPACE.into(), " ".into());

            for (i, entry) in self.entries.into_iter().enumerate() {
                if i != 0 {
                    builder.token(COMMA.into(), ",".into());
                    builder.token(WHITESPACE.into(), " ".into());
                }
                entry.into_tree(builder);
            }

            builder.token(WHITESPACE.into(), " ".into());
            builder.token(BRACE_END.into(), "}".into());

            builder.finish_node();
        } else {
            let key = self.key.unwrap();

            if self.part_of_array {
                builder.start_node(TABLE_ARRAY_HEADER.into());
                builder.token(BRACKET_START.into(), "[".into());
                builder.token(BRACKET_START.into(), "[".into());
                key.into_tree(builder);
                builder.token(BRACKET_END.into(), "]".into());
                builder.token(BRACKET_END.into(), "]".into());
                builder.token(NEWLINE.into(), "\n".into());
                builder.finish_node();

                let entry_count = self.entries.len();

                for (i, entry) in self.entries.into_iter().enumerate() {
                    entry.into_tree(builder);
                    if i != entry_count - 1 {
                        builder.token(NEWLINE.into(), "\n".into());
                    }
                }
            } else {
                builder.start_node(TABLE_HEADER.into());
                builder.token(BRACKET_START.into(), "[".into());
                key.into_tree(builder);
                builder.token(BRACKET_END.into(), "]".into());
                builder.token(NEWLINE.into(), "\n".into());
                builder.finish_node();

                let entry_count = self.entries.len();

                for (i, entry) in self.entries.into_iter().enumerate() {
                    entry.into_tree(builder);
                    if i != entry_count - 1 {
                        builder.token(NEWLINE.into(), "\n".into());
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntegerNode {
    pub(crate) val: i64,
}

impl IntegerNode {
    pub(crate) fn into_tree(self, builder: &mut GreenNodeBuilder) {
        builder.token(INTEGER.into(), self.val.to_string().into());
    }
}

#[derive(Debug, Clone)]
pub struct StringNode {
    pub(crate) val: String,
}

impl StringNode {
    pub(crate) fn into_tree(self, builder: &mut GreenNodeBuilder) {
        // TODO: handle multiple content.
        builder.token(STRING.into(), self.val.into());
    }
}

#[derive(Debug, Clone)]
pub struct BoolNode {
    pub(crate) val: bool,
}

impl BoolNode {
    pub(crate) fn into_tree(self, builder: &mut GreenNodeBuilder) {
        builder.token(BOOL.into(), self.val.to_string().into());
    }
}

#[derive(Debug, Clone)]
pub struct FloatNode {
    pub(crate) val: f64,
}

impl FloatNode {
    pub(crate) fn into_tree(self, builder: &mut GreenNodeBuilder) {
        builder.token(FLOAT.into(), self.val.to_string().into());
    }
}

#[derive(Debug, Clone)]
pub struct DateNode {}

impl DateNode {
    pub(crate) fn into_tree(self, builder: &mut GreenNodeBuilder) {
        todo!()
    }
}

rewrite_impl!(
    dom::TableNode,
    dom::EntryNode,
    dom::KeyNode,
    dom::ValueNode,
    dom::ArrayNode,
    dom::IntegerNode,
    dom::StringNode,
    dom::BoolNode,
    dom::FloatNode,
    dom::DateNode,
);
