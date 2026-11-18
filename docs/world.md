# Module: world

**File:** `\crates\taplo-lsp\src\world.rs`

## Overview

Module `world` contains 14 analyzed entities.

```
use crate::{
    config::{InitConfig, LspConfig},
    lsp_ext::notification::{DidChangeSchemaAssociation, DidChangeSchemaAssociationParams},
};
use anyhow::anyhow;
use arc_swap::ArcSwap;
use lsp_async_stub::{rpc, util::Mapper, Context, RequestWriter};
use lsp_types::Url;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::json;
use std::{str, sync::Arc, time::Duration};
use taplo::{dom::Node, parser::Parse};
use taplo_common::{
    config::Config,
    environment::Environment,
    schema::{
        associations::{priority, source, AssociationRule, SchemaAssociation},
        Schemas,
    },
    AsyncRwLock, HashMap, IndexMap,
};

pub type World<E> = Arc<WorldState<E>>;

#[repr(transparent)]
pub struct Workspaces<E: Environment>(IndexMap<Url, WorkspaceState<E>>);

impl<E: Environment> std::ops::Deref for Workspaces<E> {
    type Target = IndexMap<Url, WorkspaceState<E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E: Environment> std::ops::DerefMut for Workspaces<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E: Environment> Workspaces<E> {
    #[must_use]
    pub fn by_document(&self, url: &Url) -> &WorkspaceState<E> {
        self.0
            .iter()
            .filter(|(key, _)| url.as_str().starts_with(key.as_str()))
            .max_by(|(a, _), (b, _)| a.as_str().len().cmp(&b.as_str().len()))
            .map_or_else(
                || {
                    tracing::warn!(document_url = %url, "using detached workspace");
                    self.0.get(&*DEFAULT_WORKSPACE_URL).unwrap()
                },
                |(_, ws)| ws,
            )
    }

    pub fn by_document_mut(&mut self, url: &Url) -> &mut WorkspaceState<E> {
        self.0
            .iter_mut()
            .filter(|(key, _)| {
                url.as_str().starts_with(key.as_str()) || *key == &*DEFAULT_WORKSPACE_URL
            })
            .max_by(|(a, _), (b, _)| a.as_str().len().cmp(&b.as_str().len()))
  
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `world.by_document` | function | No description |
| `world.new` | function | No description |
| `world.emit_associations` | function | No description |
| `world.deref` | function | No description |
| `world.deref_mut` | function | No description |
| `world.initialize` | function | No description |
| `world.by_document_mut` | function | No description |
| `world.set_default_config` | function | No description |
| `world.document` | function | No description |
| `world.load_config` | function | No description |
| `world.Workspaces` | struct | No description |
| `world.WorldState` | struct | No description |
| `world.WorkspaceState` | struct | No description |
| `world.DocumentState` | struct | No description |

## Git History

- Commits: 13
- Authors: 3
- Bug fixes: 4
- Features: 5

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
