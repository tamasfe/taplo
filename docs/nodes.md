# Module: nodes

**File:** `\crates\taplo\src\dom\node\nodes.rs`

## Overview

Module `nodes` contains 30 analyzed entities.

```
use super::{DomNode, Node};
use crate::{
    dom::{error::Error, Entries, KeyOrIndex, Keys},
    syntax::{SyntaxElement, SyntaxKind},
    util::{shared::Shared, unescape},
};
use logos::Lexer;
use once_cell::unsync::OnceCell;
use rowan::{NodeOrToken, TextRange};
use std::{fmt::Write, iter::once, sync::Arc};
use time::macros::format_description;

macro_rules! wrap_node {
    (
    $(#[$attrs:meta])*
    $vis:vis struct $name:ident {
        inner: $inner:ident
    }
    ) => {
        $(#[$attrs])*
        $vis struct $name {
            pub(crate) inner: Arc<$inner>,
        }

        impl $crate::private::Sealed for $name {}
        impl $crate::dom::node::DomNode for $name {
            fn syntax(&self) -> Option<&$crate::syntax::SyntaxElement> {
                self.inner.syntax.as_ref()
            }

            fn errors(&self) -> &$crate::util::shared::Shared<Vec<$crate::dom::error::Error>> {
                &self.inner.errors
            }

            fn validate_node(&self) -> Result<(), &$crate::util::shared::Shared<Vec<$crate::dom::error::Error>>> {
                self.validate_impl()
            }
        }

        impl $inner {
            #[allow(dead_code)]
            pub(crate) fn wrap(self) -> $name {
                self.into()
            }
        }

        impl From<$inner> for $name {
            fn from(inner: $inner) -> $name {
                $name {
                    inner: Arc::new(inner)
                }
            }
        }
    };
}

#[derive(Debug)]
pub(crate) struct TableInner {
    pub(crate) errors: Shared<Vec<Error>>,
    pub(crate) syntax: Option<SyntaxElement>,
    pub(crate) header: bool,
    pub(crate) kind: TableKind,
    pub(crate) entries: Shared<Entries>,
}

wrap_node! {
    #[derive(Debug, Clone)]
    pub struct Table { inner: TableInner }
}

impl Table {
    pub fn get(&self, key: impl Into<Key>) -> Option<Node> {
        let key = key.into();
        let entries = self.inner.entries.read();
        entries.loo
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `nodes.get` | function | No description |
| `nodes.is_inline` | function | No description |
| `nodes.is_negative` | function | No description |
| `nodes.validate_impl` | function | No description |
| `nodes.text_ranges` | function | No description |
| `nodes.is_tables` | function | No description |
| `nodes.fmt` | function | No description |
| `nodes.entries` | function | No description |
| `nodes.kind` | function | No description |
| `nodes.add_entry` | function | No description |
| `nodes.join` | function | No description |
| `nodes.items` | function | No description |
| `nodes.value` | function | No description |
| `nodes.from` | function | No description |
| `nodes.as_ref` | function | No description |
| `nodes.eq` | function | No description |
| `nodes.hash` | function | No description |
| `nodes.as_positive` | function | No description |
| `nodes.as_negative` | function | No description |
| `nodes.new` | function | No description |

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
