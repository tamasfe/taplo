use super::node::{IntegerValue, Node};
use serde::{
    ser::{Error, SerializeMap, SerializeSeq},
    Serialize, Serializer,
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
                    map.serialize_entry(key.value(), entry)?;
                }

                map.end()
            }
            Node::Array(arr) => {
                let items = arr.inner.items.read();
                let mut seq = ser.serialize_seq(Some(items.len()))?;
                for item in &**items {
                    seq.serialize_element(item)?;
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
