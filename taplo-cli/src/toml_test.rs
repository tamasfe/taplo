use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize,
};
use taplo::value::{Date, Value};

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TomlTestType {
    String,
    Integer,
    Float,
    Bool,
    DateTime,
    #[serde(rename = "datetime-local")]
    DateTimeLocal,
    #[serde(rename = "date-local")]
    DateLocal,
    #[serde(rename = "time-local")]
    TimeLocal,
}

impl TomlTestType {
    fn of(value: &Value) -> Option<Self> {
        match value {
            Value::Bool(_) => Some(TomlTestType::Bool),
            Value::UnsignedInteger(_) | Value::Integer(_) => Some(TomlTestType::Integer),
            Value::Float(_) => Some(TomlTestType::Float),
            Value::String(_) => Some(TomlTestType::String),
            Value::Date(d) => match d {
                Date::OffsetDateTime(_) => Some(TomlTestType::DateTime),
                Date::LocalDateTime(_) => Some(TomlTestType::DateTimeLocal),
                Date::LocalDate(_) => Some(TomlTestType::DateLocal),
                Date::LocalTime(_) => Some(TomlTestType::TimeLocal),
            },
            Value::Array(_) => None,
            Value::Map(_) => None,
        }
    }
}

pub struct TomlTestValue<'a> {
    r#type: Option<TomlTestType>,
    value: &'a Value,
}

impl<'a> TomlTestValue<'a> {
    pub fn new(value: &'a Value) -> Self {
        Self {
            r#type: TomlTestType::of(value),
            value,
        }
    }
}

impl<'a> Serialize for TomlTestValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(ty) = self.r#type {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("type", &ty)?;
            map.serialize_entry(
                "value",
                &serde_json::to_string(&self.value)
                    .map_err(serde::ser::Error::custom)?,
            )?;
            map.end()
        } else {
            match &self.value {
                Value::Array(values) => {
                    let mut seq = serializer.serialize_seq(Some(values.len()))?;
                    for value in values {
                        seq.serialize_element(&TomlTestValue::new(value))?;
                    }
                    seq.end()
                }
                Value::Map(object) => {
                    let mut map = serializer.serialize_map(Some(object.len()))?;
                    for (key, value) in object {
                        map.serialize_entry(key, &TomlTestValue::new(value))?;
                    }
                    map.end()
                }
                _ => unreachable!(),
            }
        }
    }
}
