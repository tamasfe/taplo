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
    syntax::{SyntaxElement, SyntaxKind, SyntaxKind::*, SyntaxNode, SyntaxToken},
    util::{unescape, StringExt},
};
use indexmap::IndexMap;
use rowan::{TextRange, TextSize};
use std::{hash::Hash, iter::FromIterator, mem, rc::Rc};

#[macro_use]
mod macros;

/// Casting allows constructing DOM nodes from syntax nodes.
pub trait Cast: Sized + private::Sealed {
    fn cast(element: SyntaxElement) -> Option<Self>;
}

pub trait Common: core::fmt::Display + core::fmt::Debug + private::Sealed {
    fn syntax(&self) -> SyntaxElement;
    fn text_range(&self) -> TextRange;

    fn is_valid(&self) -> bool {
        true
    }
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
        DateNode
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

dom_node_from!(
    RootNode => Root,
    TableNode => Table,
    EntryNode => Entry,
    KeyNode => Key,
    ValueNode => Value,
    ArrayNode => Array
);

impl core::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Root(v) => v.fmt(f),
            Node::Table(v) => v.fmt(f),
            Node::Entry(v) => v.fmt(f),
            Node::Key(v) => v.fmt(f),
            Node::Value(v) => v.fmt(f),
            Node::Array(v) => v.fmt(f),
        }
    }
}

impl Common for Node {
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

