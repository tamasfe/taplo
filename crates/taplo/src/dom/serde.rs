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

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a TOML value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BoolInner {
            errors: Default::default(),
            syntax: None,
            value: v.into(),
        }
        .wrap()
        .into())
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(IntegerInner {
            errors: Default::default(),
            syntax: None,
            repr: super::node::IntegerRepr::Dec,
            value: if v.is_negative() {
                IntegerValue::Negative(v)
            } else {
                IntegerValue::Positive(v as _)
            }
            .into(),
        }
        .wrap()
        .into())
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(IntegerInner {
            errors: Default::default(),
            syntax: None,
            repr: super::node::IntegerRepr::Dec,
            value: IntegerValue::Positive(v).into(),
        }
        .wrap()
        .into())
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(FloatInner {
            errors: Default::default(),
            syntax: None,
            value: v.into(),
        }
        .wrap()
        .into())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StrInner {
            errors: Default::default(),
            repr: super::node::StrRepr::Basic,
            syntax: None,
            value: v.to_string().into(),
        }
        .wrap()
        .into())
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let _ = v;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Bytes(v),
            &self,
        ))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &self,
        ))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Unit,
            &self,
        ))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

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

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        let _ = data;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Enum,
            &self,
        ))
    }
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        de.deserialize_any(TomlVisitor)
    }
}
