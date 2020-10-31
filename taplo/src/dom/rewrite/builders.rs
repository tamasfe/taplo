use super::{OldOrNew, RewriteBuilder};
use crate::dom;

#[derive(Debug, Clone)]
pub struct TableNode {
    inline: bool,
    part_of_array: bool,
    key: Option<OldOrNew<dom::KeyNode>>,
    entries: Vec<OldOrNew<dom::EntryNode>>,
}

impl TableNode {
    /// Adds a child node to the entries.
    pub fn with_entry(mut self, child: impl Into<OldOrNew<dom::EntryNode>>) -> Self {
        self.entries.push(child.into());
        self
    }

    /// Adds child nodes to the entries.
    pub fn with_entries(
        mut self,
        children: impl IntoIterator<Item = impl Into<OldOrNew<dom::EntryNode>>>,
    ) -> Self {
        self.entries.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn inline(mut self) -> Self {
        self.inline = true;
        self
    }

    pub fn top_level(mut self, key: impl Into<OldOrNew<dom::KeyNode>>, array: bool) -> Self {
        self.key = Some(key.into());
        self.part_of_array = array;
        self.inline = false;
        self
    }
}

impl RewriteBuilder for TableNode {
    type NewNode = super::TableNode;

    fn new() -> Self {
        Self {
            entries: Vec::new(),
            inline: true,
            part_of_array: false,
            key: None,
        }
    }

    fn build(self) -> Self::NewNode {
        super::TableNode {
            entries: self.entries,
            inline: self.inline,
            part_of_array: self.part_of_array,
            key: self.key,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntryNode {
    key: Option<OldOrNew<dom::KeyNode>>,
    value: Option<OldOrNew<dom::ValueNode>>,
}

impl EntryNode {
    pub fn with_key(mut self, key: impl Into<OldOrNew<dom::KeyNode>>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<OldOrNew<dom::ValueNode>>) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl RewriteBuilder for EntryNode {
    type NewNode = super::EntryNode;

    fn new() -> Self {
        Self {
            key: None,
            value: None,
        }
    }

    fn build(self) -> Self::NewNode {
        super::EntryNode {
            key: Box::new(self.key.expect("entry key is required").into()),
            value: Box::new(self.value.expect("entry value is required").into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyNode {
    keys: Option<String>,
}

impl KeyNode {
    pub fn with_dotted_keys(mut self, keys: impl ToString) -> Self {
        self.keys = Some(keys.to_string());
        self
    }
}

impl RewriteBuilder for KeyNode {
    type NewNode = super::KeyNode;

    fn new() -> Self {
        Self { keys: None }
    }

    fn build(self) -> Self::NewNode {
        super::KeyNode {
            keys: self.keys.expect("keys are missing"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValueNode {
    value: Option<super::ValueNode>,
}

impl ValueNode {
    pub fn with_value(mut self, value: impl Into<super::ValueNode>) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl RewriteBuilder for ValueNode {
    type NewNode = super::ValueNode;

    fn new() -> Self {
        Self { value: None }
    }

    fn build(self) -> Self::NewNode {
        self.value.expect("value is required")
    }
}

#[derive(Debug, Clone)]
pub struct IntegerNode {
    val: Option<i64>,
}

impl IntegerNode {
    pub fn with_value(mut self, value: impl Into<i64>) -> Self {
        self.val = Some(value.into());
        self
    }
}

impl RewriteBuilder for IntegerNode {
    type NewNode = super::IntegerNode;

    fn new() -> Self {
        Self { val: None }
    }

    fn build(self) -> Self::NewNode {
        super::IntegerNode {
            val: self.val.expect("value is required"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StringNode {
    val: Option<String>,
}

impl StringNode {
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.val = Some(value.into());
        self
    }
}

impl RewriteBuilder for StringNode {
    type NewNode = super::StringNode;

    fn new() -> Self {
        Self { val: None }
    }

    fn build(self) -> Self::NewNode {
        super::StringNode {
            val: self.val.expect("value is required"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoolNode {
    val: Option<bool>,
}

impl BoolNode {
    pub fn with_value(mut self, value: impl Into<bool>) -> Self {
        self.val = Some(value.into());
        self
    }
}

impl RewriteBuilder for BoolNode {
    type NewNode = super::BoolNode;

    fn new() -> Self {
        Self { val: None }
    }

    fn build(self) -> Self::NewNode {
        super::BoolNode {
            val: self.val.expect("value is required"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FloatNode {
    val: Option<f64>,
}

impl FloatNode {
    pub fn with_value(mut self, value: impl Into<f64>) -> Self {
        self.val = Some(value.into());
        self
    }
}

impl RewriteBuilder for FloatNode {
    type NewNode = super::FloatNode;

    fn new() -> Self {
        Self { val: None }
    }

    fn build(self) -> Self::NewNode {
        super::FloatNode {
            val: self.val.expect("value is required"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DateNode {}

impl RewriteBuilder for DateNode {
    type NewNode = super::DateNode;

    fn new() -> Self {
        todo!()
    }

    fn build(self) -> Self::NewNode {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ArrayNode {
    items: Vec<OldOrNew<dom::ValueNode>>,
}

impl ArrayNode {
    pub fn with_item(mut self, item: impl Into<OldOrNew<dom::ValueNode>>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn with_items(mut self, items: impl IntoIterator<Item = OldOrNew<dom::ValueNode>>) -> Self {
        self.items.extend(items.into_iter().map(Into::into));
        self
    }
}

impl RewriteBuilder for ArrayNode {
    type NewNode = super::ArrayNode;

    fn new() -> Self {
        Self {
            items: Default::default(),
        }
    }

    fn build(self) -> Self::NewNode {
        super::ArrayNode { items: self.items }
    }
}