    fn text_range(&self) -> TextRange {
        match self {
            Node::Root(v) => v.text_range(),
            Node::Table(v) => v.text_range(),
            Node::Entry(v) => v.text_range(),
            Node::Key(v) => v.text_range(),
            Node::Value(v) => v.text_range(),
            Node::Array(v) => v.text_range(),
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            Node::Root(v) => v.is_valid(),
            Node::Table(v) => v.is_valid(),
            Node::Entry(v) => v.is_valid(),
            Node::Key(v) => v.is_valid(),
            Node::Value(v) => v.is_valid(),
            Node::Array(v) => v.is_valid(),
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

impl Node {
    pub fn text_range(&self) -> TextRange {
        match self {
            Node::Root(v) => v.text_range(),
            Node::Table(v) => v.text_range(),
            Node::Entry(v) => v.text_range(),
            Node::Key(v) => v.text_range(),
            Node::Value(v) => v.text_range(),
            Node::Array(v) => v.text_range(),
        }
    }

    pub fn kind(&self) -> SyntaxKind {
        match self {
            Node::Root(v) => v.syntax().kind(),
            Node::Table(v) => v.syntax().kind(),
            Node::Entry(v) => v.syntax().kind(),
            Node::Key(v) => v.syntax().kind(),
            Node::Value(v) => v.syntax().kind(),
            Node::Array(v) => v.syntax().kind(),
        }
    }
}

dom_display!(
    RootNode,
    TableNode,
    EntryNode,
    ArrayNode,
    IntegerNode,
    StringNode
);

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
    syntax: SyntaxNode,
    errors: Vec<Error>,
    entries: Entries,
}

impl RootNode {
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

impl Common for RootNode {
    fn syntax(&self) -> SyntaxElement {
        self.syntax.clone().into()
    }

    fn text_range(&self) -> TextRange {
        self.syntax.text_range()
    }
}

// TODO(refactor)
// This has become a mess, it screams for a refactor
#[allow(clippy::cognitive_complexity)]
impl Cast for RootNode {
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        if syntax.kind() != ROOT {
            return None;
        }

        // Syntax node of the root.
        let syntax_node = syntax.into_node().unwrap();

        let child_count = syntax_node.children_with_tokens().count();

        // All the entries in the TOML document.
        // The key is their full path, including all parent tables.
        //
        // The contents of inline tables are not checked, and they are
        // treated like any other value.
        let mut entries: IndexMap<KeyNode, EntryNode> = IndexMap::with_capacity(child_count);

        // Prefixes are remembered for each entry.
        // this is to determine which table owns which entry.
        // Its length should match the entries' length.
        let mut prefixes: Vec<Option<KeyNode>> = Vec::with_capacity(child_count);

        // Table prefix for the entries following
        let mut prefix: Option<KeyNode> = None;

        // All top-level tables for a given index
        let mut tables: Vec<Vec<KeyNode>> = Vec::with_capacity(child_count);

        let mut errors = Vec::new();

        for child in syntax_node.children_with_tokens() {
            match child.kind() {
                TABLE_HEADER | TABLE_ARRAY_HEADER => {
                    let t = match TableNode::cast(child) {
                        None => continue,
                        Some(t) => t,
                    };

                    let mut key = match t
                        .syntax
                        .first_child()
                        .and_then(|n| KeyNode::cast(rowan::NodeOrToken::Node(n)))
                        .ok_or(Error::Spanned {
                            range: t.text_range(),
                            message: "table has no key".into(),
                        }) {
                        Ok(k) => k,
                        Err(err) => {
                            errors.push(err);
                            continue;
                        }
                    };

                    // We have to go through everything because we don't
                    // know the last index. And the existing table can be anything
                    // anywhere.
                    let existing_table = entries.iter().rev().find(|(k, _)| k.eq_keys(&key));

                    // The entries below still belong to this table,
                    // so we cannot skip its prefix, even on errors.
                    if let Some((existing_key, existing)) = existing_table {
                        let existing_table_array = match &existing.value {
                            ValueNode::Table(t) => t.is_part_of_array(),
                            _ => false,
                        };

                        if existing_table_array && !t.is_part_of_array() {
                            errors.push(Error::ExpectedTableArray {
                                target: existing.key().clone(),
                                key: key.clone(),
                            });
                        } else if !existing_table_array && t.is_part_of_array() {
                            errors.push(Error::ExpectedTableArray {
                                target: key.clone(),
                                key: existing.key().clone(),
                            });
                        } else if !existing_table_array && !t.is_part_of_array() {
                            errors.push(Error::DuplicateKey {
                                first: existing.key().clone(),
                                second: key.clone(),
                            });
                        } else {
                            key = key.with_index(existing_key.index + 1);
                            entries.insert(
                                key.clone(),
                                EntryNode {
                                    syntax: t.syntax.clone(),
                                    key: key.clone(),
                                    value: ValueNode::Table(t),
                                    next_entry: None,
                                },
                            );
                        }
                    } else {
                        entries.insert(
                            key.clone(),
                            EntryNode {
                                syntax: t.syntax.clone(),
                                key: key.clone(),
                                value: ValueNode::Table(t),
                                next_entry: None,
                            },
                        );
                    }

                    // Search for an entry that clashes with this table.
                    for (i, (k, e)) in entries.iter().enumerate().rev().skip(1) {
                        let entry_prefix = prefixes.get(i).unwrap();
                        if let Some(p) = entry_prefix {
                            if k.contains(&key) && p.common_prefix_count(&key) < key.key_count() {
                                errors.push(Error::TopLevelTableDefined {
                                    table: key.clone(),
                                    key: e.key.clone(),
                                });
                            }
                        }
                    }

                    if tables.len() == key.index {
                        tables.push(vec![key.clone()]);
                    } else {
                        tables[key.index].push(key.clone());
                    }

                    prefixes.push(None);
                    prefix = Some(key);
                }
                ENTRY => {
                    let entry = match EntryNode::cast(child) {
                        None => continue,
                        Some(e) => e,
                    };

                    let insert_key = match &prefix {
                        None => entry.key().clone(),
                        Some(p) => entry.key().clone().with_prefix(p),
                    };

                    if let Some(p) = &prefix {
                        let table_containing_entry =
                            tables.get(insert_key.index).and_then(|same_index_tables| {
                                same_index_tables
                                    .iter()
                                    .find(|table| {
                                        insert_key
                                            .clone()
                                            .without_prefix(p)
                                            .contains(&(&**table).clone().without_prefix(p))
                                    })
                                    .and_then(|table| {
                                        if table != same_index_tables.last().unwrap() {
                                            Some(table)
                                        } else {
                                            None
                                        }
                                    })
                            });

                        if let Some(table_key) = table_containing_entry {
                            errors.push(Error::TopLevelTableDefined {
                                table: table_key.clone(),
                                key: entry.key().clone(),
                            });
                            continue;
                        }
                    }

                    if let Some(existing) = entries.get(&insert_key) {
                        errors.push(Error::DuplicateKey {
                            first: existing.key().clone(),
                            second: entry.key().clone(),
                        });
                        continue;
                    }

                    prefixes.push(prefix.clone());
                    entries.insert(insert_key, entry);
                }
                _ => {}
            }
        }

        // Some additional checks for each entry
        let grouped_by_index = entries.iter().fold(
            Vec::<Vec<(&KeyNode, &EntryNode)>>::new(),
            |mut all, (k, e)| {
                if all.len() < k.index + 1 {
                    let mut v = Vec::with_capacity(entries.len());
                    v.push((k, e));
                    all.push(v);
                } else {
                    all[k.index].push((k, e));
                }

                all
            },
        );

        #[allow(clippy::never_loop)]
        'outer: for (group_idx, group) in grouped_by_index.iter().enumerate() {
            for (i, (k, e)) in group.iter().enumerate() {
                // Look for regular sub-tables before arrays of tables
                if group_idx == 0 {
                    let is_table_array = match &e.value {
                        ValueNode::Table(t) => t.is_part_of_array(),
                        _ => false,
                    };

                    if !is_table_array {
                        let table_array = group.iter().skip(i).find(|(k2, e2)| match &e2.value {
                            ValueNode::Table(t) => k2.is_part_of(k) && t.is_part_of_array(),
                            _ => false,
                        });

                        if let Some((k2, _)) = table_array {
                            errors.push(Error::ExpectedTableArray {
                                target: (&**k2).clone(),
                                key: (&**k).clone(),
                            })
                        }
                    }
                }

                // We might do more checks if needed
                break 'outer;
            }
        }

        let mut final_entries = Entries::from_map(entries);

        // Otherwise we could show false errors.
        if errors.is_empty() {
            final_entries.merge(&mut errors);
            final_entries.normalize();
        }

        final_entries.set_table_spans(
            &syntax_node,
            Some(syntax_node.text_range().end() + TextSize::from(1)),
        );

        Some(Self {
            entries: final_entries,
            errors,
            syntax: syntax_node,
        })
    }
}

/// A table node is used for tables, arrays of tables,
/// and also inline tables.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableNode {
    syntax: SyntaxNode,

