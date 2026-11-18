# Module: to_toml

**File:** `\crates\taplo\src\dom\to_toml.rs`

## Overview

Module `to_toml` contains 4 analyzed entities.

```
use crate::util::escape;

use super::{
    node::{ArrayKind, DomNode, IntegerRepr, IntegerValue, TableKind},
    Keys, Node,
};
use std::fmt::{Formatter, Write};

impl Node {
    pub fn to_toml(&self, inline: bool, prefer_single_quote: bool) -> String {
        let mut s = String::new();
        self.to_toml_fmt(&mut s, inline, prefer_single_quote)
            .unwrap();
        s
    }

    pub fn to_toml_fmt(
        &self,
        f: &mut impl Write,
        inline: bool,
        prefer_single_quote: bool,
    ) -> core::fmt::Result {
        self.to_toml_impl(f, Keys::empty(), inline, false, prefer_single_quote)
    }

    fn to_toml_impl(
        &self,
        f: &mut impl Write,
        parent_keys: Keys,
        inline: bool,
        no_header: bool,
        prefer_single_quote: bool,
    ) -> core::fmt::Result {
        if let Node::Bool(_) | Node::Str(_) | Node::Integer(_) | Node::Float(_) | Node::Date(_) =
            self
        {
            if !parent_keys.is_empty() {
                f.write_str(parent_keys.dotted())?;
                f.write_str(" = ")?;
            }

            // Use the original representation of primitives if available.
            if let Some(syntax) = self.syntax() {
                return write!(f, "{}", syntax);
            }
        }

        match self {
            Node::Table(table) => {
                if table.inner.kind == TableKind::Inline || inline {
                    if !parent_keys.is_empty() {
                        f.write_str(parent_keys.dotted())?;
                        f.write_str(" = ")?;
                    }

                    f.write_str("{ ")?;

                    let entries = table.entries().read();

                    let mut first = true;
                    for (key, node) in entries.iter() {
                        if !first {
                            f.write_str(", ")?;
                        }
                        node.to_toml_impl(f, key.clone().into(), true, false, prefer_sin
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `to_toml.to_toml` | function | No description |
| `to_toml.fmt` | function | No description |
| `to_toml.to_toml_fmt` | function | No description |
| `to_toml.to_toml_impl` | function | No description |

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 0
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
