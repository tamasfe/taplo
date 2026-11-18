# Module: rpc

**File:** `\crates\lsp-async-stub\src\rpc.rs`

## Overview

Module `rpc` contains 31 analyzed entities.

```
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub type RequestId = lsp_types::NumberOrString;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub jsonrpc: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RequestId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

impl Message {
    pub fn is_notification(&self) -> bool {
        self.method.is_some() && self.id.is_none()
    }

    pub fn is_response(&self) -> bool {
        self.method.is_none()
    }

    pub fn into_request(self) -> Request<serde_json::Value> {
        Request {
            jsonrpc: self.jsonrpc,
            method: self.method.unwrap(),
            id: self.id,
            params: self.params,
        }
    }

    pub fn into_response(self) -> Response<serde_json::Value> {
        Response {
            jsonrpc: self.jsonrpc,
            id: self.id.unwrap(),
            error: self.error,
            result: self.result,
        }
    }
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<T = ()> {
    pub jsonrpc: String,
    pub method: String,

    pub id: Option<RequestId>,
    pub params: Option<T>,
}

impl<T: Serialize + DeserializeOwned> Request<T> {
    pub fn new() -> Self {
        Self {
            jsonrpc: "2.0".into(),
            method: "".into(),
            id: None,
            params: None,
        }
    }

    pub fn with_method(self, method: &str) -> Self {
        Self {
            method: method.into(),
            ..self
        }
    }

    pub fn with_id(se
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `rpc.into_params` | function | No description |
| `rpc.invalid_request` | function | No description |
| `rpc.from` | function | No description |
| `rpc.fmt` | function | No description |
| `rpc.into_response` | function | No description |
| `rpc.parse` | function | No description |
| `rpc.request_cancelled` | function | No description |
| `rpc.is_notification` | function | No description |
| `rpc.with_id` | function | No description |
| `rpc.error` | function | No description |
| `rpc.with_data` | function | No description |
| `rpc.with_method` | function | No description |
| `rpc.with_code` | function | No description |
| `rpc.new` | function | No description |
| `rpc.is_response` | function | No description |
| `rpc.into_message` | function | No description |
| `rpc.server_not_initialized` | function | No description |
| `rpc.with_params` | function | No description |
| `rpc.into_result` | function | No description |
| `rpc.internal_error` | function | No description |

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