    /// Array of tables.
    array: bool,

    /// Pseudo-tables are made from dotted keys.
    /// These are actually not part of the parsed
    /// source.
    pseudo: bool,

    // Offset of the next entry if any,
    // this is needed because tables span
    // longer than their actual syntax in TOML.
    next_entry: Option<TextSize>,

    entries: Entries,
}

impl TableNode {
    pub fn into_entries(self) -> Entries {
        self.entries
    }

    pub fn entries(&self) -> &Entries {
        &self.entries
    }

    pub fn is_part_of_array(&self) -> bool {
        self.array
    }

    pub fn is_inline(&self) -> bool {
        match self.syntax.kind() {
            INLINE_TABLE => true,
            _ => false,
        }
    }

    pub fn is_pseudo(&self) -> bool {
        self.pseudo
    }
}

impl Common for TableNode {
    fn syntax(&self) -> SyntaxElement {
        self.syntax.clone().into()
    }

    fn text_range(&self) -> TextRange {
        let mut range = self.syntax.text_range();

        if let Some(r) = self.entries().text_range() {
            range = range.cover(r)
        }

        if let Some(r) = self.next_entry.as_ref() {
            range = range.cover_offset(*r);
        }

        range
    }
}

impl Cast for TableNode {
    fn cast(syntax: SyntaxElement) -> Option<Self> {
        match syntax.kind() {
            TABLE_HEADER | TABLE_ARRAY_HEADER => {
                let n = syntax.into_node().unwrap();

                let key = n
                    .first_child()
                    .and_then(|e| KeyNode::cast(rowan::NodeOrToken::Node(e)));

                key.as_ref()?;

                Some(Self {
                    entries: Entries::default(),
                    next_entry: None,
                    pseudo: false,
                    array: n.kind() == TABLE_ARRAY_HEADER,
                    syntax: n,
                })
            }
            // FIXME(recursion)
            INLINE_TABLE => Some(Self {
                entries: syntax
                    .as_node()
                    .unwrap()
                    .children_with_tokens()
                    .filter_map(Cast::cast)
                    .collect(),
                next_entry: None,
                array: false,
                pseudo: false,
                syntax: syntax.into_node().unwrap(),
            }),
            _ => None,
        }
    }
}

/// Newtype that adds features to the regular
/// index map, used by root and table nodes.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Entries(Vec<EntryNode>);

impl Entries {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &EntryNode> {
        self.0.iter()
    }

    pub fn text_range(&self) -> Option<TextRange> {
        let mut range = None;
        for e in &self.0 {
            match &mut range {
                None => range = Some(e.key().text_range()),
                Some(r) => *r = r.cover(e.value().text_range()),
            }
        }
        range
    }

