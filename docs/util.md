# Module: util

**File:** `\crates\taplo-common\src\util.rs`

## Overview

Module `util` contains 22 analyzed entities.

```
use globset::{Glob, GlobSetBuilder};
use percent_encoding::percent_decode_str;
use serde_json::Value;
use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct GlobRule {
    include: globset::GlobSet,
    exclude: globset::GlobSet,
}

impl GlobRule {
    pub fn new(
        include: impl IntoIterator<Item = impl AsRef<str>>,
        exclude: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Self, anyhow::Error> {
        let mut inc = GlobSetBuilder::new();
        for glob in include {
            inc.add(Glob::new(glob.as_ref())?);
        }

        let mut exc = GlobSetBuilder::new();
        for glob in exclude {
            exc.add(Glob::new(glob.as_ref())?);
        }

        Ok(Self {
            include: inc.build()?,
            exclude: exc.build()?,
        })
    }

    pub fn is_match(&self, text: impl AsRef<Path>) -> bool {
        if !self.include.is_match(text.as_ref()) {
            return false;
        }

        !self.exclude.is_match(text.as_ref())
    }
}

#[derive(Eq)]
pub struct ArcHashValue(pub Arc<Value>);

impl Hash for ArcHashValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        HashValue(&self.0).hash(state);
    }
}

impl PartialEq for ArcHashValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Eq)]
pub struct HashValue<'v>(pub &'v Value);

impl PartialEq for HashValue<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for HashValue<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.0 {
            Value::Null => 0.hash(state),
            Value::Bool(v) => v.hash(state),
            Value::Number(v) => v.hash(state),
            Value::String(v) => v.hash(state),
            Value::Array(v) => {
                for v in v {
                    HashValue(v).hash(state);
                }
            }
            Value::Object(v) => {
    
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `util.new` | function | No description |
| `util.text_range` | function | No description |
| `util.new_utf8` | function | No description |
| `util.new_utf16` | function | No description |
| `util.range` | function | No description |
| `util.all_range` | function | No description |
| `util.relative_position` | function | No description |
| `util.new_impl` | function | No description |
| `util.into_lsp` | function | No description |
| `util.from_lsp` | function | No description |
| `util.offset` | function | No description |
| `util.position` | function | No description |
| `util.mappings` | function | No description |
| `util.line_count` | function | No description |
| `util.relative_range` | function | No description |
| `util.test_mapper` | function | No description |
| `util.Position` | struct | No description |
| `util.Range` | struct | No description |
| `util.CharacterRange` | struct | No description |
| `util.Mapper` | struct | No description |

## Git History

- Commits: 11
- Authors: 4
- Bug fixes: 7
- Features: 4

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
