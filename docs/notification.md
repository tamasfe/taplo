# Module: notification

**File:** `\crates\taplo-lsp\src\lsp_ext\notification.rs`

## Overview

Module `notification` contains 3 analyzed entities.

```
use lsp_types::{notification::Notification, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub enum MessageWithOutput {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageKind {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageWithOutputParams {
    pub kind: MessageKind,
    pub message: String,
}

impl Notification for MessageWithOutput {
    type Params = MessageWithOutputParams;
    const METHOD: &'static str = "taplo/messageWithOutput";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssociationRule {
    Glob(String),
    Regex(String),
    Url(Url),
}

pub enum AssociateSchema {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssociateSchemaParams {
    pub document_uri: Option<Url>,
    pub schema_uri: Url,
    pub rule: AssociationRule,
    pub priority: Option<usize>,
    pub meta: Option<Value>,
}

impl Notification for AssociateSchema {
    type Params = AssociateSchemaParams;
    const METHOD: &'static str = "taplo/associateSchema";
}

pub enum DidChangeSchemaAssociation {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeSchemaAssociationParams {
    pub document_uri: Url,
    pub schema_uri: Option<Url>,
    pub meta: Option<Value>,
}

impl Notification for DidChangeSchemaAssociation {
    type Params = DidChangeSchemaAssociationParams;
    const METHOD: &'static str = "taplo/didChangeSchemaAssociation";
}
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `notification.MessageWithOutputParams` | struct | No description |
| `notification.DidChangeSchemaAssociationParams` | struct | No description |
| `notification.AssociateSchemaParams` | struct | No description |

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
