//! Impls for serializing and deserializing Value with serde.

use crate::value::Value;
use indexmap::IndexMap;
use serde_crate::{
    de::{MapAccess, Visitor},
    ser::{Serialize, SerializeMap, SerializeSeq, Serializer},
    Deserialize,
};

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::UnsizedInteger(i) => serializer.serialize_u64(*i),
            Value::Integer(i) => serializer.serialize_i64(*i),
            Value::Float(f) => serializer.serialize_f64(*f),
            Value::String(s) => serializer.serialize_str(s),
            #[cfg(any(feature = "time", feature = "chrono"))]
            Value::Date(d) => match d {
                #[cfg(feature = "chrono")]
                crate::value::Date::OffsetDateTime(dt) => {
                    serializer.serialize_str(&dt.to_rfc3339())
                }
                #[cfg(feature = "time")]
                crate::value::Date::OffsetDateTime(dt) => {
                    serializer.serialize_str(&dt.format(time::Format::Rfc3339))
                }
                crate::value::Date::LocalDateTime(dt) => {
                    serializer.serialize_str(&dt.format("%Y-%m-%d %H:%M:%S").to_string())
                }
                crate::value::Date::LocalDate(date) => {
                    serializer.serialize_str(&date.format("%Y-%m-%d").to_string())
                }
                crate::value::Date::LocalTime(time) => {
                    serializer.serialize_str(&time.format("%H:%M:%S").to_string())
                }
            },
            Value::Array(arr) => {
                let mut seq = serializer.serialize_seq(Some(arr.len()))?;
                for item in arr {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            Value::Map(m) => {
                let mut map = serializer.serialize_map(Some(m.len()))?;

                for (key, value) in m {
                    map.serialize_entry(key, value)?;
                }

                map.end()
            }
        }
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "unsupported value")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut m = IndexMap::with_capacity(access.size_hint().unwrap_or(0));

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry::<String, Value>()? {
            m.insert(key, value);
        }

        Ok(Value::Map(m))
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde_crate::de::Error,
    {
        Ok(Value::Bool(v))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde_crate::de::Error,
    {
        Ok(Value::Integer(v.into()))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde_crate::de::Error,
    {
        Ok(Value::Integer(v.into()))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde_crate::de::Error,
    {
        Ok(Value::Integer(v.into()))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde_crate::de::Error,
    {
        Ok(Value::Integer(v))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde_crate::de::Error,
    {
        Ok(Value::UnsizedInteger(v.into()))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde_crate::de::Error,
    {
        Ok(Value::UnsizedInteger(v.into()))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde_crate::de::Error,
    {
        Ok(Value::UnsizedInteger(v.into()))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde_crate::de::Error,
    {
        Ok(Value::UnsizedInteger(v))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde_crate::de::Error,
    {
        Ok(Value::Float(v.into()))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde_crate::de::Error,
    {
        Ok(Value::Float(v))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde_crate::de::Error,
    {
        Ok(Value::String(v.to_string()))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde_crate::de::Error,
    {
        Ok(Value::String(v.to_string()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde_crate::de::Error,
    {
        Ok(Value::String(v))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde_crate::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde_crate::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde_crate::de::SeqAccess<'de>,
    {
        let mut arr = Vec::with_capacity(seq.size_hint().unwrap_or(0));

        while let Some(item) = seq.next_element()? {
            arr.push(item);
        }

        Ok(Value::Array(arr))
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde_crate::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}
