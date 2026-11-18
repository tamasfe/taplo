# toml_test.serialize

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-cli\src\commands\toml_test.rs` (lines 94-134)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(ty) = self.r#type {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("type", &ty)?;
            map.serialize_entry(
                "value",
                &match self.node {
                    Node::Str(d) => d.value().to_string(),
                    Node::Float(f) if f.value().is_nan() => String::from("nan"),
                    Node::Float(f) if f.value().is_infinite() => f.syntax().unwrap().to_string(),
                    _ => serde_json::to_string(&self.node).map_err(serde::ser::Error::custom)?,
                },
            )?;
            map.end()
        } else {
            match &self.node {
                Node::Array(array) => {
                    let items = array.items().read();

                    let mut seq = serializer.serialize_seq(Some(items.len()))?;
                    for value in &**items {
                        seq.serialize_element(&TomlTestValue::new(value))?;
                    }
                    seq.end()
                }
                Node::Table(table) => {
                    let entries = table.entries().read();

                    let mut map = serializer.serialize_map(Some(entries.len()))?;
                    for (key, value) in entries.iter() {
                        map.serialize_entry(key.value(), &TomlTestValue::new(value))?;
                    }
                    map.end()
                }
                _ => unreachable!(),
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(ty) = self.r#type {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("type", &ty)?;
            map.serialize_entry(
                "value",
                &match self.node {
                    Node::Str(d) => d.value().to_string(),
                    Node::Float(f) if f.value().is_nan() => String::from("nan"),
                    Node::Float(f) if f.value().is_infinite() => f.syntax().unwrap().to_string(),
                    _ => serde_json::to_string(&self.node).map_err(serde::ser::Error::custom)?,
                },
            )?;
            map.end()
        } else {
            match &self.node {
                Node::Array(array) => {
                    let items = array.items().read();

                    let mut seq = serializer.serialize_seq(Some(items.len()))?;
                    for value in &**items {
                        seq.serialize_element(&TomlTestValue::new(value))?;
                    }
                    seq.end()
                }
                Node::Table(table) => {
                    let entries = table.entries().read();

                    let mut map = serializer.serialize_map(Some(entries.len()))?;
                    for (key, value) in entries.iter() {
                        map.serialize_entry(key.value(), &TomlTestValue::new(value))?;
                    }
                    map.end()
                }
                _ => unreachable!(),
            }
        }
    }
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
