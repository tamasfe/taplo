use rowan::TextRange;

use crate::dom;

#[doc(hidden)]
pub mod builders;
mod impls;

#[derive(Debug)]
pub enum RewriteNode {
    Old(dom::Node),
    New(Node),
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

#[derive(Debug)]
pub enum OldOrNew<N: Rewrite + Into<dom::Node>> {
    Old(N),
    New(<N::Builder as RewriteBuilder>::NewNode),
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

pub trait RewriteBuilder: dom::private::Sealed {
    type NewNode: Into<Node> + core::fmt::Debug;

    fn new() -> Self;
    fn build(self) -> Self::NewNode;
}

pub trait Rewrite: dom::private::Sealed {
    type Builder: RewriteBuilder;

    fn rewrite() -> Self::Builder;
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct EntryNode {
    pub(crate) key: Box<RewriteNode>,
}

#[derive(Debug)]
pub struct KeyNode {
    pub(crate) keys: String,
}

#[derive(Debug)]
pub enum ValueNode {
    Bool(OldOrNew<dom::BoolNode>),
    String(OldOrNew<dom::StringNode>),
    Integer(OldOrNew<dom::IntegerNode>),
    Float(OldOrNew<dom::FloatNode>),
    Array(OldOrNew<dom::ArrayNode>),
    Date(OldOrNew<dom::DateNode>),
    Table(OldOrNew<dom::TableNode>),
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

#[derive(Debug)]
pub struct ArrayNode {
    pub(crate) items: Vec<OldOrNew<dom::ValueNode>>,
}

#[derive(Debug)]
pub struct TableNode {
    pub(crate) entries: Vec<OldOrNew<dom::EntryNode>>,
}

#[derive(Debug)]
pub struct IntegerNode {
    pub(crate) val: i64,
}

#[derive(Debug)]
pub struct StringNode {
    pub(crate) val: String,
}

#[derive(Debug)]
pub struct BoolNode {
    pub(crate) val: bool,
}

#[derive(Debug)]
pub struct FloatNode {
    pub(crate) val: f64,
}

#[derive(Debug)]
pub struct DateNode {}

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
