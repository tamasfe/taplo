use super::cache::Cache;
use crate::{config::Config, environment::Environment, util::GlobRule, IndexMap};
use anyhow::anyhow;
use parking_lot::{RwLock, RwLockReadGuard};
use regex::Regex;
use semver::Version;
use serde::{de::Error, Deserialize, Serialize};
use serde_json::{json, Value};
use std::{borrow::Cow, path::Path, sync::Arc};
use tap::Tap;
use taplo::dom::Node;
use tokio::sync::Semaphore;
use url::Url;

pub const DEFAULT_CATALOGS: &[&str] = &[
    "https://taplo.tamasfe.dev/schema_index.json",
    "https://www.schemastore.org/api/json/catalog.json",
];

pub mod priority {
    pub const CATALOG: usize = 0;
    pub const CONFIG: usize = 50;
    pub const CONFIG_RULE: usize = 51;
    pub const LSP_CONFIG: usize = 60;
    pub const DIRECTIVE: usize = 70;
    pub const MAX: usize = usize::MAX;
}

pub mod source {
    pub const CATALOG: &str = "catalog";
    pub const CONFIG: &str = "config";
    pub const LSP_CONFIG: &str = "lsp_config";
    pub const MANUAL: &str = "manual";
    pub const DIRECTIVE: &str = "directive";
}

#[derive(Clone)]
pub struct SchemaAssociations<E: Environment> {
    concurrent_requests: Arc<Semaphore>,
    http: reqwest::Client,
    env: E,
    associations: Arc<RwLock<Vec<(AssociationRule, SchemaAssociation)>>>,
    cache: Cache<E>,
}

impl<E: Environment> SchemaAssociations<E> {
    pub(crate) fn new(env: E, cache: Cache<E>, http: reqwest::Client) -> Self {
        Self {
            concurrent_requests: Arc::new(Semaphore::new(10)),
            cache,
            env,
            http,
            associations: Default::default(),
        }
    }

    pub fn add(&self, rule: AssociationRule, assoc: SchemaAssociation) {
        self.associations.write().push((rule, assoc));
    }

    pub fn retain(&self, f: impl Fn(&(AssociationRule, SchemaAssociation)) -> bool) {
        self.associations.write().retain(f);
    }

