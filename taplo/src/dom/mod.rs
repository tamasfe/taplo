//! This module contains the DOM for TOML source.
//!
//! Nodes in the DOM tree are typed and contain their character offsets
//! this allows for inspecting values while knowing where they actually are.
//!
//! When constructed from the root (which is practically always),
//! the tree is semantically analyzed according to the TOML specification.
//!
//! All the dotted keys and arrays of tables are also merged and collected
//! into tables and arrays. The order is always preserved when possible.
//!
//! The current DOM doesn't have comment or whitespace information directly exposed,
//! but these can be added anytime.
//!
//! The DOM is immutable right now, and only allows for semantic analysis,
//! but the ability to partially rewrite it is planned.
use crate::{
    syntax::{SyntaxElement, SyntaxKind::*, SyntaxToken},
    util::{unescape, StrExt},
    value::Value,
};
use indexmap::{indexmap, IndexMap};
use regex::Regex;
use rowan::{TextRange, TextSize};
use smallvec::{smallvec, SmallVec};
use std::{convert::TryFrom, convert::TryInto, hash::Hash, iter::FromIterator, mem, rc::Rc};

#[macro_use]
mod macros;

pub type TextRanges = SmallVec<[TextRange; 5]>;

#[cfg(feature = "rewrite")]
pub mod rewrite;

pub mod nodes {
    pub use super::ArrayNode;
    pub use super::BoolNode;
    pub use super::DateNode;
    pub use super::EntryNode;
    pub use super::FloatNode;
    pub use super::IntegerNode;
    pub use super::KeyNode;
    pub use super::Node;
    pub use super::RootNode;
    pub use super::StringNode;
    pub use super::TableNode;
    pub use super::ValueNode;
}

/// Casting allows constructing DOM nodes from syntax nodes.
pub trait Cast: Sized + private::Sealed {
    fn cast(element: SyntaxElement) -> Option<Self>;
}

pub trait NodeSyntax: core::fmt::Debug + private::Sealed {
    fn syntax(&self) -> SyntaxElement;
}

mod private {
    use super::*;

    pub trait Sealed {}
    dom_sealed!(
        Node,
        RootNode,
        EntryNode,
        KeyNode,
        ValueNode,
        ArrayNode,
        TableNode,
        IntegerNode,
        StringNode,
        BoolNode,
        FloatNode,
        DateNode,
    );

    #[cfg(feature = "rewrite")]
    dom_sealed!(
        rewrite::Node,
        rewrite::EntryNode,
        rewrite::KeyNode,
        rewrite::ValueNode,
        rewrite::ArrayNode,
        rewrite::TableNode,
        rewrite::IntegerNode,
        rewrite::StringNode,
        rewrite::BoolNode,
        rewrite::FloatNode,
        rewrite::DateNode,
    );

    #[cfg(feature = "rewrite")]
    dom_sealed!(
        rewrite::builders::TableNode,
        rewrite::builders::EntryNode,
        rewrite::builders::KeyNode,
        rewrite::builders::ValueNode,
        rewrite::builders::ArrayNode,
        rewrite::builders::IntegerNode,
        rewrite::builders::StringNode,
        rewrite::builders::BoolNode,
        rewrite::builders::FloatNode,
        rewrite::builders::DateNode,
    );
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Root(RootNode),
    Table(TableNode),
    Entry(EntryNode),
    Key(KeyNode),
    Value(ValueNode),
    Array(ArrayNode),
}

impl Node {
    /// Converts the node into a value.
    ///
    /// Panics if the node contains invalid values.
    ///
    /// Use `Value::try_from` for a fallible alternative.
    pub fn to_value(&self) -> Value {
        Value::try_from(self.clone()).unwrap()
    }
}

impl_is! {Node;
    is_root() -> Root;
    is_table() -> Table;
    is_entry() -> Entry;
    is_key() -> Key;
    is_value() -> Value;
    is_array() -> Array;
}

dom_node_from!(
    RootNode => Root,
    TableNode => Table,
    EntryNode => Entry,
    KeyNode => Key,
    ValueNode => Value,
    ArrayNode => Array
);

impl NodeSyntax for Node {
    fn syntax(&self) -> SyntaxElement {
        match self {
            Node::Root(v) => v.syntax(),
            Node::Table(v) => v.syntax(),
            Node::Entry(v) => v.syntax(),
            Node::Key(v) => v.syntax(),
            Node::Value(v) => v.syntax(),
            Node::Array(v) => v.syntax(),
        }
    }
}

impl Cast for Node {
    fn cast(element: SyntaxElement) -> Option<Self> {
        match element.kind() {
            STRING
            | MULTI_LINE_STRING
            | STRING_LITERAL
            | MULTI_LINE_STRING_LITERAL
            | INTEGER
            | INTEGER_HEX
            | INTEGER_OCT
            | INTEGER_BIN
            | FLOAT
            | BOOL
            | DATE
            | INLINE_TABLE => ValueNode::dom_inner(element).map(Node::Value),
            KEY => KeyNode::cast(element).map(Node::Key),
            VALUE => ValueNode::cast(element).map(Node::Value),
            TABLE_HEADER | TABLE_ARRAY_HEADER => TableNode::cast(element).map(Node::Table),
            ENTRY => EntryNode::cast(element).map(Node::Entry),
            ARRAY => ArrayNode::cast(element).map(Node::Array),
            ROOT => RootNode::cast(element).map(Node::Root),
            _ => None,
        }
    }
}

dom_node! {
    /// The root of the DOM.
    ///
    /// Constructing it will normalize all the dotted keys,
    /// and merge all the tables that need to be merged,
    /// and also creates arrays from array of tables.
    /// And also semantically validates the tree according
    /// to the TOML specification.
    ///
    /// If any errors occur, the tree might be
    /// missing entries, or will be completely empty.
    ///
    /// Syntax errors are **not** reported, those have to
    /// be checked before constructing the DOM.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct RootNode {
        errors: Vec<Error>,
        entries: Entries,
    }
}

impl RootNode {
    pub fn text_ranges(&self) -> TextRanges {
        smallvec![self.syntax.text_range()]
    }

