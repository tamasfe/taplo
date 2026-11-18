# Module: node

**File:** `\crates\taplo\src\dom\node.rs`

## Overview

Module `node` contains 43 analyzed entities.

```
use std::iter::empty;

use crate::{
    private::Sealed,
    syntax::{SyntaxElement, SyntaxKind},
    util::shared::Shared,
};

mod nodes;
use either::Either;
pub use nodes::*;
use rowan::TextRange;

use super::{
    error::{Error, QueryError},
    index::Index,
    Comment, FromSyntax, KeyOrIndex, Keys,
};

pub trait DomNode: Sized + Sealed {
    fn syntax(&self) -> Option<&SyntaxElement>;
    fn errors(&self) -> &Shared<Vec<Error>>;
    fn validate_node(&self) -> Result<(), &Shared<Vec<Error>>>;
    fn is_valid_node(&self) -> bool {
        self.validate_node().is_ok()
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Table(Table),
    Array(Array),
    Bool(Bool),
    Str(Str),
    Integer(Integer),
    Float(Float),
    Date(DateTime),
    Invalid(Invalid),
}

impl Sealed for Node {}
impl DomNode for Node {
    fn syntax(&self) -> Option<&SyntaxElement> {
        match self {
            Node::Table(n) => n.syntax(),
            Node::Array(n) => n.syntax(),
            Node::Bool(n) => n.syntax(),
            Node::Str(n) => n.syntax(),
            Node::Integer(n) => n.syntax(),
            Node::Float(n) => n.syntax(),
            Node::Date(n) => n.syntax(),
            Node::Invalid(n) => n.syntax(),
        }
    }

    fn errors(&self) -> &Shared<Vec<Error>> {
        match self {
            Node::Table(n) => n.errors(),
            Node::Array(n) => n.errors(),
            Node::Bool(n) => n.errors(),
            Node::Str(n) => n.errors(),
            Node::Integer(n) => n.errors(),
            Node::Float(n) => n.errors(),
            Node::Date(n) => n.errors(),
            Node::Invalid(n) => n.errors(),
        }
    }

    fn validate_node(&self) -> Result<(), &Shared<Vec<Error>>> {
        match self {
            Node::Table(n) => n.validate_node(),
            Node::Array(n) => n.validate_node(),
            Node::Bool(n) => n.validate_node(),
            Node::Str(n) => n.validate_node(),
            Node::Integer(n) => n.validate_node(),
 
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `node.validate` | function | No description |
| `node.text_ranges` | function | No description |
| `node.is_bool` | function | No description |
| `node.as_bool` | function | No description |
| `node.as_date` | function | No description |
| `node.try_into_date` | function | No description |
| `node.from` | function | No description |
| `node.find_all_matches` | function | No description |
| `node.is_array` | function | No description |
| `node.as_array` | function | No description |
| `node.try_into_float` | function | No description |
| `node.path` | function | No description |
| `node.is_table` | function | No description |
| `node.as_table` | function | No description |
| `node.try_into_integer` | function | No description |
| `node.collect_flat` | function | No description |
| `node.try_get` | function | No description |
| `node.header_comments` | function | No description |
| `node.is_invalid` | function | No description |
| `node.try_into_str` | function | No description |

## Git History

- Commits: 8
- Authors: 6
- Bug fixes: 2
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
