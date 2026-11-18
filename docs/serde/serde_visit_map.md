# serde.visit_map

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\serde.rs` (lines 224-249)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let table = TableInner {
            errors: Default::default(),
            syntax: Default::default(),
            header: Default::default(),
            kind: super::node::TableKind::Regular,
            entries: Default::default(),
        };

        table.entries.update(|entries| loop {
            match map.next_entry::<String, Node>() {
                Ok(Some((key, node))) => {
                    entries.add(Key::new(key), node);
                }
                Ok(None) => break,
                Err(error) => {
                    tracing::debug!(%error, "invalid TOML value");
                }
            }
        });

        Ok(table.wrap().into())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let table = TableInner {
            errors: Default::default(),
            syntax: Default::default(),
            header: Default::default(),
            kind: super::node::TableKind::Regular,
            entries: Default::default(),
        };

        table.entries.update(|entries| loop {
            match map.next_entry::<String, Node>() {
                Ok(Some((key, node))) => {
                    entries.add(Key::new(key), node);
                }
                Ok(None) => break,
                Err(error) => {
                    tracing::debug!(%error, "invalid TOML value");
                }
            }
        });

        Ok(table.wrap().into())
    }
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
