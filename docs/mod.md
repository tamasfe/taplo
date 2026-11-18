# Module: mod

**File:** `\crates\taplo-common\src\schema\mod.rs`

## Overview

Module `mod` contains 14 analyzed entities.

```
use self::{associations::SchemaAssociations, builtins::builtin_schema, cache::Cache};
use crate::{environment::Environment, util::ArcHashValue, LruCache};
use anyhow::{anyhow, Context};
use async_recursion::async_recursion;
use futures::{stream::FuturesUnordered, StreamExt};
use itertools::Itertools;
use json_value_merge::Merge;
use jsonschema::{error::ValidationErrorKind, JSONSchema, SchemaResolver, ValidationError};
use parking_lot::Mutex;
use regex::Regex;
use serde_json::Value;
use std::{borrow::Cow, num::NonZeroUsize, sync::Arc};
use taplo::{
    dom::{self, node::Key, KeyOrIndex, Keys},
    rowan::TextRange,
};
use thiserror::Error;
use tokio::sync::Semaphore;
use url::Url;

pub mod associations;
pub mod cache;
pub mod ext;

pub mod builtins {
    use serde_json::Value;
    use std::sync::Arc;
    use url::Url;

    pub const TAPLO_CONFIG_URL: &str = "taplo://taplo.toml";

    #[must_use]
    pub fn taplo_config_schema() -> Arc<Value> {
        Arc::new(serde_json::to_value(schemars::schema_for!(crate::config::Config)).unwrap())
    }

    #[must_use]
    pub fn builtin_schema(url: &Url) -> Option<Arc<Value>> {
        if url.as_str() == TAPLO_CONFIG_URL {
            Some(taplo_config_schema())
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Schemas<E: Environment> {
    env: E,
    associations: SchemaAssociations<E>,
    concurrent_requests: Arc<Semaphore>,
    http: reqwest::Client,
    validators: Arc<Mutex<LruCache<Url, Arc<JSONSchema>>>>,
    cache: Cache<E>,
}

impl<E: Environment> Schemas<E> {
    pub fn new(env: E, http: reqwest::Client) -> Self {
        let cache = Cache::new(env.clone());

        Self {
            associations: SchemaAssociations::new(env.clone(), cache.clone(), http.clone()),
            cache,
            env,
            concurrent_requests: Arc::new(Semaphore::new(10)),
            http,
            validators: Arc::new(Mutex::new(LruCache::with_hasher(
                NonZeroUsize::new(3).un
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `mod.comment` | function | No description |
| `mod.string_literal` | function | No description |
| `mod.string` | function | No description |
| `mod.join_ranges` | function | No description |
| `mod.strip_quotes` | function | No description |
| `mod.find_node_deep` | function | No description |
| `mod.find_node` | function | No description |
| `mod.find` | function | No description |
| `mod.multi_line_string` | function | No description |
| `mod.multi_line_string_literal` | function | No description |
| `mod.try_join_ranges` | function | No description |
| `mod.overlaps` | function | No description |
| `mod.StrExt` | trait | No description |
| `mod.SyntaxExt` | trait | No description |

## Git History

- Commits: 13
- Authors: 5
- Bug fixes: 4
- Features: 3

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
