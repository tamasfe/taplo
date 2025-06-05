use super::{builtins, cache::Cache};
use crate::{
    config::Config,
    environment::Environment,
    util::{normalize_str, GlobRule},
    IndexMap,
};
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

pub const DEFAULT_CATALOGS: &[&str] = &["https://json.schemastore.org/api/json/catalog.json"];

pub mod priority {
    pub const BUILTIN: usize = 10;
    pub const CATALOG: usize = 25;
    pub const CONFIG: usize = 50;
    pub const CONFIG_RULE: usize = 51;
    pub const LSP_CONFIG: usize = 60;
    pub const SCHEMA_FIELD: usize = 70;
    pub const DIRECTIVE: usize = 75;
    pub const MAX: usize = usize::MAX;
}

pub mod source {
    pub const BUILTIN: &str = "builtin";
    pub const CATALOG: &str = "catalog";
    pub const CONFIG: &str = "config";
    pub const LSP_CONFIG: &str = "lsp_config";
    pub const MANUAL: &str = "manual";
    pub const SCHEMA_FIELD: &str = "$schema";
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
        let this = Self {
            concurrent_requests: Arc::new(Semaphore::new(10)),
            cache,
            env,
            http,
            associations: Default::default(),
        };
        this.add_builtins();
        this
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

    /// Clear all associations.
    ///
    /// Note that this will completely remove all associations,
    /// even built-in ones that will have to be added again.
    pub fn clear(&self) {
        self.associations.write().clear();
    }

    pub fn add_builtins(&self) {
        self.retain(|(_, assoc)| assoc.meta["source"] != source::BUILTIN);

        self.associations.write().push((
            AssociationRule::Regex(Regex::new(r".*\.?taplo\.toml$").unwrap()),
            SchemaAssociation {
                url: builtins::TAPLO_CONFIG_URL.parse().unwrap(),
                meta: json!({
                    "name": "Taplo",
                    "description": "Taplo configuration file.",
                    "source": source::BUILTIN
                }),
                priority: priority::BUILTIN,
            },
        ));
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

    /// Adds the schema from either a directive, or a `$schema` key in the root.
    pub fn add_from_document(&self, doc_url: &Url, root: &Node) {
        self.retain(|(rule, assoc)| match rule {
            AssociationRule::Url(u) => {
                !(u == doc_url
                    && (assoc.meta["source"] == source::DIRECTIVE
                        || assoc.meta["source"] == source::SCHEMA_FIELD))
            }
            _ => true,
        });

        for comment in root.header_comments() {
            if let Some("schema") = comment.directive() {
                let value = comment.value();

                if value.is_empty() {
                    tracing::warn!("empty schema directive");
                    continue;
                }

                let schema_url: Url = match value.parse() {
                    Ok(url) => url,
                    Err(error) => {
                        tracing::debug!(%error, "invalid url in directive, assuming file path instead");

                        if self.env.is_absolute(Path::new(value)) {
                            match format!("file://{value}").parse() {
                                Ok(u) => u,
                                Err(error) => {
                                    tracing::error!(%error, "invalid schema directive");
                                    continue;
                                }
                            }
                        } else {
                            match doc_url.join(value) {
                                Ok(u) => u,
                                Err(error) => {
                                    tracing::error!(%error, "invalid schema directive");
                                    continue;
                                }
                            }
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

        if let Node::Str(s) = root.get("$schema") {
            let schema_url: Url = if s.value().starts_with('.') {
                match doc_url.join(s.value()) {
                    Ok(s) => s,
                    Err(error) => {
                        tracing::error!(%error, "invalid schema url or path given in the `$schema` field");
                        return;
                    }
                }
            } else {
                match s.value().parse() {
                    Ok(s) => s,
                    Err(error) => {
                        tracing::error!(%error, "invalid schema url or path given in the `$schema` field");
                        return;
                    }
                }
            };

            self.associations.write().push((
                AssociationRule::Url(doc_url.clone()),
                SchemaAssociation {
                    url: schema_url,
                    priority: priority::SCHEMA_FIELD,
                    meta: json!({ "source": source::SCHEMA_FIELD }),
                },
            ));
        }
    }

    pub fn add_from_config(&self, config: &Config) {
        for rule in &config.rule {
            let Some(file_rule) = rule.file_rule.clone() else {
                continue;
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

        let Some(file_rule) = config.file_rule.clone() else {
            return;
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

    pub fn association_for(&self, file: &Url) -> Option<SchemaAssociation> {
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

        index.transform_paths();

        if self.cache.is_cache_path_set() {
            if let Err(error) = self
                .cache
                .save(index_url.clone(), Arc::new(serde_json::to_value(&index)?))
                .await
            {
                tracing::warn!(%error, "failed to cache index");
            }
        }

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
                            .to_file_path_normalized(index_url)
                            .ok_or_else(|| anyhow!("invalid file path"))?
                            .as_ref(),
                    )
                    .await?,
            )?),
            scheme => Err(anyhow!("the scheme `{scheme}` is not supported")),
        }
    }
}

#[derive(Clone)]
pub enum AssociationRule {
    Glob(GlobRule),
    Regex(Regex),
    Url(Url),
}

impl AssociationRule {
    pub fn glob(pattern: &str) -> Result<Self, anyhow::Error> {
        Ok(Self::Glob(GlobRule::new([pattern], &[] as &[&str])?))
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
    #[must_use]
    pub fn is_match(&self, url: &Url) -> bool {
        match self {
            // Glob associations typically come from config files
            // with a glob pattern that is an absolute file path
            // without a scheme.
            //
            // So in order to be a match, we need to
            // strip the scheme from the URL.
            AssociationRule::Glob(g) => g.is_match(&*normalize_str(
                url.as_str()
                    .strip_prefix(url.scheme())
                    .unwrap()
                    .strip_prefix("://")
                    .unwrap(),
            )),
            AssociationRule::Regex(r) => r.is_match(&normalize_str(url.as_str())),
            AssociationRule::Url(u) => u == url,
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
        if let SchemaCatalog::SchemaStore(index) = self {
            for s in &mut index.schemas {
                for fm in &mut s.file_match {
                    if !fm.starts_with("**/") {
                        *fm = String::from("**/") + fm.as_str();
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
                "expected $schema to be {SCHEMA_STORE_CATALOG_SCHEMA_URL}"
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
