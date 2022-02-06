use super::{
    error::Error,
    node::{
        Array, ArrayInner, ArrayKind, Bool, BoolInner, DateTime, DateTimeInner, DomNode, Float,
        FloatInner, Integer, IntegerInner, IntegerRepr, Invalid, InvalidInner, Key, KeyInner, Node,
        Str, StrInner, StrRepr, Table, TableInner, TableKind,
    },
    Comment, Keys,
};
use crate::{
    private::Sealed,
    syntax::{SyntaxElement, SyntaxKind::*},
    util::{iter::ExactIterExt, shared::Shared},
};
use either::Either;

pub trait FromSyntax: Sized + Sealed {
    fn from_syntax(syntax: SyntaxElement) -> Self;
}

impl FromSyntax for Node {
    fn from_syntax(syntax: SyntaxElement) -> Self {
        match syntax.kind() {
            VALUE => {
                if let Some(child) = syntax.as_node().and_then(|n| n.first_child_or_token()) {
                    Node::from_syntax(child)
                } else {
                    Invalid::from_syntax(syntax).into()
                }
            }
            STRING | MULTI_LINE_STRING | STRING_LITERAL | MULTI_LINE_STRING_LITERAL => {
                Str::from_syntax(syntax).into()
            }
            INTEGER | INTEGER_HEX | INTEGER_OCT | INTEGER_BIN => {
                Integer::from_syntax(syntax).into()
            }
            FLOAT => Float::from_syntax(syntax).into(),
            BOOL => Bool::from_syntax(syntax).into(),
            DATE_TIME_OFFSET | DATE_TIME_LOCAL | DATE | TIME => {
                DateTime::from_syntax(syntax).into()
            }
            ARRAY => Array::from_syntax(syntax).into(),
            INLINE_TABLE | ROOT => Table::from_syntax(syntax).into(),
            _ => Invalid::from_syntax(syntax).into(),
        }
    }
}

impl FromSyntax for Table {
    fn from_syntax(syntax: SyntaxElement) -> Self {
        let mut errors = Vec::new();
        match syntax.kind() {
            ROOT => root_from_syntax(syntax),
            TABLE_HEADER | TABLE_ARRAY_HEADER => TableInner {
                errors: errors.into(),
                syntax: Some(syntax),
                header: true,
                kind: TableKind::Regular,
                entries: Default::default(),
            }
            .wrap(),
            INLINE_TABLE => {
                let table = TableInner {
                    errors: errors.into(),
                    header: false,
                    syntax: Some(syntax.clone()),
                    kind: TableKind::Inline,
                    entries: Default::default(),
                }
                .wrap();

                let entries = syntax
                    .as_node()
                    .map(|n| n.children().map(|syntax| entry_from_syntax(syntax.into())));

                if let Some(entries) = entries {
                    for (key, node) in entries {
                        table.add_entry(key, node);
                    }
                }

                table
            }
            _ => {
                errors.push(Error::UnexpectedSyntax {
                    syntax: syntax.clone(),
                });
                TableInner {
                    errors: errors.into(),
                    header: false,
                    syntax: Some(syntax),
                    kind: TableKind::Regular,
                    entries: Default::default(),
                }
                .into()
            }
        }
    }
}

impl Table {
    fn pseudo(key: &Key, header: bool) -> Self {
        TableInner {
            errors: Default::default(),
            syntax: key.syntax().cloned(),
            header,
            kind: TableKind::Pseudo,
            entries: Default::default(),
        }
        .wrap()
    }
}

