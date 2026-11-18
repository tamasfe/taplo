# Module: request

**File:** `\crates\taplo-lsp\src\lsp_ext\request.rs`

## Overview

Module `request` contains 9 analyzed entities.

```
use lsp_types::{request::Request, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Serialize a TOML text to JSON.
pub enum ConvertToJsonRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertToJsonParams {
    /// TOML or JSON text.
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertToJsonResponse {
    /// JSON text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Request for ConvertToJsonRequest {
    type Params = ConvertToJsonParams;
    type Result = ConvertToJsonResponse;
    const METHOD: &'static str = "taplo/convertToJson";
}

/// Serialize a TOML text to JSON.
pub enum ConvertToTomlRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertToTomlParams {
    /// TOML or JSON text.
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertToTomlResponse {
    /// TOML text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Request for ConvertToTomlRequest {
    type Params = ConvertToTomlParams;
    type Result = ConvertToTomlResponse;
    const METHOD: &'static str = "taplo/convertToToml";
}

pub enum ListSchemasRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSchemasParams {
    pub document_uri: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSchemasResponse {
    pub schemas: Vec<SchemaInfo>,
}

impl Request for ListSchemasRequest {
    type Params = ListSchemasParams;
    type Result = ListSchemasResponse;
    
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `request.ListSchemasParams` | struct | No description |
| `request.ConvertToTomlResponse` | struct | No description |
| `request.SchemaInfo` | struct | No description |
| `request.AssociatedSchemaParams` | struct | No description |
| `request.AssociatedSchemaResponse` | struct | No description |
| `request.ConvertToJsonParams` | struct | No description |
| `request.ConvertToJsonResponse` | struct | No description |
| `request.ConvertToTomlParams` | struct | No description |
| `request.ListSchemasResponse` | struct | No description |

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