    pub fn read(&self) -> RwLockReadGuard<'_, Vec<(AssociationRule, SchemaAssociation)>> {
        self.associations.read()
    }

    pub fn clear(&self) {
        self.associations.write().clear();
    }

    pub async fn add_from_catalog(&self, url: &Url) -> Result<(), anyhow::Error> {
        let index = self.load_catalog(url).await?;
        match index {
            SchemaCatalog::SchemaStore(index) => {
                for schema in &index.schemas {
                    match GlobRule::new(&schema.file_match, [] as [&str; 0]) {
                        Ok(rule) => {
                            self.associations.write().push((
                                rule.into(),
                                SchemaAssociation {
                                    url: schema.url.clone(),
                                    meta: json!({
                                        "name": schema.name,
                                        "description": schema.description,
                                        "source": source::CATALOG,
                                        "catalog_url": url,
                                    }),
                                    priority: priority::CATALOG,
                                },
                            ));
                        }
                        Err(error) => {
                            tracing::warn!(
                                %error,
                                schema_name = %schema.name,
                                source = %url,
                                "invalid glob pattern(s)"
                            );
                        }
                    }
                }
            }
            SchemaCatalog::Taplo(index) => {
                for schema in &index.schemas {
                    for pattern in &schema.extra.patterns {
                        let regex = match Regex::new(pattern) {
                            Ok(pat) => pat,
                            Err(error) => {
                                tracing::warn!(
                                    %error,
                                    pattern = %pattern,
                                    schema_name = %schema.title,
                                    "invalid regex pattern"
                                );
                                continue;
                            }
                        };

                        self.associations.write().push((
                            regex.into(),
                            SchemaAssociation {
                                url: schema.url.clone(),
                                meta: json!({
                                    "name": schema.title,
                                    "description": schema.description,
                                    "source": source::CATALOG,
                                    "catalog_url": url,
                                }),
                                priority: priority::CATALOG,
                            },
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn add_from_directive(&self, doc_url: &Url, root: &Node) {
        self.retain(|(rule, assoc)| match rule {
            AssociationRule::Url(u) => !(u == doc_url && assoc.meta["source"] == source::DIRECTIVE),
            _ => true,
        });

        for comment in root.header_comments() {
            if let Some("schema") = comment.directive() {
                let schema_url: Url = if comment.value().starts_with('.') {
                    match doc_url.join(comment.value()) {
                        Ok(s) => s,
                        Err(error) => {
                            tracing::error!(%error, "invalid schema directive");
                            continue;
                        }
                    }
                } else {
                    match comment.value().parse() {
                        Ok(s) => s,
                        Err(error) => {
                            tracing::error!(%error, "invalid schema directive");
                            continue;
                        }
                    }
                };

                self.associations.write().push((
                    AssociationRule::Url(doc_url.clone()),
                    SchemaAssociation {
                        url: schema_url,
                        priority: priority::DIRECTIVE,
                        meta: json!({ "source": source::DIRECTIVE }),
                    },
                ));
                break;
            }
        }
    }

    pub fn add_from_config(&self, config: &Config) {
        for rule in &config.rule {
            let file_rule = match rule.file_rule.clone() {
                Some(rule) => rule,
                None => continue,
            };

            if let Some(schema_opts) = &rule.options.schema {
                if let Some(url) = &schema_opts.url {
                    if schema_opts.enabled.unwrap_or(true) {
                        self.associations.write().push((
                            file_rule.into(),
                            SchemaAssociation {
                                url: url.clone(),
                                meta: json!({
                                    "source": source::CONFIG,
                                }),
                                priority: priority::CONFIG_RULE,
                            },
                        ));
                    }
                }
            }
        }

        let file_rule = match config.file_rule.clone() {
            Some(rule) => rule,
            None => return,
        };

        if let Some(schema_opts) = &config.global_options.schema {
            if let Some(url) = &schema_opts.url {
                if schema_opts.enabled.unwrap_or(true) {
                    self.associations.write().push((
                        file_rule.into(),
                        SchemaAssociation {
                            url: url.clone(),
                            meta: json!({
                                "source": source::CONFIG,
                            }),
                            priority: priority::CONFIG,
                        },
                    ));
                }
            }
        }
    }

    pub fn association_for(&self, file: &str) -> Option<SchemaAssociation> {
        self.associations
            .read()
            .iter()
            .filter_map(|(rule, assoc)| {
                if rule.is_match(file) {
                    Some(assoc.clone())
                } else {
                    None
                }
            })
            .max_by_key(|assoc| assoc.priority)
            .tap(|s| {
                if let Some(schema_association) = s {
                    tracing::debug!(
                        schema.url = %schema_association.url,
                        schema.name = schema_association.meta["name"].as_str().unwrap_or(""),
                        schema.source = schema_association.meta["source"].as_str().unwrap_or(""),
                        "found schema association"
                    );
                }
            })
    }

    async fn load_catalog(&self, index_url: &Url) -> Result<SchemaCatalog, anyhow::Error> {
        if let Ok(s) = self.cache.load(index_url, false).await {
            return Ok(serde_json::from_value((*s).clone())?);
        }

        let mut index = match self.fetch_external(index_url).await {
            Ok(idx) => idx,
            Err(error) => {
                tracing::warn!(?error, "failed to fetch catalog");
                if let Ok(s) = self.cache.load(index_url, true).await {
                    return Ok(serde_json::from_value((*s).clone())?);
                }
                return Err(error);
            }
        };

        if self.cache.is_cache_path_set() {
            if let Err(error) = self
                .cache
                .save(index_url.clone(), Arc::new(serde_json::to_value(&index)?))
                .await
            {
                tracing::warn!(%error, "failed to cache index");
            }
        }

        index.transform_paths();

        Ok(index)
    }

    async fn fetch_external(&self, index_url: &Url) -> Result<SchemaCatalog, anyhow::Error> {
        let _permit = self.concurrent_requests.acquire().await?;
        match index_url.scheme() {
            "http" | "https" => Ok(self
                .http
                .get(index_url.clone())
                .send()
                .await?
                .json()
                .await?),
            "file" => Ok(serde_json::from_slice(
                &self
                    .env
                    .read_file(
                        self.env
                            .to_file_path(index_url)
                            .ok_or_else(|| anyhow!("invalid file path"))?,
                    )
                    .await?,
            )?),
            scheme => Err(anyhow!("the scheme `{scheme}` is not supported")),
        }
    }
}

pub enum AssociationRule {
    Glob(GlobRule),
    Regex(Regex),
    Url(Url),
}

impl AssociationRule {
    pub fn glob(pattern: &str) -> Result<Self, anyhow::Error> {
        Ok(Self::Glob(GlobRule::new(&[pattern], &[] as &[&str])?))
    }

    pub fn regex(regex: &str) -> Result<Self, anyhow::Error> {
        Ok(Self::Regex(Regex::new(regex)?))
    }
}

impl From<Regex> for AssociationRule {
    fn from(v: Regex) -> Self {
        Self::Regex(v)
    }
}

impl From<GlobRule> for AssociationRule {
    fn from(v: GlobRule) -> Self {
        Self::Glob(v)
    }
}

impl AssociationRule {
    pub fn is_match(&self, text: &str) -> bool {
        match self {
            AssociationRule::Glob(g) => g.is_match(text),
            AssociationRule::Regex(r) => r.is_match(text),
            AssociationRule::Url(u) => u.as_str() == text,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaCatalog {
    SchemaStore(SchemaStoreCatalog),
    Taplo(TaploSchemaCatalog),
}

impl SchemaCatalog {
    fn transform_paths(&mut self) {
        // Common extensions that can be replaced with "toml".
        const COMMON_EXTENSIONS: &[&str] = &["yaml", "yml", "json"];

        if let SchemaCatalog::SchemaStore(index) = self {
            for s in &mut index.schemas {
                for fm in &mut s.file_match {
                    // Replace extensions with toml.
                    if Path::new(fm).extension().is_some() {
                        let ext = fm.rsplit('.').next().unwrap();
                        let ext_len = ext.len();
                        if COMMON_EXTENSIONS
                            .iter()
                            .any(|common_ext| ext.eq_ignore_ascii_case(common_ext))
                        {
                            fm.truncate(fm.len() - ext_len);
                            *fm += "toml";
                        }
                    }

                    if !fm.starts_with("**/") {
                        *fm = String::from("**/") + fm;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaploSchemaCatalog {
    pub schemas: Vec<TaploSchemaMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaploSchemaMeta {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    pub url: Url,
    pub url_hash: String,

    #[serde(flatten)]
    pub extra: TaploSchemaExtraInfo,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaploSchemaExtraInfo {
    pub authors: Vec<String>,
    pub version: Option<Version>,
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaStoreCatalog {
    #[serde(rename = "$schema")]
    pub schema: SchemaStoreCatalogSchema,
    pub schemas: Vec<SchemaStoreSchemaMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaStoreSchemaMeta {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub url: Url,
    #[serde(default)]
    pub file_match: Vec<String>,
    #[serde(default)]
    pub versions: IndexMap<String, Url>,
}

pub const SCHEMA_STORE_CATALOG_SCHEMA_URL: &str =
    "https://json.schemastore.org/schema-catalog.json";

#[derive(Debug, Clone, Copy)]
pub struct SchemaStoreCatalogSchema;

impl<'de> Deserialize<'de> for SchemaStoreCatalogSchema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = Cow::<'static, str>::deserialize(deserializer)?;

        if s != SCHEMA_STORE_CATALOG_SCHEMA_URL {
            return Err(Error::custom(format!(
                "expected $schema to be {}",
                SCHEMA_STORE_CATALOG_SCHEMA_URL
            )));
        }

        Ok(SchemaStoreCatalogSchema)
    }
}

impl Serialize for SchemaStoreCatalogSchema {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SCHEMA_STORE_CATALOG_SCHEMA_URL.serialize(serializer)
    }
}

#[derive(Debug, Clone)]
pub struct SchemaAssociation {
    pub meta: Value,
    pub url: Url,
    pub priority: usize,
}