impl FromSyntax for Array {
    fn from_syntax(syntax: SyntaxElement) -> Self {
        let mut errors = Vec::new();
        match syntax.kind() {
            ARRAY => ArrayInner {
                errors: errors.into(),
                syntax: Some(syntax.clone()),
                kind: ArrayKind::Inline,
                items: Shared::new(
                    syntax
                        .as_node()
                        .map(|n| {
                            n.children()
                                .map(|syntax| Node::from_syntax(syntax.into()))
                                .collect()
                        })
                        .unwrap_or_default(),
                ),
            }
            .into(),
            TABLE_ARRAY_HEADER => ArrayInner {
                errors: errors.into(),
                syntax: Some(syntax),
                kind: ArrayKind::Tables,
                items: Default::default(),
            }
            .into(),
            _ => {
                errors.push(Error::UnexpectedSyntax {
                    syntax: syntax.clone(),
                });
                ArrayInner {
                    errors: errors.into(),
                    syntax: Some(syntax),
                    kind: ArrayKind::Inline,
                    items: Default::default(),
                }
                .into()
            }
        }
    }
}

impl FromSyntax for Key {
    fn from_syntax(syntax: SyntaxElement) -> Self {
        match syntax.kind() {
            IDENT | STRING | STRING_LITERAL => KeyInner {
                errors: Shared::default(),
                is_valid: true,
                syntax: Some(syntax),
                value: Default::default(),
                additional_syntaxes: Default::default(),
            }
            .into(),
            _ => Key::from_syntax_invalid(syntax),
        }
    }
}

impl Key {
    fn from_syntax_invalid(syntax: SyntaxElement) -> Self {
        KeyInner {
            errors: Shared::new(Vec::from([Error::UnexpectedSyntax {
                syntax: syntax.clone(),
            }])),
            is_valid: false,
            value: Default::default(),
            syntax: Some(syntax),
            additional_syntaxes: Default::default(),
        }
        .wrap()
    }
}

impl FromSyntax for DateTime {
    fn from_syntax(syntax: SyntaxElement) -> Self {
        let mut errors = Vec::new();
        match syntax.kind() {
            DATE_TIME_OFFSET | DATE_TIME_LOCAL | DATE | TIME => DateTimeInner {
                errors: errors.into(),
                syntax: Some(syntax),
                value: Default::default(),
            }
            .into(),
            _ => {
                errors.push(Error::UnexpectedSyntax {
                    syntax: syntax.clone(),
                });
                DateTimeInner {
                    errors: errors.into(),
                    syntax: Some(syntax),
                    value: Default::default(),
                }
                .into()
            }
        }
    }
}

impl FromSyntax for Bool {
    fn from_syntax(syntax: SyntaxElement) -> Self {
        let mut errors = Vec::new();
        match syntax.kind() {
            BOOL => BoolInner {
                errors: errors.into(),
                value: Default::default(),
                syntax: Some(syntax),
            }
            .into(),
            _ => {
                errors.push(Error::UnexpectedSyntax {
                    syntax: syntax.clone(),
                });
                BoolInner {
                    errors: errors.into(),
                    value: Default::default(),
                    syntax: Some(syntax),
                }
                .into()
            }
        }
    }
}

impl FromSyntax for Float {
    fn from_syntax(syntax: SyntaxElement) -> Self {
        let mut errors = Vec::new();
        match syntax.kind() {
            FLOAT => FloatInner {
                errors: errors.into(),
                value: Default::default(),
                syntax: Some(syntax),
            }
            .into(),
            _ => {
                errors.push(Error::UnexpectedSyntax {
                    syntax: syntax.clone(),
                });
                FloatInner {
                    errors: errors.into(),
                    value: Default::default(),
                    syntax: Some(syntax),
                }
                .into()
            }
        }
    }
}

