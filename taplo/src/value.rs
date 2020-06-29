//! This module is used to convert the DOM
//! nodes into the values they contain.

use crate::{dom, util::unescape};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use indexmap::IndexMap;
use std::convert::{TryFrom, TryInto};

/// This occurs when a key has an invalid escape
/// sequence.
#[derive(Debug)]
pub struct UnescapeError;

impl core::fmt::Display for UnescapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the key contains invalid escape sequence")
    }
}

impl std::error::Error for UnescapeError {}

pub type Map = IndexMap<String, Value>;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Date {
    OffsetDateTime(DateTime<FixedOffset>),
    LocalDateTime(NaiveDateTime),
    LocalDate(NaiveDate),
    LocalTime(NaiveTime),
}

/// Contains all possible value types in a TOML document.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    UnsizedInteger(u64),
    Integer(i64),
    Float(f64),
    Date(Date),
    String(String),
    Array(Vec<Value>),
    Map(Map),
}

impl Value {
    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            Value::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn into_bool(self) -> Option<bool> {
        match self {
            Value::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<&u64> {
        match self {
            Value::UnsizedInteger(v) => Some(v),
            _ => None,
        }
    }

    pub fn into_u64(self) -> Option<u64> {
        match self {
            Value::UnsizedInteger(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<&i64> {
        match self {
            Value::Integer(v) => Some(v),
            _ => None,
        }
    }

    pub fn into_i64(self) -> Option<i64> {
        match self {
            Value::Integer(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<&f64> {
        match self {
            Value::Float(v) => Some(v),
            _ => None,
        }
    }

    pub fn into_f64(self) -> Option<f64> {
        match self {
            Value::Float(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_date(&self) -> Option<&Date> {
        match self {
            Value::Date(v) => Some(v),
            _ => None,
        }
    }

    pub fn into_date(self) -> Option<Date> {
        match self {
            Value::Date(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Value::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn into_array(self) -> Option<Vec<Value>> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&Map> {
        match self {
            Value::Map(v) => Some(v),
            _ => None,
        }
    }

    pub fn into_map(self) -> Option<Map> {
        match self {
            Value::Map(v) => Some(v),
            _ => None,
        }
    }
}

impl TryFrom<dom::Node> for Value {
    type Error = Error;

    fn try_from(node: dom::Node) -> Result<Self, Self::Error> {
        match node {
            dom::Node::Root(v) => v.try_into(),
            dom::Node::Table(v) => v.try_into(),
            dom::Node::Value(v) => v.try_into(),
            dom::Node::Array(v) => v.try_into(),
            _ => Err(dom::Error::Spanned {
                range: node.text_range(),
                message: format!(
                    "cannot convert {:?} directly to value without context",
                    node.kind()
                ),
            }
            .into()),
        }
    }
}

impl TryFrom<dom::RootNode> for Value {
    type Error = Error;
    fn try_from(node: dom::RootNode) -> Result<Self, Self::Error> {
        Ok(Value::Map(
            node.into_entries()
                .into_iter()
                .try_fold::<_, _, Result<IndexMap<String, Value>, Self::Error>>(
                    IndexMap::new(),
                    |mut m, entry| {
                        m.insert(
                            unescape(&entry.key().full_key_string_stripped())
                                .map_err(|_| UnescapeError)?,
                            entry.into_value().try_into()?,
                        );
                        Ok(m)
                    },
                )?,
        ))
    }
}

impl TryFrom<dom::TableNode> for Value {
    type Error = Error;
    fn try_from(node: dom::TableNode) -> Result<Self, Self::Error> {
        Ok(Value::Map(
            node.into_entries()
                .into_iter()
                .try_fold::<_, _, Result<IndexMap<String, Value>, Self::Error>>(
                    IndexMap::new(),
                    |mut m, entry| {
                        m.insert(
                            unescape(&entry.key().full_key_string_stripped())
                                .map_err(|_| UnescapeError)?,
                            entry.into_value().try_into()?,
                        );
                        Ok(m)
                    },
                )?,
        ))
    }
}

impl TryFrom<dom::ArrayNode> for Value {
    type Error = Error;
    fn try_from(node: dom::ArrayNode) -> Result<Self, Self::Error> {
        Ok(Value::Array(
            node.into_items()
                .into_iter()
                .map(Value::try_from)
                .collect::<Result<Vec<Value>, Self::Error>>()?,
        ))
    }
}

impl TryFrom<dom::ValueNode> for Value {
    type Error = Error;
    fn try_from(node: dom::ValueNode) -> Result<Self, Self::Error> {
        Ok(match node {
            dom::ValueNode::Bool(v) => v.try_into()?,
            dom::ValueNode::String(v) => v.try_into()?,
            dom::ValueNode::Integer(v) => v.try_into()?,
            dom::ValueNode::Float(v) => v.try_into()?,
            dom::ValueNode::Array(v) => v.try_into()?,
            dom::ValueNode::Date(v) => v.try_into()?,
            dom::ValueNode::Table(v) => v.try_into()?,
            dom::ValueNode::Invalid(_) => unimplemented!(),
            _ => panic!("empty node"),
        })
    }
}

impl TryFrom<dom::BoolNode> for Value {
    type Error = Error;
    fn try_from(node: dom::BoolNode) -> Result<Self, Self::Error> {
        Ok(Value::Bool(node.to_string().parse()?))
    }
}

impl TryFrom<dom::StringNode> for Value {
    type Error = Error;
    fn try_from(node: dom::StringNode) -> Result<Self, Self::Error> {
        Ok(match node.string_kind() {
            dom::StringKind::Basic => Value::String(node.into_content()),
            dom::StringKind::MultiLine => Value::String(node.into_content()),
            dom::StringKind::Literal => Value::String(node.into_content()),
            dom::StringKind::MultiLineLiteral => Value::String(node.into_content()),
        })
    }
}

impl TryFrom<dom::IntegerNode> for Value {
    type Error = Error;
    fn try_from(node: dom::IntegerNode) -> Result<Self, Self::Error> {
        let node_str = node.to_string().replace("_", "");

        Ok(match node.repr() {
            dom::IntegerRepr::Dec => match i64::from_str_radix(&node_str, 10) {
                Ok(i) => Value::Integer(i),
                Err(_) => Value::UnsizedInteger(u64::from_str_radix(&node_str, 10)?),
            },

            dom::IntegerRepr::Bin => {
                match i64::from_str_radix(&node_str.trim_start_matches("0b"), 2) {
                    Ok(i) => Value::Integer(i),
                    Err(_) => Value::UnsizedInteger(u64::from_str_radix(
                        &node_str.trim_start_matches("0b"),
                        2,
                    )?),
                }
            }
            dom::IntegerRepr::Oct => {
                match i64::from_str_radix(&node_str.trim_start_matches("0o"), 8) {
                    Ok(i) => Value::Integer(i),
                    Err(_) => Value::UnsizedInteger(u64::from_str_radix(
                        &node_str.trim_start_matches("0o"),
                        8,
                    )?),
                }
            }
            dom::IntegerRepr::Hex => {
                match i64::from_str_radix(&node_str.trim_start_matches("0x"), 16) {
                    Ok(i) => Value::Integer(i),
                    Err(_) => Value::UnsizedInteger(u64::from_str_radix(
                        &node_str.trim_start_matches("0x"),
                        16,
                    )?),
                }
            }
        })
    }
}

impl TryFrom<dom::FloatNode> for Value {
    type Error = Error;
    fn try_from(node: dom::FloatNode) -> Result<Self, Self::Error> {
        Ok(Value::Float(
            node.to_string()
                .replace("_", "")
                .replace("nan", "NaN")
                .parse()?,
        ))
    }
}

impl TryFrom<dom::DateNode> for Value {
    type Error = Error;
    fn try_from(node: dom::DateNode) -> Result<Self, Self::Error> {
        let date_str = node.to_string().replace(" ", "T").replace("t", "T");

        if let Ok(d) = DateTime::parse_from_rfc3339(&date_str) {
            return Ok(Value::Date(Date::OffsetDateTime(d)));
        }

        if let Ok(d) = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S") {
            return Ok(Value::Date(Date::LocalDateTime(d)));
        }

        if let Ok(d) = NaiveTime::parse_from_str(&date_str, "%H:%M:%S") {
            return Ok(Value::Date(Date::LocalTime(d)));
        }

        if let Ok(d) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            return Ok(Value::Date(Date::LocalDate(d)));
        }

        Err(InvalidDateError(date_str).into())
    }
}

#[derive(Debug)]
pub struct InvalidDateError(String);

impl core::fmt::Display for InvalidDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid date format: \"{}\"", &self.0)
    }
}
impl std::error::Error for InvalidDateError {}

#[derive(Debug)]
pub struct Error(Box<dyn std::error::Error>);

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<E: std::error::Error + 'static> From<E> for Error {
    fn from(e: E) -> Self {
        Self(Box::new(e))
    }
}
