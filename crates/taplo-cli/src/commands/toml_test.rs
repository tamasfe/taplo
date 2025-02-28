use crate::Taplo;
use anyhow::anyhow;
use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize,
};
use taplo::dom::{
    node::{DateTimeValue, DomNode},
    Node,
};
use taplo_common::environment::Environment;
use tokio::io::AsyncReadExt;

impl<E: Environment> Taplo<E> {
    pub async fn execute_toml_test(&self) -> Result<(), anyhow::Error> {
        let mut buf = String::new();
        self.env.stdin().read_to_string(&mut buf).await?;

        let parse = taplo::parser::parse(&buf);

        if !parse.errors.is_empty() {
            for err in parse.errors {
                eprintln!("{err}");
            }

            return Err(anyhow!("invalid toml"));
        }
        let dom = parse.into_dom();

        if let Err(err) = dom.validate() {
            for err in err {
                eprintln!("{err}");
            }
            return Err(anyhow!("invalid toml"));
        }

        serde_json::to_writer(std::io::stdout(), &TomlTestValue::new(&dom))?;

        Ok(())
    }
}

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
    fn of(node: &Node) -> Option<Self> {
        match node {
            Node::Bool(_) => Some(TomlTestType::Bool),
            Node::Integer(_) => Some(TomlTestType::Integer),
            Node::Float(_) => Some(TomlTestType::Float),
            Node::Str(_) => Some(TomlTestType::String),
            Node::Date(d) => match d.value() {
                DateTimeValue::OffsetDateTime(_) => Some(TomlTestType::DateTime),
                DateTimeValue::LocalDateTime(_) => Some(TomlTestType::DateTimeLocal),
                DateTimeValue::Date(_) => Some(TomlTestType::DateLocal),
                DateTimeValue::Time(_) => Some(TomlTestType::TimeLocal),
            },
            Node::Array(_) => None,
            Node::Table(_) => None,
            Node::Invalid(_) => unreachable!(),
        }
    }
}

pub struct TomlTestValue<'a> {
    r#type: Option<TomlTestType>,
    node: &'a Node,
}

impl<'a> TomlTestValue<'a> {
    pub fn new(node: &'a Node) -> Self {
        Self {
            r#type: TomlTestType::of(node),
            node,
        }
    }
}

impl Serialize for TomlTestValue<'_> {
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
}
