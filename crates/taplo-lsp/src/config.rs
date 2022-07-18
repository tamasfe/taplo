use figment::{providers::Serialized, Figment};
use lsp_types::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use taplo_common::{
    config::Rule,
    schema::{associations::DEFAULT_CATALOGS, cache::DEFAULT_LRU_CACHE_EXPIRATION_TIME},
    HashMap,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitConfig {
    pub cache_path: Option<PathBuf>,
    #[serde(default = "default_configuration_section")]
    pub configuration_section: String,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            cache_path: Default::default(),
            configuration_section: default_configuration_section(),
        }
    }
}

fn default_configuration_section() -> String {
    String::from("evenBetterToml")
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LspConfig {
    pub taplo: TaploConfig,
    pub schema: SchemaConfig,
    pub completion: CompletionConfig,
    pub syntax: SyntaxConfig,
    pub formatter: taplo::formatter::OptionsIncompleteCamel,
    pub rules: Vec<Rule>,
}

impl LspConfig {
    pub fn update_from_json(&mut self, json: &Value) -> Result<(), anyhow::Error> {
        *self = Figment::new()
            .merge(Serialized::defaults(&self))
            .merge(Serialized::defaults(json))
            .extract()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionConfig {
    pub max_keys: usize,
}

impl Default for CompletionConfig {
    fn default() -> Self {
        Self { max_keys: 5 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyntaxConfig {
    pub semantic_tokens: bool,
}

impl Default for SyntaxConfig {
    fn default() -> Self {
        Self {
            semantic_tokens: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaConfig {
    pub enabled: bool,
    pub associations: HashMap<String, String>,
    pub catalogs: Vec<Url>,
    pub links: bool,
    pub cache: SchemaCacheConfig,
}

impl Default for SchemaConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            associations: Default::default(),
            catalogs: DEFAULT_CATALOGS
                .iter()
                .map(|c| c.parse().unwrap())
                .collect(),
            links: false,
            cache: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaCacheConfig {
    pub memory_expiration: u64,
    pub disk_expiration: u64,
}

impl Default for SchemaCacheConfig {
    fn default() -> Self {
        Self {
            memory_expiration: DEFAULT_LRU_CACHE_EXPIRATION_TIME.as_secs(),
            disk_expiration: DEFAULT_LRU_CACHE_EXPIRATION_TIME.as_secs(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaploConfig {
    pub config_file: TaploConfigFileConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaploConfigFileConfig {
    pub path: Option<PathBuf>,
    pub enabled: bool,
}

impl Default for TaploConfigFileConfig {
    fn default() -> Self {
        Self {
            path: Default::default(),
            enabled: true,
        }
    }
}
