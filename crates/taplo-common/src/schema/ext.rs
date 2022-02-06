use serde::{Deserialize, Serialize};

pub const EXTENSION_KEY: &str = "x-taplo";

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ExtMeta {
    pub hidden: Option<bool>,
    pub links: Option<ExtLinks>,
    pub docs: Option<ExtDocs>,
    pub init_keys: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ExtDocs {
    pub main: Option<String>,
    pub const_value: Option<String>,
    pub default_value: Option<String>,
    pub enum_values: Option<Vec<Option<String>>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ExtLinks {
    pub key: Option<String>,
    pub enum_values: Option<Vec<Option<String>>>,
}
