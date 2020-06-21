/// Requests that are not in the LSP spec

use lsp_types::{Url, request::Request};
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

/// Show Syntax Tree
pub(crate) enum SyntaxTreeRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SyntaxTreeParams {
    /// URI of the document
    pub uri: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SyntaxTreeResponse {
    pub text: String,
}

impl Request for SyntaxTreeRequest {
    type Params = SyntaxTreeParams;
    type Result = SyntaxTreeResponse;
    const METHOD: &'static str = "taplo/syntaxTree";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DomTreeParams {
    /// URI of the document
    pub uri: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DomTreeResponse {
    pub text: String,
}

pub(crate) enum DomTreeRequest {}

impl Request for DomTreeRequest {
    type Params = DomTreeParams;
    type Result = DomTreeResponse;
    const METHOD: &'static str = "taplo/domTree";
}

/// Show Line mappings
pub(crate) enum LineMappingsRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LineMappingsParams {
    /// URI of the document
    pub uri: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LineMappingsResponse {
    pub lines: Vec<String>,
}

impl Request for LineMappingsRequest {
    type Params = LineMappingsParams;
    type Result = LineMappingsResponse;
    const METHOD: &'static str = "taplo/lineMappings";
}
