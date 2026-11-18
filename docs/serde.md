# Module: serde

**File:** `\crates\taplo\src\dom\serde.rs`

## Overview

Module `serde` contains 17 analyzed entities.

```
use super::node::{
    ArrayInner, ArrayKind, BoolInner, FloatInner, IntegerInner, IntegerValue, Node, StrInner,
    TableInner,
};
use crate::dom::node::Key;
use serde::{
    de::Visitor,
    ser::{Error, SerializeMap, SerializeSeq},
    Deserialize, Serialize, Serializer,
};

impl Serialize for Node {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Node::Table(t) => {
                let entries = t.inner.entries.read();
                let mut map = ser.serialize_map(Some(entries.all.len()))?;

                for (key, entry) in entries.all.iter() {
                    if !entry.is_invalid() {
                        map.serialize_entry(key.value(), entry)?;
                    }
                }

                map.end()
            }
            Node::Array(arr) => {
                let items = arr.inner.items.read();
                let mut seq = ser.serialize_seq(Some(items.len()))?;
                for item in &**items {
                    if !item.is_invalid() {
                        seq.serialize_element(item)?;
                    }
                }
                seq.end()
            }
            Node::Bool(v) => ser.serialize_bool(v.value()),
            Node::Str(v) => ser.serialize_str(v.value()),
            Node::Integer(v) => match v.value() {
                IntegerValue::Negative(v) => ser.serialize_i64(v),
                IntegerValue::Positive(v) => ser.serialize_u64(v),
            },
            Node::Float(v) => ser.serialize_f64(v.value()),
            Node::Date(date) => ser.serialize_str(&date.value().to_string()),
            Node::Invalid(_) => {
                // Invalid nodes are simply skipped from the serialization.
                Err(Error::custom("invalid node cannot be serialized"))
            }
        }
    }
}

#[derive(Default)]
struct TomlVisitor;

impl<'de> Visitor<'de> for TomlVisitor {
    type Value = Node;

    fn expectin
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `serde.visit_str` | function | No description |
| `serde.visit_map` | function | No description |
| `serde.visit_unit` | function | No description |
| `serde.visit_u64` | function | No description |
| `serde.visit_i64` | function | No description |
| `serde.expecting` | function | No description |
| `serde.visit_f64` | function | No description |
| `serde.visit_bytes` | function | No description |
| `serde.visit_some` | function | No description |
| `serde.visit_seq` | function | No description |
| `serde.visit_enum` | function | No description |
| `serde.visit_none` | function | No description |
| `serde.deserialize` | function | No description |
| `serde.serialize` | function | No description |
| `serde.visit_bool` | function | No description |
| `serde.visit_newtype_struct` | function | No description |
| `serde.TomlVisitor` | struct | No description |

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
