# serde.visit_seq

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\serde.rs` (lines 184-222)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut array = ArrayInner {
            errors: Default::default(),
            syntax: Default::default(),
            kind: ArrayKind::Inline,
            items: Default::default(),
        };

        let mut all_table = true;

        array.items.update(|items| loop {
            match seq.next_element::<Node>() {
                Ok(Some(node)) => {
                    if !node.is_table() {
                        all_table = false;
                    }

                    items.push(node);
                }
                Ok(None) => break,
                Err(error) => {
                    tracing::debug!(%error, "invalid TOML value");
                }
            }
        });

        if array.items.read().is_empty() {
            all_table = false;
        }

        if all_table {
            array.kind = ArrayKind::Tables;
        }

        Ok(array.wrap().into())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut array = ArrayInner {
            errors: Default::default(),
            syntax: Default::default(),
            kind: ArrayKind::Inline,
            items: Default::default(),
        };

        let mut all_table = true;

        array.items.update(|items| loop {
            match seq.next_element::<Node>() {
                Ok(Some(node)) => {
                    if !node.is_table() {
                        all_table = false;
                    }

                    items.push(node);
                }
                Ok(None) => break,
                Err(error) => {
                    tracing::debug!(%error, "invalid TOML value");
                }
            }
        });

        if array.items.read().is_empty() {
            all_table = false;
        }

        if all_table {
            array.kind = ArrayKind::Tables;
        }

        Ok(array.wrap().into())
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