    // Top level tables and arrays of tables
    // need to span across whitespace as well.
    fn set_table_spans(&mut self, root_syntax: &SyntaxNode, end: Option<TextSize>) {
        for entry in self.0.iter_mut() {
            // We search for the next headers that don't
            // have the current entry as a prefix.
            if let TABLE_HEADER = entry.syntax.kind() {
                let mut found = false;

                let entry_header_string = entry.syntax.to_string();

                for n in root_syntax.children() {
                    if n.text_range().start() < entry.syntax.text_range().end() {
                        continue;
                    }

                    let new_header_string = n.to_string();

                    if let TABLE_HEADER | TABLE_ARRAY_HEADER = n.kind() {
                        if !new_header_string
                            .trim_start_matches('[')
                            .trim_end_matches(']')
                            .starts_with(
                                entry_header_string
                                    .trim_start_matches('[')
                                    .trim_end_matches(']'),
                            )
                        {
                            entry.next_entry = Some(n.text_range().start());
                            found = true;
                            break;
                        }
                    }
                }

                if !found {
                    entry.next_entry = end.clone();
                }
            }

            if let TABLE_ARRAY_HEADER = entry.syntax.kind() {
                let mut found = false;

                let entry_header_string = entry.syntax.to_string();

                for n in root_syntax.children() {
                    if n.text_range().start() < entry.syntax.text_range().end() {
                        continue;
                    }

                    let new_header_string = n.to_string();

                    if let TABLE_ARRAY_HEADER = n.kind() {
                        if !new_header_string
                            .trim_start_matches('[')
                            .trim_end_matches(']')
                            .starts_with(
                                entry_header_string
                                    .trim_start_matches('[')
                                    .trim_end_matches(']'),
                            )
                        {
                            entry.next_entry = Some(n.text_range().start());
                            found = true;
                            break;
                        }
                    }
                }

                if !found {
                    entry.next_entry = end.clone();
                }
            }

            match &mut entry.value {
                ValueNode::Table(t) => {
                    t.next_entry = entry.next_entry.clone();
                    t.entries.set_table_spans(root_syntax, end);
                }
                ValueNode::Array(arr) => {
                    if arr.is_array_of_tables() {
                        arr.set_table_spans(root_syntax, end)
                    }
                }
                _ => {}
            }
        }
    }

    fn from_map(map: IndexMap<KeyNode, EntryNode>) -> Self {
        Entries(
            map.into_iter()
                .map(|(k, mut e)| {
                    e.key = k;
                    e
                })
                .collect(),
        )
    }

    /// Merges entries into tables, merges tables where possible,
    /// creates arrays from arrays of tables.
    ///
    /// Any errors are pushed into errors and the affected
    /// values are dropped.
    ///
    /// The resulting entries are not normalized
    /// and will still contain dotted keys.
    ///
    /// This function assumes that arrays of tables have correct
    /// indices in order without skips and will panic otherwise.
    /// It also doesn't care about table duplicates, and will happily merge them.
    fn merge(&mut self, errors: &mut Vec<Error>) {
        // The new entry keys all will have indices of 0 as arrays are merged.
        let mut new_entries: Vec<EntryNode> = Vec::with_capacity(self.0.len());

        // We try to merge or insert all entries.
        for mut entry in mem::take(&mut self.0) {
            // We don't care about the exact index after this point,
            // everything should be in the correct order.
            entry.key = entry.key.with_index(0);

            let mut should_insert = true;

            for existing_entry in &mut new_entries {
                // If false, the entry was already merged
                // or a merge failure happened.
                match Entries::merge_entry(existing_entry, &entry, errors) {
                    Ok(merged) => {
                        if merged {
                            should_insert = false;
                            break;
                        }
                    }
                    Err(err) => {
                        errors.push(err);
                        should_insert = false;
                        break;
                    }
                }
            }

            if should_insert {
                // Transform array of tables into array
                entry.value = match entry.value {
                    ValueNode::Table(mut t) => {
                        if t.array {
                            t.array = false;
                            ValueNode::Array(ArrayNode {
                                syntax: t.syntax.clone(),
                                items: vec![ValueNode::Table(t)],
                                next_header_start: None,
                                tables: true,
                            })
                        } else {
                            ValueNode::Table(t)
                        }
                    }
                    v => v,
                };

                new_entries.push(entry);
            }
        }

        self.0 = new_entries;
    }