    pub fn entries(&self) -> &Entries {
        &self.entries
    }

    pub fn into_entries(self) -> Entries {
        self.entries
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
}

impl Cast for RootNode {
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        if syntax.kind() != ROOT {
            return None;
        }

        let root_syntax = syntax.as_node().unwrap();

        let mut entries = Entries::new();
        // top-level tables AND arrays of tables.
        let mut tables: Vec<TableNode> = Vec::new();
        let mut errors: Vec<Error> = Vec::new();

        for child in root_syntax.children_with_tokens() {
            match child.kind() {
                TABLE_HEADER | TABLE_ARRAY_HEADER => {
                    let table = match TableNode::cast(child) {
                        None => continue,
                        Some(table) => table,
                    };

                    if let Some(t) = tables.last_mut() {
                        t.end_offset = table.syntax.text_range().start();
                    }

                    tables.push(table);
                }
                ENTRY => {
                    let entry = match EntryNode::cast(child) {
                        None => continue,
                        Some(e) => e,
                    };

                    if tables.is_empty() {
                        entries.insert(entry, &mut errors);
                        continue;
                    }

                    let table_count = tables.len();

                    tables
                        .get_mut(table_count - 1)
                        .unwrap()
                        .entries
                        .insert(entry, &mut errors);
                }
                _ => {}
            }
        }

        if let Some(t) = tables.last_mut() {
            t.end_offset = root_syntax.text_range().end();
        }

        entries.add_tables(tables, &mut errors);
        entries.normalize();
        entries.sync_keys();

        Some(Self {
            syntax,
            errors,
            entries,
        })
    }
}

dom_node! {
    /// A table node is used for tables, arrays of tables,
    /// and also inline tables.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct TableNode {
        /// Key for top-level tables.
        key: Option<KeyNode>,

        /// Whether the table is part of an array
        /// of tables.
        array: bool,

        /// Whether the table is an inline table.
        inline: bool,

        /// Pseudo-tables are made from dotted keys.
        /// These are actually not part of the parsed
        /// source.
        pseudo: bool,

        // Offset of the next entry or EOF,
        // this is needed because tables span
        // longer than their actual syntax in TOML.
        end_offset: TextSize,

        entries: Entries,
    }
}

impl TableNode {
    pub fn into_entries(self) -> Entries {
        self.entries
    }

    pub fn entries(&self) -> &Entries {
        &self.entries
    }

    pub fn is_inline(&self) -> bool {
        self.inline
    }

    pub fn is_pseudo(&self) -> bool {
        self.pseudo
    }

    pub fn is_part_of_array(&self) -> bool {
        self.array
    }

    pub fn key(&self) -> Option<&KeyNode> {
        self.key.as_ref()
    }

    pub fn text_ranges(&self) -> TextRanges {
        let mut ranges = SmallVec::with_capacity(self.entries.len() + 1);

        ranges.push(self.syntax().text_range().cover_offset(self.end_offset));
        ranges.extend(self.entries.iter().map(|(_, e)| e.syntax().text_range()));

        ranges
    }
}

impl Cast for TableNode {
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        match syntax.kind() {
            TABLE_HEADER | TABLE_ARRAY_HEADER => {
                let n = syntax.as_node().unwrap();

                let key = n
                    .first_child()
                    .and_then(|e| KeyNode::cast(rowan::NodeOrToken::Node(e)));

                key.as_ref()?;

                Some(Self {
                    entries: Entries::default(),
                    end_offset: syntax.text_range().end(),
                    pseudo: false,
                    inline: false,
                    array: syntax.kind() == TABLE_ARRAY_HEADER,
                    key: syntax
                        .as_node()
                        .unwrap()
                        .first_child()
                        .and_then(|n| Cast::cast(n.into())),
                    syntax,
                })
            }
            // FIXME(recursion)
            INLINE_TABLE => Some(Self {
                entries: Entries(
                    syntax
                        .as_node()
                        .unwrap()
                        .children_with_tokens()
                        .filter_map(EntryNode::cast)
                        .map(|entry| (entry.key().clone(), entry))
                        .collect(),
                ),
                key: None,
                end_offset: syntax.text_range().end(),
                inline: true,
                array: false,
                pseudo: false,
                syntax,
            }),
            _ => None,
        }
    }
}

/// New type that adds features to the regular
/// index map, used by root and table nodes.
#[derive(Debug, Default, Clone, Eq)]
#[repr(transparent)]
pub struct Entries(IndexMap<KeyNode, EntryNode>);

impl Entries {
    /// Creates a new list entry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the number of elements.
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = (&KeyNode, &EntryNode)> {
        self.0.iter()
    }

