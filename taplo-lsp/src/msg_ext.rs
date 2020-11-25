//! Messages that are not part of the LSP spec.
use std::collections::HashMap;

use lsp_types::{notification::Notification, request::Request, Url};
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

pub(crate) enum MessageWithOutput {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum MessageKind {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MessageWithOutputParams {
    pub kind: MessageKind,
    pub message: String,
}

impl Notification for MessageWithOutput {
    type Params = MessageWithOutputParams;
    const METHOD: &'static str = "taplo/messageWithOutput";
}

pub(crate) enum UpdateBuiltInSchemas {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateBuiltInSchemasParams {
    pub associations: HashMap<String, String>,
}

impl Notification for UpdateBuiltInSchemas {
    type Params = UpdateBuiltInSchemasParams;
    const METHOD: &'static str = "taplo/updateBuiltinSchemas";
}

pub(crate) enum GetCachedSchemaRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetCachedSchemaParams {
    /// URI of the schema
    pub schema_uri: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetCachedSchemaResponse {
    pub schema_json: Option<String>,
}

impl Request for GetCachedSchemaRequest {
    type Params = GetCachedSchemaParams;
    type Result = GetCachedSchemaResponse;
    const METHOD: &'static str = "taplo/getCachedSchema";
}

pub(crate) enum CacheSchemaRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CacheSchemaParams {
    /// URI of the schema
    pub schema_uri: Url,
    pub schema_json: String,
}

impl Notification for CacheSchemaRequest {
    type Params = CacheSchemaParams;
    const METHOD: &'static str = "taplo/cacheSchema";
}

pub(crate) enum ConfigFileChanged {}

impl Notification for ConfigFileChanged {
    type Params = ();
    const METHOD: &'static str = "taplo/configFileChanged";
}


pub(crate) enum WatchConfigFile {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WatchConfigFileParams {
    pub config_path: String,
}


impl Notification for WatchConfigFile {
    type Params = WatchConfigFileParams;
    const METHOD: &'static str = "taplo/watchConfigFile";
}