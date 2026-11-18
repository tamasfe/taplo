# Module: config

**File:** `\crates\taplo-lsp\src\config.rs`

## Overview

Module `config` contains 1 analyzed entities.

```
use figment::{providers::Serialized, Figment};
use lsp_types::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use taplo_common::{
    config::Rule,
    schema::{associations::DEFAULT_CATALOGS, cache::DEFAULT_LRU_CACHE_EXPIRATION_TIME},
    HashMap,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitConfig {
    pub cache_path: Option<PathBuf>,
    #[serde(default = "default_configuration_section")]
    pub configuration_section: String,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            cache_path: Default::default(),
            configuration_section: default_configuration_section(),
        }
    }
}

fn default_configuration_section() -> String {
    String::from("evenBetterToml")
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LspConfig {
    pub taplo: TaploConfig,
    pub schema: SchemaConfig,
    pub completion: CompletionConfig,
    pub syntax: SyntaxConfig,
    pub formatter: taplo::formatter::OptionsIncompleteCamel,
    pub rules: Vec<Rule>,
}

impl LspConfig {
    pub fn update_from_json(&mut self, json: &Value) -> Result<(), anyhow::Error> {
        *self = Figment::new()
            .merge(Serialized::defaults(&self))
            .merge(Serialized::defaults(json))
            .extract()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionConfig {
    pub max_keys: usize,
}

impl Default for CompletionConfig {
    fn default() -> Self {
        Self { max_keys: 5 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyntaxConfig {
    pub semantic_tokens: bool,
}

impl Default for SyntaxConfig {
    fn default() -> Self {
        Self {
            semantic_tokens: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `config.execute_config` | function | No description |

## Git History

- Commits: 3
- Authors: 1
- Bug fixes: 1
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
