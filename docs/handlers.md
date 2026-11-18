# Module: handlers

**File:** `\crates\taplo-lsp\src\handlers.rs`

## Overview

Module `handlers` contains 0 analyzed entities.

```
mod initialize;
pub(crate) use initialize::*;

mod documents;
pub(crate) use documents::*;

mod semantic_tokens;
pub(crate) use semantic_tokens::*;

mod folding_ranges;
pub(crate) use folding_ranges::*;

mod document_symbols;
pub(crate) use document_symbols::*;

mod formatting;
pub(crate) use formatting::*;

mod hover;
pub(crate) use hover::*;

mod completion;
pub(crate) use completion::*;

mod schema;
pub(crate) use schema::*;

mod configuration;
pub(crate) use configuration::*;

mod workspaces;
pub(crate) use workspaces::*;

mod links;
pub(crate) use links::*;

mod rename;
pub(crate) use rename::*;

mod conversion;
pub(crate) use conversion::*;
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| _none_ | — | — |

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
