# from_syntax.root_from_syntax

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\from_syntax.rs` (lines 460-614)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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

... (truncated)
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
