use lsp_types::{request::Request, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Serialize a TOML text to JSON.
pub enum TomlToJsonRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TomlToJsonParams {
    /// TOML text.
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TomlToJsonResponse {
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

/// Serialize a TOML text to JSON.
pub enum JsonToTomlRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonToTomlParams {
    /// JSON text.
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonToTomlResponse {
    /// TOML text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Request for JsonToTomlRequest {
    type Params = JsonToTomlParams;
    type Result = JsonToTomlResponse;
    const METHOD: &'static str = "taplo/jsonToToml";
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
    const METHOD: &'static str = "taplo/listSchemas";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaInfo {
    pub url: Url,
    pub meta: Value,
}

pub enum AssociatedSchemaRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssociatedSchemaParams {
    pub document_uri: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssociatedSchemaResponse {
    pub schema: Option<SchemaInfo>,
}

impl Request for AssociatedSchemaRequest {
    type Params = AssociatedSchemaParams;
    type Result = AssociatedSchemaResponse;
    const METHOD: &'static str = "taplo/associatedSchema";
}