    fn insert(&mut self, mut entry: EntryNode, errors: &mut Vec<Error>) {
        for (_, existing_entry) in self.0.iter_mut().rev() {
            let existing_key = &existing_entry.key;

            if existing_key == &entry.key {
                errors.push(Error::DuplicateKey {
                    first: existing_entry.key().clone(),
                    second: entry.key().clone(),
                });
                return;
            }

            let existing_part_of = existing_key.is_part_of(entry.key());
            let existing_contains = existing_key.contains(entry.key());
            let common_count = existing_key.common_prefix_count(entry.key());

            if existing_part_of || existing_contains {
                if let ValueNode::Table(t) = &entry.value {
                    if t.is_inline() {
                        errors.push(Error::InlineTable {
                            target: existing_key.clone(),
                            key: entry.key.clone(),
                        });
                        return;
                    }
                } else if let ValueNode::Table(t) = &existing_entry.value {
                    if t.is_inline() {
                        errors.push(Error::InlineTable {
                            target: existing_key.clone(),
                            key: entry.key.clone(),
                        });
                        return;
                    }
                } else if existing_key.key_count() != entry.key().key_count() {
                    errors.push(Error::DottedKeyConflict {
                        first: existing_key.clone(),
                        second: entry.key.clone(),
                    });
                    return;
                }

                let common_prefix = existing_key.clone().common_prefix(entry.key());

                existing_entry.key = existing_entry.key.clone().without_prefix(&common_prefix);
                entry.key = entry.key.without_prefix(&common_prefix);

                let pseudo_table = EntryNode {
                    syntax: common_prefix.clone().syntax,
                    key: common_prefix.clone(),
                    value: ValueNode::Table(TableNode {
                        syntax: common_prefix.syntax.clone(),
                        inline: false,
                        end_offset: Default::default(),
                        pseudo: true,
                        array: false,
                        key: None,
                        entries: Entries(indexmap! {
                            existing_entry.key.clone().without_prefix(&common_prefix) => mem::replace(existing_entry, EntryNode {
                                syntax: common_prefix.syntax.clone(),
                                key: common_prefix.clone(),
                                value: ValueNode::Empty
                            }),
                            entry.key.clone().without_prefix(&common_prefix) => entry
                        }),
                    }),
                };

                *existing_entry = pseudo_table;
                return;
            } else if common_count > 0 {
                let mut common_prefix_key = existing_key.clone().common_prefix(entry.key());
                common_prefix_key
                    .additional_keys
                    .push(entry.key().clone().common_prefix(&existing_key));

                let mut pseudo_entries = Entries(IndexMap::with_capacity(2));

                let existing_value = mem::take(&mut existing_entry.value);

                pseudo_entries.0.insert(
                    existing_key.clone().without_prefix(&common_prefix_key),
                    EntryNode {
                        syntax: existing_entry.syntax.clone(),
                        key: existing_key.clone().without_prefix(&common_prefix_key),
                        value: existing_value,
                    },
                );

                pseudo_entries.0.insert(
                    entry.key.clone().without_prefix(&common_prefix_key),
                    EntryNode {
                        syntax: entry.syntax,
                        key: entry.key.without_prefix(&common_prefix_key),
                        value: entry.value,
                    },
                );

                let pseudo_table = EntryNode {
                    syntax: common_prefix_key.clone().syntax,
                    key: common_prefix_key.clone(),
                    value: ValueNode::Table(TableNode {
                        syntax: common_prefix_key.syntax.clone(),
                        inline: false,
                        end_offset: Default::default(),
                        pseudo: true,
                        array: false,
                        key: None,
                        entries: pseudo_entries,
                    }),
                };

                *existing_entry = pseudo_table;
                return;
            }
        }

        self.0.insert(entry.key().clone(), entry);
    }

    fn add_tables(&mut self, tables: Vec<TableNode>, errors: &mut Vec<Error>) {
        for table in tables {
            match table.syntax.kind() {
                TABLE_ARRAY_HEADER => self.insert_table_array(None, table, errors),
                TABLE_HEADER => self.insert_table(None, table, errors),
                _ => unreachable!(),
            }
        }
    }

    fn insert_table(
        &mut self,
        alternative_key: Option<KeyNode>,
        table: TableNode,
        errors: &mut Vec<Error>,
    ) {
        let mut key = alternative_key.unwrap_or_else(|| table.key().unwrap().clone());

        for (_, entry) in &mut self.0 {
            // We don't rely on the key values in the map,
            // as the entry keys are modified anyway.
            let existing_key = entry.key.clone();

            if existing_key == key {
                match entry.syntax().kind() {
                    TABLE_HEADER => {
                        errors.push(Error::DuplicateKey {
                            first: existing_key.clone(),
                            second: key,
                        });
                    }
                    _ => errors.push(Error::ExpectedTable {
                        target: existing_key.clone(),
                        key,
                    }),
                }
                return;
            }

            if existing_key.is_part_of(&key) {
                match &mut entry.value {
                    ValueNode::Array(arr) => {
                        if !arr.tables {
                            errors.push(Error::ExpectedTable {
                                target: existing_key.clone(),
                                key,
                            });
                            return;
                        }

                        arr.insert_table(key.without_prefix(&existing_key).into(), table, errors)
                    }
                    ValueNode::Table(existing_table) => {
                        entry
                            .key
                            .additional_keys
                            .push(key.clone().common_prefix(&existing_key));

                        existing_table.entries.insert_table(
                            key.without_prefix(&existing_key).into(),
                            table,
                            errors,
                        )
                    }
                    _ => errors.push(Error::ExpectedTable {
                        target: existing_key.clone(),
                        key,
                    }),
                }

                return;
            } else if key.is_part_of(&existing_key) {
                match &entry.value {
                    ValueNode::Table(_) => {
                        key.additional_keys
                            .push(existing_key.clone().common_prefix(&key));

                        let mut new_entry = EntryNode {
                            syntax: table.syntax.clone(),
                            key,
                            value: ValueNode::Table(table),
                        };

                        mem::swap(entry, &mut new_entry);

                        match &mut entry.value {
                            ValueNode::Table(new_table) => {
                                match new_entry.value {
                                    ValueNode::Table(old_table) => new_table.entries.insert_table(
                                        existing_key.clone().without_prefix(&new_entry.key).into(),
                                        old_table,
                                        errors,
                                    ),
                                    _ => {
                                        // We've just swapped it.
                                        unreachable!()
                                    }
                                }
                            }
                            _ => {
                                // We've just swapped it.
                                unreachable!()
                            }
                        }
                    }
                    _ => errors.push(Error::ExpectedTable {
                        target: existing_key.clone(),
                        key,
                    }),
                }
                return;
            } else if key.key_count() == existing_key.key_count()
                && key.common_prefix_count(&existing_key) > 0
            {
                match &entry.value {
                    ValueNode::Table(_) => {
                        let mut common_prefix_key = key.clone().common_prefix(&existing_key);
                        common_prefix_key
                            .additional_keys
                            .push(existing_key.clone().common_prefix(&key));

                        let existing_value = mem::take(&mut entry.value);

                        let mut pseudo_entries = Entries(IndexMap::with_capacity(2));

                        pseudo_entries.0.insert(
                            existing_key.clone().without_prefix(&common_prefix_key),
                            EntryNode {
                                syntax: entry.syntax.clone(),
                                key: existing_key.clone().without_prefix(&common_prefix_key),
                                value: existing_value,
                            },
                        );

                        pseudo_entries.0.insert(
                            key.clone().without_prefix(&common_prefix_key),
                            EntryNode {
                                syntax: entry.syntax.clone(),
                                key: key.without_prefix(&common_prefix_key),
                                value: ValueNode::Table(table),
                            },
                        );

                        entry.key = common_prefix_key;
                        entry.value = ValueNode::Table(TableNode {
                            inline: false,
                            end_offset: entry.syntax.text_range().end(),
                            syntax: entry.syntax.clone(),
                            key: None,
                            pseudo: true,
                            array: false,
                            entries: pseudo_entries,
                        });
                    }
                    _ => errors.push(Error::ExpectedTable {
                        target: existing_key.clone(),
                        key,
                    }),
                }
                return;
            }
        }

        self.0.insert(
            key.clone(),
            EntryNode {
                syntax: table.syntax.clone(),
                key,
                value: ValueNode::Table(table),
            },
        );
    }

