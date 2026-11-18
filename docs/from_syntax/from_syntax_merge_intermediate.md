# from_syntax.merge_intermediate

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\from_syntax.rs` (lines 619-691)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
