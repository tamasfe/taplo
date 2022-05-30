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
