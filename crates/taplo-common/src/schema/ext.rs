use serde::{Deserialize, Serialize};
use serde_json::Value;
use tap::TapFallible;

pub const EXTENSION_KEY: &str = "x-taplo";

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TaploSchemaExt {
    pub hidden: Option<bool>,
    pub links: Option<ExtLinks>,
    pub docs: Option<ExtDocs>,
    pub init_keys: Option<Vec<String>>,
    #[serde(default)]
    pub plugins: Vec<String>,
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

#[must_use]
pub fn schema_ext_of(schema: &Value) -> Option<TaploSchemaExt> {
    schema.get(EXTENSION_KEY).and_then(|val| {
        if val.is_object() {
            serde_json::from_value(val.clone())
                .tap_err(
                    |error| tracing::warn!(key = EXTENSION_KEY, %error, "invalid schema extension"),
                )
                .ok()
        } else {
            None
        }
    })
}
