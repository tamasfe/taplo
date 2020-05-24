/// Requests that are not in the LSP spec

use lsp_types::request::Request;
use serde::{Deserialize, Serialize};

/// Serialize a TOML text to JSON.
pub(crate) enum TomlToJsonRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TomlToJsonParams {
    /// TOML text.
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TomlToJsonResponse {
    /// JSON text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// List of syntax or semantic errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

impl Request for TomlToJsonRequest {
    type Params = TomlToJsonParams;
    type Result = TomlToJsonResponse;
    const METHOD: &'static str = "taplo/tomlToJson";
}