    /// Normalizes all dotted keys into nested
    /// pseudo-tables.
    fn normalize(&mut self) {
        let mut entries_list = vec![&mut self.0];

        while let Some(entries) = entries_list.pop() {
            for entry in entries.iter_mut() {
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

    /// Tries to merge entries into each other,
    /// old will always have the final result.
    ///
    /// It expects arrays of tables to be in order.
    ///
    /// Returns Ok(true) on a successful merge.
    /// Returns Ok(false) if the entries shouldn't be merged.
    /// Returns Err(...) if the entries should be merged, but an error ocurred.
    fn merge_entry(
        old_entry: &mut EntryNode,
        new_entry: &EntryNode,
        errors: &mut Vec<Error>,
    ) -> Result<bool, Error> {
        let old_key = old_entry.key.clone();
        let new_key = new_entry.key.clone();

        // Try to merge new into old first
        if old_key.is_part_of(&new_key) {
            match &mut old_entry.value {
                // There should be no conflicts, and duplicates
                // should be handled before reaching this point.
                ValueNode::Table(t) => {
                    if t.is_inline() {
                        return Err(Error::InlineTable {
                            target: old_entry.key.clone(),
                            key: new_entry.key.clone(),
                        });
                    }

                    let mut to_insert = new_entry.clone();
                    to_insert.key = new_key.without_prefix(&old_key);
                    t.entries.0.push(to_insert);

                    // FIXME(recursion)
                    // It shouldn't be a problem here, but I mark it anyway.
                    t.entries.merge(errors);

                    Ok(true)
                }
                ValueNode::Array(old_arr) => {
                    if !old_arr.tables {
                        return Err(Error::ExpectedTableArray {
                            target: old_entry.key.clone(),
                            key: new_entry.key.clone(),
                        });
                    }

                    let mut final_entry = new_entry.clone();

                    match &mut final_entry.value {
                        ValueNode::Table(new_t) => {
                            if old_key.eq_keys(&new_key) && new_t.array {
                                new_t.array = false;
                                old_arr.items.push(final_entry.value);
                                Ok(true)
                            } else {
                                match old_arr.items.last_mut().unwrap() {
                                    ValueNode::Table(arr_t) => {
                                        let mut to_insert = new_entry.clone();
                                        to_insert.key = new_key.without_prefix(&old_key);

                                        arr_t.entries.0.push(to_insert);

                                        // FIXME(recursion)
                                        // It shouldn't be a problem here, but I mark it anyway.
                                        arr_t.entries.merge(errors);
                                        Ok(true)
                                    }
                                    _ => panic!("expected array of tables"),
                                }
                            }
                        }
                        ValueNode::Empty => panic!("empty value"),
                        _ => {
                            match old_arr.items.last_mut().unwrap() {
                                ValueNode::Table(arr_t) => {
                                    let mut to_insert = new_entry.clone();
                                    to_insert.key = new_key.without_prefix(&old_key);

                                    arr_t.entries.0.push(to_insert);

                                    // FIXME(recursion)
                                    // It shouldn't be a problem here, but I mark it anyway.
                                    arr_t.entries.merge(errors);
                                    Ok(true)
                                }
                                _ => panic!("expected array of tables"),
                            }
                        }
                    }
                }
                ValueNode::Empty => panic!("empty value"),
                _ => Err(Error::ExpectedTable {
                    target: old_entry.key.clone(),
                    key: new_entry.key.clone(),
                }),
            }

        // Same but the other way around.
        } else if new_key.is_part_of(&old_key) {
            let mut new_old = new_entry.clone();

            match Entries::merge_entry(&mut new_old, &old_entry, errors) {
                Ok(merged) => {
                    if merged {
                        *old_entry = new_old;
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
                Err(e) => Err(e),
            }

        // They might still share a prefix,
        // in that case a pseudo-table must be created.
        } else {
            let common_count = old_entry.key().common_prefix_count(new_entry.key());

            if common_count > 0 {
                let common_prefix = old_entry.key().clone().outer(common_count);

                let mut a = old_entry.clone();
                a.key = a.key.without_prefix(&common_prefix);

                let mut b = new_entry.clone();
                b.key = b.key.without_prefix(&common_prefix);

                old_entry.key = common_prefix;
                old_entry.value = ValueNode::Table(TableNode {
                    syntax: old_entry.syntax.clone(),
                    next_entry: None,
                    array: false,
                    pseudo: true,
                    entries: Entries(vec![a, b]),
                });
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

impl IntoIterator for Entries {
    type Item = EntryNode;
    type IntoIter = std::vec::IntoIter<EntryNode>;
    fn into_iter(self) -> std::vec::IntoIter<EntryNode> {
        self.0.into_iter()
    }
}

impl FromIterator<EntryNode> for Entries {
    fn from_iter<T: IntoIterator<Item = EntryNode>>(iter: T) -> Self {
        let i = iter.into_iter();
        let hint = i.size_hint();

        let len = match hint.1 {
            None => hint.0,
            Some(l) => l,
        };

        let mut entries = Vec::with_capacity(len);

        for entry in i {
            entries.push(entry);
        }

        Entries(entries)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayNode {
    syntax: SyntaxNode,
    tables: bool,
    items: Vec<ValueNode>,

    // Offset of the next entry if any,
    // this is needed because tables span
    // longer than their actual syntax in TOML.
    next_header_start: Option<TextSize>,
}

impl ArrayNode {
    pub fn items(&self) -> &[ValueNode] {
        &self.items
    }

    pub fn into_items(self) -> Vec<ValueNode> {
        self.items
    }

    pub fn text_range(&self) -> TextRange {
        let mut range = self.syntax.text_range();

        for item in &self.items {
            range = range.cover(item.text_range())
        }

        if let Some(r) = self.next_header_start.as_ref() {
            range = range.cover_offset(*r);
        }

        range
    }

    pub fn is_array_of_tables(&self) -> bool {
        self.tables
    }

    // Top level tables and arrays of tables
    // need to span across whitespace as well.
    fn set_table_spans(&mut self, root_syntax: &SyntaxNode, end: Option<TextSize>) {
        if !self.tables {
            return;
        }

        for value in &mut self.items {
            let table = match value {
                ValueNode::Table(t) => t,
                _ => panic!("expected table"),
            };

            let mut found = false;

            let entry_header_string = table.syntax.to_string();

            for n in root_syntax.children() {
                if n.text_range().start() < table.syntax.text_range().end() {
                    continue;
                }

                let new_header_string = n.to_string();

                if let TABLE_ARRAY_HEADER = n.kind() {
                    if !new_header_string
                        .trim_start_matches('[')
                        .trim_end_matches(']')
                        .starts_with(
                            entry_header_string
                                .trim_start_matches('[')
                                .trim_end_matches(']'),
                        )
                        || new_header_string == entry_header_string
                    {
                        table.next_entry = Some(n.text_range().start());
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                table.next_entry = end.clone();
            }

            table.entries.set_table_spans(root_syntax, end);
        }
    }
}

impl Common for ArrayNode {
    fn syntax(&self) -> SyntaxElement {
        self.syntax.clone().into()
    }

    fn text_range(&self) -> TextRange {
        let mut range = self.syntax.text_range();

        for item in &self.items {
            range = range.cover(item.text_range())
        }

        if let Some(r) = self.next_header_start.as_ref() {
            range = range.cover_offset(*r);
        }

        range
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
                next_header_start: None,
                tables: false,
                syntax: syntax.into_node().unwrap(),
            }),
            TABLE_ARRAY_HEADER => Some(Self {
                items: Vec::new(),
                next_header_start: None,
                tables: false,
                syntax: syntax.into_node().unwrap(),
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntryNode {
    syntax: SyntaxNode,
    key: KeyNode,
    value: ValueNode,
    // Offset of the next entry if any,
    // this is needed because tables span
    // longer than their actual syntax in TOML.
    next_entry: Option<TextSize>,
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

    pub fn token_eq_text_range(&self) -> Option<TextRange> {
        self.syntax.children_with_tokens().find_map(|t| {
            if t.kind() == EQ {
                Some(t.text_range())
            } else {
                None
            }
        })
    }

    /// Turns a dotted key into nested pseudo-tables.
    fn normalize(&mut self) {
        while self.key.key_count() > 1 {
            let new_key = self.key.clone().prefix();
            let inner_key = self.key.clone().last();

            let value = mem::take(&mut self.value);

            // We have to keep track of it in the pseudo-table.
            let is_array_table = match &value {
                ValueNode::Table(t) => t.is_part_of_array(),
                _ => false,
            };

            let inner_entry = EntryNode {
                syntax: self.syntax.clone(),
                key: inner_key.clone(),
                next_entry: None,
                value,
            };

            let mut entries = Entries(Vec::with_capacity(1));

            entries.0.push(inner_entry);

            self.value = ValueNode::Table(TableNode {
                syntax: inner_key.syntax.clone(),
                array: is_array_table,
                next_entry: None,
                pseudo: true,
                entries,
            });
            self.key = new_key;
        }
    }
}

impl Common for EntryNode {
    fn syntax(&self) -> SyntaxElement {
        self.syntax.clone().into()
    }

    fn text_range(&self) -> TextRange {
        let r = self.syntax.text_range();

        match self.next_entry {
            Some(next_entry) => r.cover_offset(next_entry),
            None => r,
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
                .and_then(Cast::cast)
                .unwrap_or(ValueNode::Invalid(None));

            Some(Self {
                key: key.unwrap(),
                value: val,
                next_entry: None,
                syntax: element.into_node().unwrap(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyNode {
    syntax: SyntaxNode,

    // To avoid cloning the idents vec,
    // we mask them instead.
    mask_left: usize,
    mask_right: usize,

    // The visible ident count, can never be 0
    mask_visible: usize,

    // Hash and equality is based on only
    // the string values of the idents.
    idents: Rc<Vec<SyntaxToken>>,

    // This also contributes to equality and hashes.
    //
    // It is only used to differentiate arrays of tables
    // during parsing.
    index: usize,
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
        self.idents().map(|t| {
            let mut s = t.text().as_str();

            if s.starts_with('\"') || s.starts_with('\'') {
                s = &s[1..s.len() - 1];
            }

            s
        })
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

        for (a, b) in self.keys_str().zip(other.keys_str()) {
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
    /// e.g.: outer.inner => super
    /// there will be at least one ident remaining
    pub fn outer(mut self, n: usize) -> Self {
        let skip = usize::min(
            self.mask_visible - 1,
            self.mask_visible.checked_sub(n).unwrap_or_default(),
        );
        self.mask_right += skip;
        self.mask_visible -= skip;
        self
    }

    /// skips n idents from the left,
    /// e.g.: outer.inner => inner
    /// there will be at least one ident remaining
    pub fn inner(mut self, n: usize) -> Self {
        let skip = usize::min(self.mask_visible - 1, n);
        self.mask_left += skip;
        self.mask_visible -= skip;
        self
    }

    /// Counts the shared prefix keys, ignores index
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

    /// Eq that ignores the index of the key
    pub fn eq_keys(&self, other: &KeyNode) -> bool {
        self.key_count() == other.key_count() && self.is_part_of(other)
    }

    /// Prepends other's idents, and also inherits
    /// other's index.
    fn with_prefix(mut self, other: &KeyNode) -> Self {
        // We have to modify our existing vec here
        let mut new_idents = (*self.idents).clone();
        new_idents.truncate(new_idents.len() - self.mask_right);
        new_idents.drain(0..self.mask_left);
        new_idents.splice(0..0, other.idents().cloned());

        self.mask_visible = new_idents.len();
        self.idents = Rc::new(new_idents);
        self.mask_left = 0;
        self.mask_right = 0;
        self.index = other.index;
        self
    }

    /// Removes other's prefix from self
    fn without_prefix(self, other: &KeyNode) -> Self {
        let count = self.common_prefix_count(other);

        if count > 0 {
            self.inner(count)
        } else {
            self
        }
    }

    fn with_index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    fn prefix(self) -> Self {
        let count = self.key_count();
        self.outer(count - 1)
    }

    fn last(self) -> Self {
        let count = self.key_count();
        self.inner(count)
    }
}

impl Common for KeyNode {
    fn syntax(&self) -> SyntaxElement {
        self.syntax.clone().into()
    }

    fn text_range(&self) -> TextRange {
        self.idents()
            .fold(self.idents().next().unwrap().text_range(), |r, t| {
                r.cover(t.text_range())
            })
    }
}

impl PartialEq for KeyNode {
    fn eq(&self, other: &Self) -> bool {
        self.eq_keys(other) && self.index == other.index
    }
}

impl Eq for KeyNode {}

// Needed because of custom PartialEq
impl Hash for KeyNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for s in self.keys_str() {
            s.hash(state)
        }
        self.index.hash(state)
    }
}

impl core::fmt::Display for KeyNode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.full_key_string().fmt(f)
    }
}

impl Cast for KeyNode {
    fn cast(element: SyntaxElement) -> Option<Self> {
        if element.kind() != KEY {
            None
        } else {
            element.into_node().and_then(|n| {
                let i: Vec<SyntaxToken> = n
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
                    index: 0,
                    syntax: n,
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
    Invalid(Option<SyntaxElement>),
    Empty,
}

impl Default for ValueNode {
    fn default() -> Self {
        ValueNode::Empty
    }
}

impl ValueNode {
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

impl Common for ValueNode {
    fn syntax(&self) -> SyntaxElement {
        match self {
            ValueNode::Bool(v) => v.syntax(),
            ValueNode::String(v) => v.syntax(),
            ValueNode::Integer(v) => v.syntax(),
            ValueNode::Float(v) => v.syntax(),
            ValueNode::Array(v) => v.syntax(),
            ValueNode::Date(v) => v.syntax(),
            ValueNode::Table(v) => v.syntax(),
            _ => panic!("empty value"),
        }
    }

    fn text_range(&self) -> TextRange {
        match self {
            ValueNode::Bool(v) => v.text_range(),
            ValueNode::String(v) => v.text_range(),
            ValueNode::Integer(v) => v.text_range(),
            ValueNode::Float(v) => v.text_range(),
            ValueNode::Array(v) => v.text_range(),
            ValueNode::Date(v) => v.text_range(),
            ValueNode::Table(v) => v.text_range(),
            ValueNode::Invalid(n) => n.as_ref().map(|n| n.text_range()).unwrap_or_default(),
            _ => panic!("empty value"),
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            ValueNode::Invalid(_) | ValueNode::Empty => false,
            _ => true,
        }
    }
}

impl core::fmt::Display for ValueNode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ValueNode::Bool(v) => v.fmt(f),
            ValueNode::String(v) => v.fmt(f),
            ValueNode::Integer(v) => v.fmt(f),
            ValueNode::Float(v) => v.fmt(f),
            ValueNode::Array(v) => v.fmt(f),
            ValueNode::Date(v) => v.fmt(f),
            ValueNode::Table(v) => v.fmt(f),
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
            .or(Some(ValueNode::Invalid(Some(element))))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IntegerRepr {
    Dec,
    Bin,
    Oct,
    Hex,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntegerNode {
    syntax: SyntaxToken,
    repr: IntegerRepr,
}

impl IntegerNode {
    pub fn repr(&self) -> IntegerRepr {
        self.repr
    }

    pub fn text_range(&self) -> TextRange {
        self.syntax.text_range()
    }
}

impl Common for IntegerNode {
    fn syntax(&self) -> SyntaxElement {
        self.syntax.clone().into()
    }

    fn text_range(&self) -> TextRange {
        self.syntax.text_range()
    }
}

impl Cast for IntegerNode {
    fn cast(element: SyntaxElement) -> Option<Self> {
        match element.kind() {
            INTEGER => Some(IntegerNode {
                syntax: element.into_token().unwrap(),
                repr: IntegerRepr::Dec,
            }),
            INTEGER_BIN => Some(IntegerNode {
                syntax: element.into_token().unwrap(),
                repr: IntegerRepr::Bin,
            }),
            INTEGER_HEX => Some(IntegerNode {
                syntax: element.into_token().unwrap(),
                repr: IntegerRepr::Hex,
            }),
            INTEGER_OCT => Some(IntegerNode {
                syntax: element.into_token().unwrap(),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringNode {
    syntax: SyntaxToken,
    kind: StringKind,

    /// Unescaped (and trimmed where defined by TOML) value.
    content: String,
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

impl Common for StringNode {
    fn syntax(&self) -> SyntaxElement {
        self.syntax.clone().into()
    }

    fn text_range(&self) -> TextRange {
        self.syntax.text_range()
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
                syntax: element.into_token().unwrap(),
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
                syntax: element.into_token().unwrap(),
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
                syntax: element.into_token().unwrap(),
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
                syntax: element.into_token().unwrap(),
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Error {
    DuplicateKey { first: KeyNode, second: KeyNode },
    ExpectedTableArray { target: KeyNode, key: KeyNode },
    ExpectedTable { target: KeyNode, key: KeyNode },
    TopLevelTableDefined { table: KeyNode, key: KeyNode },
    InlineTable { target: KeyNode, key: KeyNode },
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
                &first.text_range(),
                &second.full_key_string(),
                &second.text_range()
            ),
            Error::ExpectedTable { target, key } => write!(
                f,
                "Expected \"{}\" ({:?}) to be a table, but it is not, required by \"{}\" ({:?})",
                &target.full_key_string(),
                &target.text_range(),
                &key.full_key_string(),
                &key.text_range()
            ),
            Error::TopLevelTableDefined { table, key } => write!(
                f,
                "full table definition \"{}\" ({:?}) conflicts with dotted keys \"{}\" ({:?})",
                &table.full_key_string(),
                &table.text_range(),
                &key.full_key_string(),
                &key.text_range()
            ),
            Error::ExpectedTableArray { target, key } => write!(
                f,
                "\"{}\" ({:?}) conflicts with array of tables: \"{}\" ({:?})",
                &target.full_key_string(),
                &target.text_range(),
                &key.full_key_string(),
                &key.text_range()
            ),
            Error::InlineTable { target, key } => write!(
                f,
                "inline tables cannot be modified: \"{}\" ({:?}), modification attempted here: \"{}\" ({:?})",
                &target.full_key_string(),
                &target.text_range(),
                &key.full_key_string(),
                &key.text_range()
            ),
            Error::Spanned { range, message } => write!(f, "{} ({:?})", message, range),
            Error::Generic(s) => s.fmt(f),
        }
    }
}
impl std::error::Error for Error {}

#[test]
fn asd() {
    let src = r#"
asd.bsd.csd.dsd.esd.fsd = 1
"#;

    let _p = crate::parser::parse(src).into_dom();
}
