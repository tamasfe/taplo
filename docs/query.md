# Module: query

**File:** `\crates\taplo-lsp\src\query.rs`

## Overview

Module `query` contains 23 analyzed entities.

```
//! Cursor queries of a TOML document.

use taplo::{
    dom::{
        node::{DomNode, Key},
        FromSyntax, KeyOrIndex, Keys, Node,
    },
    rowan::{Direction, TextRange, TextSize},
    syntax::{SyntaxKind::*, SyntaxNode, SyntaxToken},
    util::join_ranges,
};

#[derive(Debug, Default)]
pub struct Query {
    /// The offset the query was made for.
    pub offset: TextSize,
    /// Before the cursor.
    pub before: Option<PositionInfo>,
    /// After the cursor.
    pub after: Option<PositionInfo>,
}

impl Query {
    /// Query a DOM root with the given cursor offset.
    /// Returns [`None`] if the position is out of range.
    ///
    /// # Panics
    ///
    /// Panics if the DOM was not entirely constructed from a syntax tree (e.g. if a node has no associated syntax element).
    /// Also panics if the given DOM node is not root.
    ///
    /// Also the given offset must be within the tree.
    #[must_use]
    pub fn at(root: &Node, offset: TextSize) -> Self {
        let syntax = root.syntax().cloned().unwrap().into_node().unwrap();

        Query {
            offset,
            before: offset
                .checked_sub(TextSize::from(1))
                .and_then(|offset| Self::position_info_at(root, &syntax, offset)),
            after: if offset >= syntax.text_range().end() {
                None
            } else {
                Self::position_info_at(root, &syntax, offset)
            },
        }
    }

    fn position_info_at(
        root: &Node,
        syntax: &SyntaxNode,
        offset: TextSize,
    ) -> Option<PositionInfo> {
        let syntax = match syntax.token_at_offset(offset) {
            taplo::rowan::TokenAtOffset::None => return None,
            taplo::rowan::TokenAtOffset::Single(s) => s,
            taplo::rowan::TokenAtOffset::Between(_, right) => right,
        };

        Some(PositionInfo {
            syntax,
            dom_node: root
                .flat_iter()
                .filter(|(k, n)| full_range(k, n
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `query.entry_value` | function | No description |
| `query.in_entry_keys` | function | No description |
| `query.entry_keys` | function | No description |
| `query.entry_key` | function | No description |
| `query.empty_line` | function | No description |
| `query.in_array` | function | No description |
| `query.header_key` | function | No description |
| `query.in_inline_table` | function | No description |
| `query.in_table_array_header` | function | No description |
| `query.is_inline` | function | No description |
| `query.is_single_quote_value` | function | No description |
| `query.position_info_at` | function | No description |
| `query.full_range` | function | No description |
| `query.at` | function | No description |
| `query.in_entry_value` | function | No description |
| `query.lookup_keys` | function | No description |
| `query.parent_table_or_array_table` | function | No description |
| `query.entry_has_eq` | function | No description |
| `query.dom_node` | function | No description |
| `query.in_table_header` | function | No description |

## Git History

- Commits: 6
- Authors: 3
- Bug fixes: 1
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