    fn insert_table_array(
        &mut self,
        alternative_key: Option<KeyNode>,
        array_table: TableNode,
        errors: &mut Vec<Error>,
    ) {
        let key = alternative_key.unwrap_or_else(|| array_table.key().unwrap().clone());

        for (_, entry) in self.0.iter_mut().rev() {
            let existing_key = &entry.key.clone();
            if existing_key == &key {
                match &mut entry.value {
                    ValueNode::Array(arr) => {
                        if !arr.tables {
                            errors.push(Error::ExpectedTable {
                                target: existing_key.clone(),
                                key,
                            });
                            return;
                        }

                        entry.key.additional_keys.push(key);

                        arr.items.push(ValueNode::Table(array_table))
                    }
                    _ => errors.push(Error::ExpectedTableArray {
                        target: existing_key.clone(),
                        key,
                    }),
                }
                return;
            }

            if existing_key.is_part_of(&key) {
                match &mut entry.value {
                    ValueNode::Array(arr) => {
                        if !arr.tables {
                            errors.push(Error::ExpectedTable {
                                target: existing_key.clone(),
                                key,
                            });
                            return;
                        }

                        entry
                            .key
                            .additional_keys
                            .push(key.clone().common_prefix(existing_key));

                        arr.insert_table(
                            key.without_prefix(existing_key).into(),
                            array_table,
                            errors,
                        )
                    }
                    ValueNode::Table(existing_table) => {
                        entry
                            .key
                            .additional_keys
                            .push(key.clone().common_prefix(existing_key));

                        existing_table.entries.insert_table_array(
                            key.without_prefix(existing_key).into(),
                            array_table,
                            errors,
                        )
                    }
                    _ => errors.push(Error::ExpectedTable {
                        target: existing_key.clone(),
                        key,
                    }),
                }
                return;
            } else if existing_key.contains(&key) {
                match &entry.value {
                    ValueNode::Table(_) => {
                        errors.push(Error::SubTableBeforeTableArray {
                            target: existing_key.clone(),
                            key,
                        });
                    }
                    _ => {
                        errors.push(Error::ExpectedTableArray {
                            target: existing_key.clone(),
                            key,
                        });
                    }
                }
                return;
            }
        }

        self.0.insert(
            key.clone(),
            EntryNode {
                syntax: array_table.syntax.clone(),
                key,
                value: ValueNode::Array(ArrayNode {
                    syntax: array_table.syntax.clone(),
                    tables: true,
                    items: vec![ValueNode::Table(array_table)],
                }),
            },
        );
    }

    /// Synchronize keys with the entries they point to while keeping the order,
    /// this is required since merging entries might cause the keys to change.
    fn sync_keys(&mut self) {
        let entries = mem::take(&mut self.0);

        self.0 = entries
            .into_iter()
            .map(|(key, mut entry)| {
                entry.value.sync_keys();
                if &key != entry.key() {
                    (entry.key.clone(), entry)
                } else {
                    (key, entry)
                }
            })
            .collect();
    }

