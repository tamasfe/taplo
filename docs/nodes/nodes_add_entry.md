# nodes.add_entry

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 87-119)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) fn add_entry(&self, key: Key, node: Node) {
        self.inner.entries.update(|entries| {
            if let Some((existing_key, value)) = entries.lookup.get_key_value(&key) {
                // Merge the two pseudo-tables together.
                if let (Node::Table(existing_table), Node::Table(new_table)) = (value, &node) {
                    if existing_table.inner.kind == TableKind::Pseudo
                        && new_table.inner.kind == TableKind::Pseudo
                    {
                        let new_entries = new_table.entries().read();
                        for (k, n) in new_entries.iter() {
                            if let Some(additional_syntax) = k.syntax() {
                                existing_key.inner.additional_syntaxes.update(|syntaxes| {
                                    syntaxes.push(additional_syntax.clone());
                                });
                            }

                            existing_table.add_entry(k.clone(), n.clone());
                        }
                        return;
                    }
                }

                self.inner.errors.update(|errors| {
                    errors.push(Error::ConflictingKeys {
                        key: key.clone(),
                        other: existing_key.clone(),
                    })
                });
            }

            entries.add(key, node);
        });
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub(crate) fn add_entry(&self, key: Key, node: Node) {
        self.inner.entries.update(|entries| {
            if let Some((existing_key, value)) = entries.lookup.get_key_value(&key) {
                // Merge the two pseudo-tables together.
                if let (Node::Table(existing_table), Node::Table(new_table)) = (value, &node) {
                    if existing_table.inner.kind == TableKind::Pseudo
                        && new_table.inner.kind == TableKind::Pseudo
                    {
                        let new_entries = new_table.entries().read();
                        for (k, n) in new_entries.iter() {
                            if let Some(additional_syntax) = k.syntax() {
                                existing_key.inner.additional_syntaxes.update(|syntaxes| {
                                    syntaxes.push(additional_syntax.clone());
                                });
                            }

                            existing_table.add_entry(k.clone(), n.clone());
                        }
                        return;
                    }
                }

                self.inner.errors.update(|errors| {
                    errors.push(Error::ConflictingKeys {
                        key: key.clone(),
                        other: existing_key.clone(),
                    })
                });
            }

            entries.add(key, node);
        });
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
