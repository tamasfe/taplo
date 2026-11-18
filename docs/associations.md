# Module: associations

**File:** `\crates\taplo-common\src\schema\associations.rs`

## Overview

Module `associations` contains 27 analyzed entities.

```
use super::{builtins, cache::Cache};
use crate::{
    config::Config,
    environment::Environment,
    util::{normalize_str, GlobRule},
    IndexMap,
};
use anyhow::anyhow;
use parking_lot::{RwLock, RwLockReadGuard};
use regex::Regex;
use semver::Version;
use serde::{de::Error, Deserialize, Serialize};
use serde_json::{json, Value};
use std::{borrow::Cow, path::Path, sync::Arc};
use tap::Tap;
use taplo::dom::Node;
use tokio::sync::Semaphore;
use url::Url;

pub const DEFAULT_CATALOGS: &[&str] = &["https://www.schemastore.org/api/json/catalog.json"];

pub mod priority {
    pub const BUILTIN: usize = 10;
    pub const CATALOG: usize = 25;
    pub const CONFIG: usize = 50;
    pub const CONFIG_RULE: usize = 51;
    pub const LSP_CONFIG: usize = 60;
    pub const SCHEMA_FIELD: usize = 70;
    pub const DIRECTIVE: usize = 75;
    pub const MAX: usize = usize::MAX;
}

pub mod source {
    pub const BUILTIN: &str = "builtin";
    pub const CATALOG: &str = "catalog";
    pub const CONFIG: &str = "config";
    pub const LSP_CONFIG: &str = "lsp_config";
    pub const MANUAL: &str = "manual";
    pub const SCHEMA_FIELD: &str = "$schema";
    pub const DIRECTIVE: &str = "directive";
}

#[derive(Clone)]
pub struct SchemaAssociations<E: Environment> {
    concurrent_requests: Arc<Semaphore>,
    http: reqwest::Client,
    env: E,
    associations: Arc<RwLock<Vec<(AssociationRule, SchemaAssociation)>>>,
    cache: Cache<E>,
}

impl<E: Environment> SchemaAssociations<E> {
    pub(crate) fn new(env: E, cache: Cache<E>, http: reqwest::Client) -> Self {
        let this = Self {
            concurrent_requests: Arc::new(Semaphore::new(10)),
            cache,
            env,
            http,
            associations: Default::default(),
        };
        this.add_builtins();
        this
    }

    pub fn add(&self, rule: AssociationRule, assoc: SchemaAssociation) {
        self.associations.write().push((rule, assoc));
    }

    pub fn retain(&self, f: impl Fn(&(AssociationRule,
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `associations.new` | function | No description |
| `associations.load_catalog` | function | No description |
| `associations.add` | function | No description |
| `associations.retain` | function | No description |
| `associations.clear` | function | No description |
| `associations.add_builtins` | function | No description |
| `associations.add_from_document` | function | No description |
| `associations.add_from_config` | function | No description |
| `associations.association_for` | function | No description |
| `associations.glob` | function | No description |
| `associations.add_from_catalog` | function | No description |
| `associations.read` | function | No description |
| `associations.fetch_external` | function | No description |
| `associations.is_match` | function | No description |
| `associations.regex` | function | No description |
| `associations.from` | function | No description |
| `associations.transform_paths` | function | No description |
| `associations.serialize` | function | No description |
| `associations.deserialize` | function | No description |
| `associations.TaploSchemaExtraInfo` | struct | No description |

## Git History

- Commits: 16
- Authors: 5
- Bug fixes: 7
- Features: 4

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