impl FromSyntax for Integer {
    fn from_syntax(syntax: SyntaxElement) -> Self {
        let mut errors = Vec::new();
        match syntax.kind() {
            INTEGER => IntegerInner {
                errors: errors.into(),
                syntax: Some(syntax),
                value: Default::default(),
                repr: IntegerRepr::Dec,
            }
            .into(),
            INTEGER_BIN => IntegerInner {
                errors: errors.into(),
                syntax: Some(syntax),
                value: Default::default(),
                repr: IntegerRepr::Bin,
            }
            .into(),
            INTEGER_HEX => IntegerInner {
                errors: errors.into(),
                syntax: Some(syntax),
                value: Default::default(),
                repr: IntegerRepr::Hex,
            }
            .into(),
            INTEGER_OCT => IntegerInner {
                errors: errors.into(),
                syntax: Some(syntax),
                value: Default::default(),
                repr: IntegerRepr::Oct,
            }
            .into(),
            _ => {
                errors.push(Error::UnexpectedSyntax {
                    syntax: syntax.clone(),
                });
                IntegerInner {
                    errors: errors.into(),
                    syntax: Some(syntax),
                    value: Default::default(),
                    repr: IntegerRepr::Dec,
                }
                .into()
            }
        }
    }
}

impl FromSyntax for Str {
    fn from_syntax(syntax: SyntaxElement) -> Self {
        let mut errors = Vec::new();
        match syntax.kind() {
            STRING => StrInner {
                errors: errors.into(),
                syntax: Some(syntax),
                repr: StrRepr::Basic,
                value: Default::default(),
            }
            .into(),
            STRING_LITERAL => StrInner {
                errors: errors.into(),
                syntax: Some(syntax),
                repr: StrRepr::Literal,
                value: Default::default(),
            }
            .into(),
            MULTI_LINE_STRING => StrInner {
                errors: errors.into(),
                syntax: Some(syntax),
                repr: StrRepr::MultiLine,
                value: Default::default(),
            }
            .into(),
            MULTI_LINE_STRING_LITERAL => StrInner {
                errors: errors.into(),
                syntax: Some(syntax),
                repr: StrRepr::MultiLineLiteral,
                value: Default::default(),
            }
            .into(),
            _ => {
                errors.push(Error::UnexpectedSyntax {
                    syntax: syntax.clone(),
                });
                StrInner {
                    errors: errors.into(),
                    syntax: Some(syntax),
                    repr: StrRepr::Basic,
                    value: Default::default(),
                }
                .into()
            }
        }
    }
}

impl FromSyntax for Invalid {
    fn from_syntax(syntax: SyntaxElement) -> Self {
        let errors = Vec::from([Error::UnexpectedSyntax {
            syntax: syntax.clone(),
        }]);
        InvalidInner {
            errors: errors.into(),
            syntax: Some(syntax),
        }
        .into()
    }
}

impl Sealed for Keys {}
impl FromSyntax for Keys {
    fn from_syntax(syntax: SyntaxElement) -> Self {
        Keys::new(keys_from_syntax(&syntax).map(Into::into))
    }
}

impl Sealed for Comment {}
impl FromSyntax for Comment {
    fn from_syntax(syntax: SyntaxElement) -> Self {
        Self {
            syntax: Some(syntax),
            value: Default::default(),
        }
    }
}

pub(crate) fn keys_from_syntax(syntax: &SyntaxElement) -> impl ExactSizeIterator<Item = Key> {
    assert!(syntax.kind() == KEY);

    syntax
        .as_node()
        .map(|n| n.children_with_tokens())
        .map(|children| {
            let iter = children.filter(|c| c.kind() == IDENT);
            let c = iter.clone().count();
            Either::Left(iter.map(Key::from_syntax).exactly(c))
        })
        .unwrap_or_else(|| Either::Right(core::iter::empty()))
}

