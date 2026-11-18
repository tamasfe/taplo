# Module: from_syntax

**File:** `\crates\taplo\src\dom\from_syntax.rs`

## Overview

Module `from_syntax` contains 8 analyzed entities.

```
use super::{
    error::Error,
    node::{
        Array, ArrayInner, ArrayKind, Bool, BoolInner, DateTime, DateTimeInner, DomNode, Float,
        FloatInner, Integer, IntegerInner, IntegerRepr, Invalid, InvalidInner, Key, KeyInner, Node,
        Str, StrInner, StrRepr, Table, TableInner, TableKind,
    },
    Comment, Keys,
};
use crate::{
    private::Sealed,
    syntax::{SyntaxElement, SyntaxKind::*},
    util::{iter::ExactIterExt, shared::Shared},
};
use either::Either;

pub trait FromSyntax: Sized + Sealed {
    fn from_syntax(syntax: SyntaxElement) -> Self;
}

impl FromSyntax for Node {
    fn from_syntax(syntax: SyntaxElement) -> Self {
        match syntax.kind() {
            VALUE => {
                if let Some(child) = syntax.as_node().and_then(|n| n.first_child_or_token()) {
                    Node::from_syntax(child)
                } else {
                    Invalid::from_syntax(syntax).into()
                }
            }
            STRING | MULTI_LINE_STRING | STRING_LITERAL | MULTI_LINE_STRING_LITERAL => {
                Str::from_syntax(syntax).into()
            }
            INTEGER | INTEGER_HEX | INTEGER_OCT | INTEGER_BIN => {
                Integer::from_syntax(syntax).into()
            }
            FLOAT => Float::from_syntax(syntax).into(),
            BOOL => Bool::from_syntax(syntax).into(),
            DATE_TIME_OFFSET | DATE_TIME_LOCAL | DATE | TIME => {
                DateTime::from_syntax(syntax).into()
            }
            ARRAY => Array::from_syntax(syntax).into(),
            INLINE_TABLE | ROOT => Table::from_syntax(syntax).into(),
            _ => Invalid::from_syntax(syntax).into(),
        }
    }
}

impl FromSyntax for Table {
    fn from_syntax(syntax: SyntaxElement) -> Self {
        let mut errors = Vec::new();
        match syntax.kind() {
            ROOT => root_from_syntax(syntax),
            TABLE_HEADER | TABLE_ARRAY_HEADER => TableInner {
                errors: errors.into(),
                syntax
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `from_syntax.keys_from_syntax` | function | No description |
| `from_syntax.from_syntax` | function | No description |
| `from_syntax.pseudo` | function | No description |
| `from_syntax.from_syntax_invalid` | function | No description |
| `from_syntax.root_from_syntax` | function | No description |
| `from_syntax.merge_intermediate` | function | No description |
| `from_syntax.entry_from_syntax` | function | No description |
| `from_syntax.FromSyntax` | trait | No description |

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