    /// Normalizes all entries, replacing dotted keys
    /// with pseudo-tables.
    fn normalize(&mut self) {
        let mut entries_list = vec![&mut self.0];

        while let Some(entries) = entries_list.pop() {
            for (_, entry) in entries.iter_mut() {
                entry.normalize();

                match &mut entry.value {
                    ValueNode::Array(a) => {
                        let mut inner_arrs = vec![a];

                        while let Some(arr) = inner_arrs.pop() {
                            for item in arr.items.iter_mut() {
                                match item {
                                    ValueNode::Array(a) => {
                                        inner_arrs.push(a);
                                    }
                                    ValueNode::Table(t) => {
                                        entries_list.push(&mut t.entries.0);
                                    }

                                    _ => {}
                                }
                            }
                        }
                    }
                    ValueNode::Table(t) => {
                        entries_list.push(&mut t.entries.0);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl IntoIterator for Entries {
    type Item = (KeyNode, EntryNode);
    type IntoIter = indexmap::map::IntoIter<KeyNode, EntryNode>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<(KeyNode, EntryNode)> for Entries {
    fn from_iter<T: IntoIterator<Item = (KeyNode, EntryNode)>>(iter: T) -> Self {
        let i = iter.into_iter();
        let hint = i.size_hint();

        let len = match hint.1 {
            None => hint.0,
            Some(l) => l,
        };

        let mut entries = IndexMap::with_capacity(len);

        for entry in i {
            entries.insert(entry.0, entry.1);
        }

        Entries(entries)
    }
}

impl PartialEq for Entries {
    fn eq(&self, other: &Self) -> bool {
        for ((_, entry1), (_, entry2)) in self.0.iter().zip(other.0.iter()) {
            if !entry1.eq(entry2) {
                return false;
            }
        }
        true
    }
}

impl Hash for Entries {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (_, entry) in &self.0 {
            entry.hash(state);
        }
    }
}

dom_node! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct ArrayNode {
        tables: bool,
        items: Vec<ValueNode>,

    }
}

impl ArrayNode {
    pub fn items(&self) -> &[ValueNode] {
        &self.items
    }

    pub fn into_items(self) -> Vec<ValueNode> {
        self.items
    }

    pub fn is_array_of_tables(&self) -> bool {
        self.tables
    }

    pub fn key(&self) -> Option<KeyNode> {
        self.syntax
            .as_node()
            .unwrap()
            .first_child()
            .and_then(|n| Cast::cast(n.into()))
    }

    pub fn text_ranges(&self) -> TextRanges {
        if !self.tables {
            return smallvec![self.syntax.text_range()];
        }

        let mut ranges = SmallVec::with_capacity(self.items.len() + 1);

        ranges.push(self.syntax().text_range());

        for item in &self.items {
            ranges.extend(item.text_ranges());
        }

        ranges
    }

    fn insert_table(
        &mut self,
        alternative_key: Option<KeyNode>,
        table: TableNode,
        errors: &mut Vec<Error>,
    ) {
        let key = alternative_key.unwrap_or_else(|| table.key().unwrap().clone());

        match self.items.last_mut().unwrap() {
            ValueNode::Table(last_table) => match table.syntax.kind() {
                TABLE_HEADER => last_table.entries.insert_table(
                    key.without_prefix(&table.key().unwrap()).into(),
                    table,
                    errors,
                ),
                TABLE_ARRAY_HEADER => last_table.entries.insert_table_array(
                    key.without_prefix(&table.key().unwrap()).into(),
                    table,
                    errors,
                ),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn sync_keys(&mut self) {
        for item in &mut self.items {
            item.sync_keys();
        }
    }
}

impl Cast for ArrayNode {
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        match syntax.kind() {
            // FIXME(recursion)
            ARRAY => Some(Self {
                items: syntax
                    .as_node()
                    .unwrap()
                    .children_with_tokens()
                    .filter_map(Cast::cast)
                    .collect(),
                tables: false,
                syntax,
            }),
            TABLE_ARRAY_HEADER => Some(Self {
                items: Vec::new(),
                tables: false,
                syntax,
            }),
            _ => None,
        }
    }
}

dom_node! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct EntryNode {
        key: KeyNode,
        value: ValueNode,
    }
}

impl EntryNode {
    pub fn key(&self) -> &KeyNode {
        &self.key
    }

    pub fn value(&self) -> &ValueNode {
        &self.value
    }

    pub fn into_value(self) -> ValueNode {
        self.value
    }

    pub fn text_ranges(&self) -> TextRanges {
        match &self.value {
            ValueNode::Array(arr) => {
                let mut ranges = smallvec![self.syntax.text_range()];
                ranges.extend(arr.text_ranges());
                ranges
            }
            ValueNode::Table(t) => {
                let mut ranges = smallvec![self.syntax.text_range()];
                ranges.extend(t.text_ranges());
                ranges
            }
            _ => smallvec![self.syntax().text_range()],
        }
    }

    /// Turns a dotted key into nested pseudo-tables.
    fn normalize(&mut self) {
        while self.key.key_count() > 1 {
            let new_key = self.key.clone().prefix();
            let inner_key = self.key.clone().last();

            let value = mem::take(&mut self.value);

            let inner_entry = EntryNode {
                syntax: self.syntax.clone(),
                key: inner_key.clone(),
                value,
            };

            let inner_entry_syntax = inner_entry.syntax.clone();

            let mut entries = Entries(IndexMap::with_capacity(1));
            entries.0.insert(inner_key.clone(), inner_entry);

            self.value = ValueNode::Table(TableNode {
                inline: false,
                end_offset: Default::default(),
                syntax: inner_entry_syntax,
                key: None,
                pseudo: true,
                array: false,
                entries,
            });
            self.key = new_key;
        }
    }
}

impl Cast for EntryNode {
    fn cast(element: SyntaxElement) -> Option<Self> {
        if element.kind() != ENTRY {
            None
        } else {
            let key = element
                .as_node()
                .unwrap()
                .first_child_or_token()
                .and_then(Cast::cast);

            key.as_ref()?;

            let val = element
                .as_node()
                .unwrap()
                .first_child()
                .and_then(|k| k.next_sibling())
                .map(rowan::NodeOrToken::Node)
                .and_then(Cast::cast);

            match val {
                Some(value) => Some(Self {
                    key: key.unwrap(),
                    value,
                    syntax: element,
                }),
                None => None,
            }
        }
    }
}

dom_node_no_display! {
    #[derive(Debug, Clone)]
    pub struct KeyNode {
        // To avoid cloning the idents vec,
        // we mask them instead.
        mask_left: usize,
        mask_right: usize,

        // The visible ident count, can never be 0
        mask_visible: usize,

        // Hash and equality is based on only
        // the string values of the idents.
        idents: Rc<SmallVec<[SyntaxToken; 10]>>,

        /// In case the same key appears multiple times (e.g. in multiple dotted keys)
        /// additional keys will be stored in this.
        additional_keys: Vec<KeyNode>,
    }
}

impl core::fmt::Display for KeyNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_key_string_stripped())
    }
}

impl KeyNode {
    pub fn idents(&self) -> impl Iterator<Item = &SyntaxToken> {
        self.idents[..self.idents.len() - self.mask_right]
            .iter()
            .skip(self.mask_left)
    }

    pub fn key_count(&self) -> usize {
        self.mask_visible
    }

    pub fn keys_str(&self) -> impl Iterator<Item = &str> {
        self.idents().map(|t| t.text().as_str())
    }

    /// Quotes are removed from the keys.
    pub fn keys_str_stripped(&self) -> impl Iterator<Item = &str> {
        self.keys_str().map(|s| s.strip_quotes())
    }

    pub fn full_key_string_stripped(&self) -> String {
        let s: Vec<String> = self.keys_str_stripped().map(|s| s.to_string()).collect();
        s.join(".")
    }

    pub fn full_key_string(&self) -> String {
        let s: Vec<String> = self.keys_str().map(|s| s.to_string()).collect();
        s.join(".")
    }

    /// Determines whether the key starts with
    /// the same dotted keys as other.
    pub fn is_part_of(&self, other: &KeyNode) -> bool {
        if other.mask_visible < self.mask_visible {
            return false;
        }

        for (a, b) in self.keys_str_stripped().zip(other.keys_str_stripped()) {
            if a != b {
                return false;
            }
        }

        true
    }

    /// Determines whether the key starts with
    /// the same dotted keys as other.
    pub fn contains(&self, other: &KeyNode) -> bool {
        other.is_part_of(self)
    }

    /// retains n idents from the left,
    /// e.g.: outer.inner => outer
    /// there will be at least one ident remaining
    pub fn skip_left(mut self, n: usize) -> Self {
        let skip = usize::min(
            self.mask_visible - 1,
            self.mask_visible.checked_sub(n).unwrap_or_default(),
        );
        self.mask_right += skip;
        self.mask_visible -= skip;

        for key in &mut self.additional_keys {
            *key = key.clone().skip_left(n);
        }

        self
    }

    /// skips n idents from the left,
    /// e.g.: outer.inner => inner
    /// there will be at least one ident remaining
    pub fn retain_left(mut self, n: usize) -> Self {
        let skip = usize::min(self.mask_visible - 1, n);
        self.mask_left += skip;
        self.mask_visible -= skip;

        for key in &mut self.additional_keys {
            *key = key.clone().retain_left(n);
        }

        self
    }

    /// Counts the shared prefix keys
    pub fn common_prefix_count(&self, other: &KeyNode) -> usize {
        let mut count = 0;

        for (a, b) in self.keys_str().zip(other.keys_str()) {
            if a != b {
                break;
            }
            count += 1;
        }

        count
    }

    /// Keys that are identical to this one but exist
    /// somewhere else in the document.
    ///
    /// These are created when two dotted keys or
    /// arrays of tables are merged.
    pub fn additional_keys(&self) -> &[KeyNode] {
        &self.additional_keys
    }

    pub fn text_range(&self) -> TextRange {
        let range = match self.idents().next().map(|id| id.text_range()) {
            Some(range) => range,
            None => return Default::default(),
        };

        self.idents()
            .fold(range, |total, id| total.cover(id.text_range()))
    }

    pub fn text_ranges(&self) -> TextRanges {
        let mut ranges = smallvec![self.text_range()];
        ranges.extend(self.additional_keys.iter().map(|k| k.text_range()));

        ranges
    }

    /// Removes other's prefix from self
    fn without_prefix(self, other: &KeyNode) -> Self {
        let count = self.common_prefix_count(other);

        if count > 0 {
            self.retain_left(count)
        } else {
            self
        }
    }

    fn common_prefix(self, other: &KeyNode) -> Self {
        let count = self.common_prefix_count(other);
        self.skip_left(count)
    }

    fn prefix(self) -> Self {
        let count = self.key_count();
        self.skip_left(count - 1)
    }

    fn last(self) -> Self {
        let count = self.key_count();
        self.retain_left(count)
    }
}

impl PartialEq for KeyNode {
    fn eq(&self, other: &Self) -> bool {
        self.key_count() == other.key_count() && self.is_part_of(other)
    }
}

impl Eq for KeyNode {}

// Needed because of custom PartialEq
impl Hash for KeyNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for s in self.keys_str_stripped() {
            s.hash(state)
        }
    }
}

impl Cast for KeyNode {
    fn cast(element: SyntaxElement) -> Option<Self> {
        if element.kind() != KEY {
            None
        } else {
            element.clone().into_node().and_then(|n| {
                let i: SmallVec<[SyntaxToken; 10]> = n
                    .children_with_tokens()
                    .filter_map(|c| {
                        if let rowan::NodeOrToken::Token(t) = c {
                            match t.kind() {
                                IDENT => Some(t),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    })
                    .collect();
                if i.is_empty() {
                    return None;
                }

                Some(Self {
                    mask_left: 0,
                    mask_right: 0,
                    mask_visible: i.len(),
                    idents: Rc::new(i),
                    additional_keys: Vec::new(),
                    syntax: element,
                })
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueNode {
    Bool(BoolNode),
    String(StringNode),
    Integer(IntegerNode),
    Float(FloatNode),
    Array(ArrayNode),
    Date(DateNode),
    Table(TableNode),
    Invalid(SyntaxElement),
    Empty,
}

impl Default for ValueNode {
    fn default() -> Self {
        ValueNode::Empty
    }
}

impl ValueNode {
    pub fn text_ranges(&self) -> TextRanges {
        match self {
            ValueNode::Bool(v) => smallvec![v.syntax().text_range()],
            ValueNode::String(v) => smallvec![v.syntax().text_range()],
            ValueNode::Integer(v) => smallvec![v.syntax().text_range()],
            ValueNode::Float(v) => smallvec![v.syntax().text_range()],
            ValueNode::Array(v) => v.text_ranges(),
            ValueNode::Date(v) => smallvec![v.syntax().text_range()],
            ValueNode::Table(v) => v.text_ranges(),
            ValueNode::Invalid(el) => smallvec![el.text_range()],
            ValueNode::Empty => unreachable!("empty value"),
        }
    }

    fn sync_keys(&mut self) {
        match self {
            ValueNode::Array(a) => a.sync_keys(),
            ValueNode::Table(t) => t.entries.sync_keys(),
            _ => {}
        }
    }

    fn dom_inner(element: SyntaxElement) -> Option<Self> {
        match element.kind() {
            INLINE_TABLE => Cast::cast(element).map(ValueNode::Table),
            ARRAY => Cast::cast(element).map(ValueNode::Array),
            BOOL => Cast::cast(element).map(ValueNode::Bool),
            STRING | STRING_LITERAL | MULTI_LINE_STRING | MULTI_LINE_STRING_LITERAL => {
                Cast::cast(element).map(ValueNode::String)
            }
            INTEGER | INTEGER_BIN | INTEGER_HEX | INTEGER_OCT => {
                Cast::cast(element).map(ValueNode::Integer)
            }
            FLOAT => Cast::cast(element).map(ValueNode::Float),
            DATE => Cast::cast(element).map(ValueNode::Date),
            _ => None,
        }
    }
}

impl NodeSyntax for ValueNode {
    fn syntax(&self) -> SyntaxElement {
        match self {
            ValueNode::Bool(v) => v.syntax(),
            ValueNode::String(v) => v.syntax(),
            ValueNode::Integer(v) => v.syntax(),
            ValueNode::Float(v) => v.syntax(),
            ValueNode::Array(v) => v.syntax(),
            ValueNode::Date(v) => v.syntax(),
            ValueNode::Table(v) => v.syntax(),
            ValueNode::Invalid(el) => el.clone(),
            _ => panic!("empty value"),
        }
    }
}

impl Cast for ValueNode {
    fn cast(element: SyntaxElement) -> Option<Self> {
        if element.kind() != VALUE {
            return None;
        }

        element
            .clone()
            .into_node()
            .and_then(|n| n.first_child_or_token())
            .and_then(|c| match c.kind() {
                INLINE_TABLE => Cast::cast(c).map(ValueNode::Table),
                ARRAY => Cast::cast(c).map(ValueNode::Array),
                BOOL => Cast::cast(c).map(ValueNode::Bool),
                STRING | STRING_LITERAL | MULTI_LINE_STRING | MULTI_LINE_STRING_LITERAL => {
                    Cast::cast(c).map(ValueNode::String)
                }
                INTEGER | INTEGER_BIN | INTEGER_HEX | INTEGER_OCT => {
                    Cast::cast(c).map(ValueNode::Integer)
                }
                FLOAT => Cast::cast(c).map(ValueNode::Float),
                DATE => Cast::cast(c).map(ValueNode::Date),
                _ => None,
            })
            .or(Some(ValueNode::Invalid(element)))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IntegerRepr {
    Dec,
    Bin,
    Oct,
    Hex,
}

dom_node! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct IntegerNode {
        repr: IntegerRepr,
    }
}

impl From<IntegerNode> for Node {
    fn from(n: IntegerNode) -> Self {
        Node::Value(ValueNode::Integer(n))
    }
}

impl IntegerNode {
    pub fn repr(&self) -> IntegerRepr {
        self.repr
    }

    pub fn as_i64(&self) -> i64 {
        match Value::try_from(self.clone()).unwrap() {
            Value::Integer(i) => i,
            _ => unreachable!(),
        }
    }
}

impl Cast for IntegerNode {
    fn cast(element: SyntaxElement) -> Option<Self> {
        match element.kind() {
            INTEGER => Some(IntegerNode {
                syntax: element,
                repr: IntegerRepr::Dec,
            }),
            INTEGER_BIN => Some(IntegerNode {
                syntax: element,
                repr: IntegerRepr::Bin,
            }),
            INTEGER_HEX => Some(IntegerNode {
                syntax: element,
                repr: IntegerRepr::Hex,
            }),
            INTEGER_OCT => Some(IntegerNode {
                syntax: element,
                repr: IntegerRepr::Oct,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StringKind {
    Basic,
    MultiLine,
    Literal,
    MultiLineLiteral,
}

dom_node! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct StringNode {
        kind: StringKind,

        /// Unescaped (and trimmed where defined by TOML) value.
        content: String,
    }
}

impl From<StringNode> for Node {
    fn from(n: StringNode) -> Self {
        Node::Value(ValueNode::String(n))
    }
}

impl StringNode {
    pub fn string_kind(&self) -> StringKind {
        self.kind
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn into_content(self) -> String {
        self.content
    }
}

impl Cast for StringNode {
    fn cast(element: SyntaxElement) -> Option<Self> {
        match element.kind() {
            STRING => Some(StringNode {
                kind: StringKind::Basic,
                content: match unescape(
                    element
                        .as_token()
                        .unwrap()
                        .text()
                        .as_str()
                        .remove_prefix(r#"""#)
                        .remove_suffix(r#"""#),
                ) {
                    Ok(s) => s,
                    Err(_) => return None,
                },
                syntax: element,
            }),
            MULTI_LINE_STRING => Some(StringNode {
                kind: StringKind::MultiLine,
                content: match unescape(
                    element
                        .as_token()
                        .unwrap()
                        .text()
                        .as_str()
                        .remove_prefix(r#"""""#)
                        .remove_suffix(r#"""""#)
                        .remove_prefix("\n"),
                ) {
                    Ok(s) => s,
                    Err(_) => return None,
                },
                syntax: element,
            }),
            STRING_LITERAL => Some(StringNode {
                kind: StringKind::Literal,
                content: element
                    .as_token()
                    .unwrap()
                    .text()
                    .as_str()
                    .remove_prefix(r#"'"#)
                    .remove_suffix(r#"'"#)
                    .into(),
                syntax: element,
            }),
            MULTI_LINE_STRING_LITERAL => Some(StringNode {
                kind: StringKind::MultiLineLiteral,
                content: element
                    .as_token()
                    .unwrap()
                    .text()
                    .as_str()
                    .remove_prefix(r#"'''"#)
                    .remove_suffix(r#"'''"#)
                    .remove_prefix("\n")
                    .into(),
                syntax: element,
            }),
            _ => None,
        }
    }
}

dom_primitives!(
    BOOL => BoolNode,
    FLOAT => FloatNode,
    DATE => DateNode
);

impl From<BoolNode> for Node {
    fn from(v: BoolNode) -> Self {
        Node::Value(ValueNode::Bool(v))
    }
}

impl From<FloatNode> for Node {
    fn from(v: FloatNode) -> Self {
        Node::Value(ValueNode::Float(v))
    }
}

impl From<DateNode> for Node {
    fn from(v: DateNode) -> Self {
        Node::Value(ValueNode::Date(v))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Error {
    DuplicateKey { first: KeyNode, second: KeyNode },
    DottedKeyConflict { first: KeyNode, second: KeyNode },
    ExpectedTableArray { target: KeyNode, key: KeyNode },
    ExpectedTable { target: KeyNode, key: KeyNode },
    InlineTable { target: KeyNode, key: KeyNode },
    SubTableBeforeTableArray { target: KeyNode, key: KeyNode },
    Spanned { range: TextRange, message: String },
    Generic(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DuplicateKey { first, second } => write!(
                f,
                "duplicate keys: \"{}\" ({:?}) and \"{}\" ({:?})",
                &first.full_key_string(),
                &first.syntax().text_range(),
                &second.full_key_string(),
                &second.syntax().text_range()
            ),
            Error::DottedKeyConflict { first, second } => write!(
                f,
                "conflicting dotted keys: \"{}\" ({:?}) and \"{}\" ({:?}), entries with overlapping keys must have the same amount of keys",
                &first.full_key_string(),
                &first.syntax().text_range(),
                &second.full_key_string(),
                &second.syntax().text_range()
            ),
            Error::ExpectedTable { target, key } => write!(
                f,
                "Expected \"{}\" ({:?}) to be a table, but it is not, required by \"{}\" ({:?})",
                &target.full_key_string(),
                &target.syntax().text_range(),
                &key.full_key_string(),
                &key.syntax().text_range()
            ),
            Error::ExpectedTableArray { target, key } => write!(
                f,
                "\"{}\" ({:?}) conflicts with array of tables: \"{}\" ({:?})",
                &target.full_key_string(),
                &target.syntax().text_range(),
                &key.full_key_string(),
                &key.syntax().text_range()
            ),
            Error::InlineTable { target, key } => write!(
                f,
                "inline tables cannot be modified: \"{}\" ({:?}), modification attempted here: \"{}\" ({:?})",
                &target.full_key_string(),
                &target.syntax().text_range(),
                &key.full_key_string(),
                &key.syntax().text_range()
            ),
            Error::SubTableBeforeTableArray { target, key } => write!(
                f,
                "subtable declared before array of tables: \"{}\" ({:?}), array of tables here: \"{}\" ({:?})",
                &target.full_key_string(),
                &target.syntax().text_range(),
                &key.full_key_string(),
                &key.syntax().text_range()
            ),
            Error::Spanned { range, message } => write!(f, "{} ({:?})", message, range),
            Error::Generic(s) => s.fmt(f),
        }
    }
}
impl std::error::Error for Error {}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PathKey {
    Key(String),
    Index(usize),
}

impl ToString for PathKey {
    fn to_string(&self) -> String {
        match self {
            PathKey::Key(v) => v.clone(),
            PathKey::Index(v) => v.to_string(),
        }
    }
}

impl From<String> for PathKey {
    fn from(s: String) -> Self {
        Self::Key(s)
    }
}

impl From<&String> for PathKey {
    fn from(s: &String) -> Self {
        Self::Key(s.to_owned())
    }
}

impl From<&str> for PathKey {
    fn from(s: &str) -> Self {
        Self::Key(s.into())
    }
}

impl From<usize> for PathKey {
    fn from(n: usize) -> Self {
        Self::Index(n)
    }
}

impl From<i32> for PathKey {
    fn from(n: i32) -> Self {
        Self::Index(n.try_into().unwrap())
    }
}

impl From<i64> for PathKey {
    fn from(n: i64) -> Self {
        Self::Index(n.try_into().unwrap())
    }
}

impl From<u32> for PathKey {
    fn from(n: u32) -> Self {
        Self::Index(n.try_into().unwrap())
    }
}

impl From<u64> for PathKey {
    fn from(n: u64) -> Self {
        Self::Index(n.try_into().unwrap())
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Path {
    keys: Rc<SmallVec<[PathKey; 10]>>,
    mask_left: usize,
    mask_right: usize,
}

impl<K: Into<PathKey>> FromIterator<K> for Path {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        Self {
            keys: Rc::new(iter.into_iter().map(Into::into).collect()),
            mask_left: 0,
            mask_right: 0,
        }
    }
}

impl Path {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new path with a new segment added to the end.
    ///
    /// This will clone the underlying storage.
    pub fn join<K: Into<PathKey>>(&self, key: K) -> Self {
        let mut new_keys: SmallVec<[PathKey; 10]> = self.keys().cloned().collect();
        new_keys.push(key.into());
        Self {
            keys: Rc::new(new_keys),
            mask_left: 0,
            mask_right: 0,
        }
    }

    pub fn extend(&self, iter: impl IntoIterator<Item = impl Into<PathKey>>) -> Self {
        let mut new_keys: SmallVec<[PathKey; 10]> = self.keys().cloned().collect();
        new_keys.extend(iter.into_iter().map(Into::into));
        Self {
            keys: Rc::new(new_keys),
            mask_left: 0,
            mask_right: 0,
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = &PathKey> {
        self.keys[..self.keys.len() - self.mask_right]
            .iter()
            .skip(self.mask_left)
    }

    pub fn dotted(&self) -> String {
        let simple = Regex::new("[A-Za-z0-9]+").unwrap();

        self.keys()
            .map(|s| s.to_string())
            .map(|s| {
                if simple.is_match(&s) {
                    s
                } else {
                    format!(r#"'{}'"#, s)
                }
            })
            .collect::<Vec<String>>()
            .join(".")
    }

    pub fn len(&self) -> usize {
        self.keys.len() - self.mask_left - self.mask_right
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a new path retaining n keys from the left,
    /// e.g.: outer.inner => outer
    pub fn retain_left(&self, n: usize) -> Self {
        let mut new = self.clone();
        let skip = usize::min(self.len(), self.len().checked_sub(n).unwrap_or_default());
        new.mask_right += skip;
        new
    }

    /// Returns a new path skipping n keys from the left,
    /// e.g.: outer.inner => inner
    pub fn skip_left(&self, n: usize) -> Self {
        let mut new = self.clone();
        let skip = usize::min(self.len(), n);
        new.mask_left += skip;
        new
    }

    /// Returns a new path skipping n keys from the right,
    /// e.g.: outer.inner => outer
    pub fn skip_right(&self, n: usize) -> Self {
        let mut new = self.clone();
        let skip = usize::min(self.len(), n);
        new.mask_right += skip;
        new
    }
}

impl IntoIterator for Path {
    type Item = PathKey;

    type IntoIter = smallvec::IntoIter<[PathKey; 10]>;

    fn into_iter(self) -> Self::IntoIter {
        (&*self.keys).clone().into_iter()
    }
}

#[macro_export]
macro_rules! path {
    ($($idx:expr,)*) => {
        {
            let mut p = $crate::dom::Path::new();
            $(p = p.join($idx);)*
            p
        }
    };
    ($($idx:expr),*) => {
        path!($(idx,)*)
    };
}