fn entry_from_syntax(syntax: SyntaxElement) -> (Key, Node) {
    assert!(syntax.kind() == ENTRY);

    let mut keys = keys_from_syntax(
        &syntax
            .as_node()
            .and_then(|n| n.first_child())
            .map(Into::into)
            .unwrap_or_else(|| syntax.clone()),
    );
    let first_key = keys
        .next()
        .unwrap_or_else(|| Key::from_syntax_invalid(syntax.clone()));

    let mut value = syntax
        .as_node()
        .and_then(|n| n.first_child())
        .and_then(|k| k.next_sibling())
        .map(|n| Node::from_syntax(n.into()))
        .unwrap_or_else(|| Invalid::from_syntax(syntax).into());

    let mut top_pseudo_table: Option<Table> = None;
    let mut last_pseudo_table: Option<Table> = None;
    while let Some(pseudo_key) = keys.next() {
        let new_pseudo_table = Table::pseudo(&pseudo_key, false);

        match (top_pseudo_table.take(), last_pseudo_table.take()) {
            (None, None) => {
                let top_pt = Table::pseudo(&first_key, false);
                top_pseudo_table = Some(top_pt.clone());

                if keys.len() == 0 {
                    top_pt.add_entry(pseudo_key, value);
                    value = top_pt.into();
                } else {
                    top_pt.add_entry(pseudo_key, new_pseudo_table.clone().into());
                    last_pseudo_table = Some(new_pseudo_table.clone());
                }
            }
            (Some(top_pt), Some(last_pt)) => {
                if keys.len() == 0 {
                    last_pt.add_entry(pseudo_key, value);
                    value = top_pt.into();
                    break;
                } else {
                    top_pseudo_table = Some(top_pt);
                    last_pt.add_entry(pseudo_key, new_pseudo_table.clone().into());
                    last_pseudo_table = Some(new_pseudo_table);
                }
            }
            _ => unreachable!(),
        }
    }

    (first_key, value)
}

