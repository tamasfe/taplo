# Module: error

**File:** `\crates\taplo\src\dom\error.rs`

## Overview

Module `error` contains 0 analyzed entities.

```
use super::node::Key;
use crate::syntax::SyntaxElement;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("the syntax was not expected here: {syntax:#?}")]
    UnexpectedSyntax { syntax: SyntaxElement },
    #[error("the string contains invalid escape sequence(s)")]
    InvalidEscapeSequence { string: SyntaxElement },
    #[error("conflicting keys")]
    ConflictingKeys { key: Key, other: Key },
    #[error("expected table")]
    ExpectedTable { not_table: Key, required_by: Key },
    #[error("expected array of tables")]
    ExpectedArrayOfTables {
        not_array_of_tables: Key,
        required_by: Key,
    },
    #[error("{0}")]
    Query(#[from] QueryError),
}

#[derive(Debug, Clone, Error)]
pub enum QueryError {
    #[error("the key or index was not found")]
    NotFound,
    #[error("invalid glob pattern: {0}")]
    InvalidGlob(#[from] globset::Error),
    #[error("the given key is invalid: {0}")]
    InvalidKey(crate::parser::Error),
}
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
