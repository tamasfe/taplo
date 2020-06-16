use crate::{
    dom::*,
    value::{Date, Value},
};

use verify::{
    span::{Span, Spanned},
    Validate, ValidateMap, ValidateSeq,
};

use rowan::TextRange;
use std::{convert::TryFrom, ops::AddAssign};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct NodeSpan(pub TextRange);

impl Span for NodeSpan {}

impl From<TextRange> for NodeSpan {
    fn from(r: TextRange) -> Self {
        Self(r)
    }
}

impl AddAssign for NodeSpan {
    fn add_assign(&mut self, rhs: Self) {
        // We don't need hierarchy, so just set the new span as the current one.
        *self = rhs
    }
}

macro_rules! impl_spanned {
    ($($ident:ident),*) => {
        $(impl Spanned for $ident {
            type Span = NodeSpan;

            fn span(&self) -> Option<Self::Span> {
                Some(self.text_range().into())
            }
        })*
    };
}

impl_spanned!(
    Node,
    EntryNode,
    KeyNode,
    ValueNode,
    IntegerNode,
    StringNode
);

// Don't highlight the entire document
impl Spanned for RootNode {
    type Span = NodeSpan;

    fn span(&self) -> Option<Self::Span> {
        Some(NodeSpan(TextRange::new(0.into(), 1.into())))
    }
}

// Only highlight the key instead of
// everything for table headers.
impl Spanned for TableNode {
    type Span = NodeSpan;

    fn span(&self) -> Option<Self::Span> {
        if self.is_inline() {
            Some(self.text_range().into())
        } else {
            Some(self.syntax().text_range().into())
        }
    }
}

// Only highlight the key instead of
// everything for table headers.
impl Spanned for ArrayNode {
    type Span = NodeSpan;

    fn span(&self) -> Option<Self::Span> {
        if self.is_array_of_tables() {
            Some(self.syntax().text_range().into())
        } else {
            Some(self.text_range().into())
        }
    }
}

impl Validate for Node {
    fn validate<V: verify::Validator<Self::Span>>(&self, validator: V) -> Result<(), V::Error> {
        match self {
            Node::Root(inner) => inner.validate(validator),
            Node::Table(inner) => inner.validate(validator),
            Node::Key(inner) => inner.validate(validator),
            Node::Value(inner) => inner.validate(validator),
            Node::Array(inner) => inner.validate(validator),
            Node::Entry(_) => unimplemented!("entry key and value must be validated separately"),
        }
    }
}

impl Validate for RootNode {
    fn validate<V: verify::Validator<Self::Span>>(&self, validator: V) -> Result<(), V::Error> {
        let mut map = validator.validate_map(Some(self.entries().len()))?;

        let mut errs: Option<V::Error> = None;

        for entry in self.entries().iter() {
            if let Err(err) = map.validate_string_entry(entry.key(), entry.value()) {
                match &mut errs {
                    Some(errs) => {
                        *errs += err;
                    }
                    None => {
                        errs = Some(err);
                    }
                }
            }
        }

        if let Err(err) = map.end() {
            match &mut errs {
                Some(errs) => {
                    *errs += err;
                }
                None => errs = Some(err),
            }
        }

        match errs {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

impl Validate for TableNode {
    fn validate<V: verify::Validator<Self::Span>>(&self, mut validator: V) -> Result<(), V::Error> {
        validator = validator.with_span(self.span());
        let mut map = validator.validate_map(Some(self.entries().len()))?;

        let mut errs: Option<V::Error> = None;

        for entry in self.entries().iter() {
            if let Err(err) = map.validate_string_entry(entry.key(), entry.value()) {
                match &mut errs {
                    Some(errs) => {
                        *errs += err;
                    }
                    None => {
                        errs = Some(err);
                    }
                }
            }
        }

        if let Err(err) = map.end() {
            match &mut errs {
                Some(errs) => {
                    *errs += err;
                }
                None => errs = Some(err),
            }
        }

        match errs {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

impl Validate for KeyNode {
    fn validate<V: verify::Validator<Self::Span>>(&self, validator: V) -> Result<(), V::Error> {
        // We assume that there are no dotted keys anymore at this point.
        validator.validate_str(self.keys_str().next().unwrap())
    }
}

impl Validate for ValueNode {
    fn validate<V: verify::Validator<Self::Span>>(&self, validator: V) -> Result<(), V::Error> {
        match self {
            ValueNode::Bool(v) => {
                validator.validate_bool(Value::try_from(v.clone()).unwrap().into_bool().unwrap())
            }
            ValueNode::String(v) => {
                validator.validate_str(&Value::try_from(v.clone()).unwrap().into_string().unwrap())
            }
            ValueNode::Integer(v) => {
                match Value::try_from(v.clone()).unwrap() {
                    // We try to use the smallest type,
                    // since some validators have size constraints,
                    // but we store everything as 64bits.
                    Value::UnsizedInteger(u) => {
                        if let Ok(v) = u8::try_from(u) {
                            validator.validate_u8(v)
                        } else if let Ok(v) = u16::try_from(u) {
                            validator.validate_u16(v)
                        } else if let Ok(v) = u32::try_from(u) {
                            validator.validate_u32(v)
                        } else {
                            validator.validate_u64(u)
                        }
                    }
                    Value::Integer(i) => {
                        if let Ok(v) = i8::try_from(i) {
                            validator.validate_i8(v)
                        } else if let Ok(v) = i16::try_from(i) {
                            validator.validate_i16(v)
                        } else if let Ok(v) = i32::try_from(i) {
                            validator.validate_i32(v)
                        } else {
                            validator.validate_i64(i)
                        }
                    }
                    _ => panic!("invalid value"),
                }
            }
            ValueNode::Float(v) => {
                validator.validate_f64(Value::try_from(v.clone()).unwrap().into_f64().unwrap())
            }
            ValueNode::Array(v) => v.validate(validator),
            ValueNode::Date(v) => {
                let date = Value::try_from(v.clone()).unwrap().into_date().unwrap();

                match date {
                    Date::OffsetDateTime(d) => validator.validate_str(&d.to_rfc3339()),
                    Date::LocalDateTime(d) => validator.validate_str(&d.to_string()),
                    Date::LocalDate(d) => validator.validate_str(&d.to_string()),
                    Date::LocalTime(d) => validator.validate_str(&d.to_string()),
                }
            }
            ValueNode::Table(v) => v.validate(validator),
            ValueNode::Empty => unimplemented!("empty node should not be used"),
        }
    }
}

impl Validate for ArrayNode {
    fn validate<V: verify::Validator<Self::Span>>(&self, mut validator: V) -> Result<(), V::Error> {
        validator = validator.with_span(self.span());
        
        let mut seq = validator.validate_seq(Some(self.items().len()))?;

        let mut errs: Option<V::Error> = None;

        for item in self.items() {
            if let Err(err) = seq.validate_element(item) {
                match &mut errs {
                    Some(errs) => {
                        *errs += err;
                    }
                    None => {
                        errs = Some(err);
                    }
                }
            }
        }

        if let Err(err) = seq.end() {
            match &mut errs {
                Some(errs) => {
                    *errs += err;
                }
                None => errs = Some(err),
            }
        }

        match errs {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}