fn root_from_syntax(syntax: SyntaxElement) -> Table {
    let node = match syntax.as_node() {
        Some(n) => n,
        None => {
            return TableInner {
                errors: Vec::from([Error::UnexpectedSyntax {
                    syntax: syntax.clone(),
                }])
                .into(),
                syntax: Some(syntax),
                header: false,
                kind: TableKind::Regular,
                entries: Default::default(),
            }
            .into()
        }
    };

    let root_table = TableInner {
        errors: Default::default(),
        syntax: Some(syntax.clone()),
        header: false,
        kind: TableKind::Regular,
        entries: Default::default(),
    }
    .wrap();

    let mut current_table: Table = root_table.clone();

    for child in node.children() {
        match child.kind() {
            table_kind @ (TABLE_ARRAY_HEADER | TABLE_HEADER) => {
                let mut keys = keys_from_syntax(
                    &child
                        .first_child()
                        .map(Into::into)
                        .unwrap_or_else(|| child.clone().into()),
                );
                current_table = root_table.clone();

                match table_kind {
                    TABLE_HEADER => {
                        while let Some(key) = keys.next() {
                            if keys.len() == 0 {
                                let new_table = Table::from_syntax(child.into());
                                let current_entries = current_table.inner.entries.read();
                                match current_entries.lookup.get_key_value(&key) {
                                    Some((k, Node::Table(t))) => {
                                        if let Some(syntax) = key.syntax() {
                                            k.inner
                                                .additional_syntaxes
                                                .update(|s| s.push(syntax.clone()));
                                        }

                                        if t.inner.kind != TableKind::Pseudo || !t.inner.header {
                                            t.inner.errors.update(|errors| {
                                                errors.push(Error::ConflictingKeys {
                                                    key: key.clone(),
                                                    other: k.clone(),
                                                })
                                            });
                                        }
                                        current_table = t.clone();
                                    }
                                    Some((k, _)) => {
                                        current_table.inner.errors.update(|errors| {
                                            errors.push(Error::ConflictingKeys {
                                                key: key.clone(),
                                                other: k.clone(),
                                            })
                                        });
                                    }
                                    None => {
                                        current_table.add_entry(key, new_table.clone().into());
                                        current_table = new_table;
                                    }
                                }
                                break;
                            } else {
                                current_table = merge_intermediate(key, current_table);
                            }
                        }
                    }
                    TABLE_ARRAY_HEADER => {
                        while let Some(key) = keys.next() {
                            if keys.len() == 0 {
                                let new_table = Table::from_syntax(child.clone().into());

                                current_table.clone().inner.entries.update(|entries| {
                                    if let Some((existing_key, existing_node)) =
                                        entries.lookup.get_key_value(&key)
                                    {
                                        if let Some(key_syntax) = key.syntax() {
                                            existing_key.inner.additional_syntaxes.update(
                                                |key_syntaxes| {
                                                    key_syntaxes.push(key_syntax.clone());
                                                },
                                            );
                                        }

                                        match existing_node {
                                            Node::Array(arr) => {
                                                if arr.inner.kind != ArrayKind::Tables {
                                                    existing_node.errors().update(|errors| {
                                                        errors.push(Error::ExpectedArrayOfTables {
                                                            not_array_of_tables: existing_key
                                                                .clone(),
                                                            required_by: key.clone(),
                                                        })
                                                    });
                                                }

                                                arr.inner.items.update(|items| {
                                                    items.push(new_table.clone().into());
                                                });

                                                current_table = new_table;
                                            }
                                            existing_node => {
                                                existing_node.errors().update(|errors| {
                                                    errors.push(Error::ExpectedArrayOfTables {
                                                        not_array_of_tables: existing_key.clone(),
                                                        required_by: key.clone(),
                                                    })
                                                });
                                            }
                                        }
                                    } else {
                                        let arr = Array::from_syntax(child.into());
                                        arr.inner.items.update(|items| {
                                            items.push(new_table.clone().into());
                                        });
                                        entries.add(key, arr.into());
                                        current_table = new_table;
                                    }
                                });
                                break;
                            } else {
                                current_table = merge_intermediate(key, current_table);
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
            ENTRY => {
                let (key, node) = entry_from_syntax(child.into());
                current_table.add_entry(key, node);
            }
            _ => {}
        }
    }

    root_table
}

/// Merge or create an intermediate dotted key in a top-level table or array.
/// Returns a pseudo-table.
#[must_use]
fn merge_intermediate(key: Key, table: Table) -> Table {
    let mut current_table = table;
    let new_table = Table::pseudo(&key, true);
    current_table.clone().inner.entries.update(|entries| {
        if let Some((existing_key, existing_node)) = entries.lookup.get_key_value(&key) {
            if let Some(key_syntax) = key.syntax() {
                existing_key
                    .inner
                    .additional_syntaxes
                    .update(|key_syntaxes| {
                        key_syntaxes.push(key_syntax.clone());
                    });
            }

            match existing_node {
                Node::Table(existing_table) => {
                    if !matches!(
                        existing_table.inner.kind,
                        TableKind::Regular | TableKind::Pseudo
                    ) {
                        existing_table.inner.errors.update(|errors| {
                            errors.push(Error::ExpectedTable {
                                not_table: existing_key.clone(),
                                required_by: key.clone(),
                            })
                        });
                    }

                    current_table = existing_table.clone();
                }
                Node::Array(existing_array) => {
                    if existing_array.inner.kind != ArrayKind::Tables {
                        existing_array.inner.errors.update(|errors| {
                            errors.push(Error::ExpectedArrayOfTables {
                                not_array_of_tables: existing_key.clone(),
                                required_by: key.clone(),
                            })
                        });
                    }

                    let items = existing_array.inner.items.read();

                    let item = items.iter().last();

                    if let Some(Node::Table(t)) = item {
                        current_table = t.clone();
                    } else {
                        let pt = Table::pseudo(&key, true);
                        existing_array
                            .inner
                            .items
                            .update(|items| items.push(pt.clone().into()));
                        current_table = pt;
                    }
                }
                _ => {
                    current_table.inner.errors.update(|errors| {
                        errors.push(Error::ExpectedTable {
                            not_table: existing_key.clone(),
                            required_by: key.clone(),
                        })
                    });
                    entries.add(key, new_table.clone().into());
                    current_table = new_table.clone();
                }
            }
        } else {
            entries.add(key, new_table.clone().into());
            current_table = new_table.clone();
        }
    });
    current_table
}
