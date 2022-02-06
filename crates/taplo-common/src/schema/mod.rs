use self::{associations::SchemaAssociations, cache::Cache};
use crate::{environment::Environment, HashMap};
use anyhow::anyhow;
use async_recursion::async_recursion;
use futures::{stream::FuturesUnordered, StreamExt};
use jsonschema::{error::ValidationErrorKind, JSONSchema, SchemaResolver, ValidationError};
use parking_lot::RwLock;
use regex::Regex;
use serde_json::Value;
use taplo::dom::{self, Keys, KeyOrIndex};
use std::{borrow::Cow, sync::Arc};
use thiserror::Error;
use tokio::sync::Semaphore;
use url::Url;

pub mod associations;
pub mod cache;
pub mod ext;

#[derive(Clone)]
pub struct Schemas<E: Environment> {
    env: E,
    associations: SchemaAssociations<E>,
    concurrent_requests: Arc<Semaphore>,
    http: reqwest::Client,
    validators: Arc<RwLock<HashMap<Url, Arc<JSONSchema>>>>,
    schemas: Arc<RwLock<HashMap<Url, Arc<Value>>>>,
    cache: Cache<E>,
}

impl<E: Environment> Schemas<E> {
    pub fn new(env: E, http: reqwest::Client) -> Self {
        let cache = Cache::new(env.clone());

        Self {
            associations: SchemaAssociations::new(env.clone(), cache.clone(), http.clone()),
            cache,
            env,
            concurrent_requests: Arc::new(Semaphore::new(10)),
            http,
            validators: Default::default(),
            schemas: Default::default(),
        }
    }

    /// Get a reference to the schemas's associations.
    pub fn associations(&self) -> &SchemaAssociations<E> {
        &self.associations
    }

    /// Get a reference to the schemas's cache.
    pub fn cache(&self) -> &Cache<E> {
        &self.cache
    }
}

impl<E: Environment> Schemas<E> {
    #[tracing::instrument(level = "debug", skip_all, fields(%schema_url))]
    pub async fn validate_root(
        &self,
        schema_url: &Url,
        root: &dom::Node,
    ) -> Result<Vec<NodeValidationError>, anyhow::Error> {
        let value = serde_json::to_value(&root)?;
        self.validate(schema_url, &value)
            .await?
            .into_iter()
            .map(|error| NodeValidationError::new(root, error))
            .collect::<Result<Vec<_>, _>>()
    }

    #[tracing::instrument(level = "debug", skip_all, fields(%schema_url))]
    pub async fn validate(
        &self,
        schema_url: &Url,
        value: &Value,
    ) -> Result<Vec<ValidationError<'static>>, anyhow::Error> {
        let validator = match self.get_validator(schema_url) {
            Some(s) => s,
            None => {
                let schema = self.load_schema(schema_url).await?;
                self.add_schema(schema_url, schema, true)?;
                self.get_validator(schema_url).unwrap()
            }
        };

        self.validate_impl(&validator, value).await
    }

    async fn validate_impl(
        &self,
        validator: &JSONSchema,
        value: &Value,
    ) -> Result<Vec<ValidationError<'static>>, anyhow::Error> {
        // The following loop is required for retrieving external schemas.
        //
        // We don't know if any external schemas are required until we reach
        // a validation path that requires it, so we might have to loop many times
        // to fully validate according to a schema that has many nested references.
        loop {
            match validator.validate(value) {
                Ok(_) => return Ok(Vec::new()),
                Err(errors) => {
                    let errors: Vec<_> = errors
                        .map(|err| ValidationError {
                            instance: Cow::Owned(err.instance.into_owned()),
                            kind: err.kind,
                            instance_path: err.instance_path,
                            schema_path: err.schema_path,
                        })
                        .collect();

                    // We check whether there were any external schema errors,
                    // and retrieve the schemas accordingly.
                    let mut external_schema_requests: FuturesUnordered<_> = errors
                        .iter()
                        .filter_map(|err| {
                            if let ValidationErrorKind::Resolver { url, .. } = &err.kind {
                                Some(async {
                                    let value = self.load_schema(url).await?;
                                    self.schemas.write().insert(url.clone(), Arc::new(value));
                                    Result::<(), anyhow::Error>::Ok(())
                                })
                            } else {
                                None
                            }
                        })
                        .collect();

                    // There are no external schemas to retrieve,
                    // return the errors as-is.
                    if external_schema_requests.is_empty() {
                        drop(external_schema_requests);
                        return Ok(errors);
                    }

                    // Retrieve external schemas, and return on the first failure.
                    while let Some(external_schema_result) = external_schema_requests.next().await {
                        if let Err(err) = external_schema_result {
                            return Err(err);
                        }
                    }

                    // Try validation again, now with external schemas
                    // resolved and cached.
                    continue;
                }
            };
        }
    }

    pub fn add_schema(
        &self,
        schema_url: &Url,
        schema: Value,
        validator: bool,
    ) -> Result<(), anyhow::Error> {
        if validator {
            let validator = self.create_validator(&schema)?;
            self.validators
                .write()
                .insert(schema_url.clone(), Arc::new(validator));
        }

        self.schemas
            .write()
            .insert(schema_url.clone(), Arc::new(schema));

        Ok(())
    }

    pub fn get_schema(&self, schema_url: &Url) -> Option<Arc<Value>> {
        self.schemas.read().get(schema_url).cloned()
    }

    pub fn get_validator(&self, schema_url: &Url) -> Option<Arc<JSONSchema>> {
        self.validators.read().get(schema_url).cloned()
    }

    #[async_recursion(?Send)]
    #[must_use]
    pub async fn resolve_schema(&self, url: Url) -> Result<Arc<Value>, anyhow::Error> {
        match url.fragment() {
            Some(fragment) => {
                let mut res_url = url.clone();
                res_url.set_fragment(None);
                let schema = self.resolve_schema(res_url).await?;
                let ptr = String::from("/") + fragment;
                schema
                    .pointer(&ptr)
                    .map(|v| Arc::new(v.clone()))
                    .ok_or_else(|| anyhow!("failed to resolve relative schema"))
            }
            None => {
                let val = self.load_schema(&url).await?;
                let val = Arc::new(val);
                self.schemas.write().insert(url, val.clone());
                Ok(val)
            }
        }
    }

    fn create_validator(&self, schema: &Value) -> Result<JSONSchema, anyhow::Error> {
        JSONSchema::options()
            .with_resolver(self.clone())
            .compile(schema)
            .map_err(|err| anyhow!("invalid schema: {err}"))
    }

    #[tracing::instrument(level = "debug", skip_all, fields(%schema_url))]
    async fn load_schema(&self, schema_url: &Url) -> Result<Value, anyhow::Error> {
        if let Ok(s) = self.cache.load(schema_url).await {
            tracing::debug!(%schema_url, "schema was found in cache");
            return Ok(s.value);
        }

        let schema = self.fetch_external(schema_url).await?;

        if self.cache.is_cache_path_set() {
            if let Err(error) = self.cache.store(schema_url.clone(), schema.clone()).await {
                tracing::warn!(%error, "failed to cache schema");
            }
        }

        Ok(schema)
    }

    async fn fetch_external(&self, schema_url: &Url) -> Result<Value, anyhow::Error> {
        let _permit = self.concurrent_requests.acquire().await?;
        match schema_url.scheme() {
            "http" | "https" => Ok(self
                .http
                .get(schema_url.clone())
                .send()
                .await?
                .json()
                .await?),
            "file" => Ok(serde_json::from_slice(
                &self
                    .env
                    .read_file(
                        self.env
                            .to_file_path(schema_url)
                            .ok_or_else(|| anyhow!("invalid file path"))?,
                    )
                    .await?,
            )?),
            scheme => Err(anyhow!("the scheme `{scheme}` is not supported")),
        }
    }
}

impl<E: Environment> Schemas<E> {
    #[tracing::instrument(level = "debug", skip_all, fields(%schema_url, ?path))]
    pub async fn schemas_at_path(
        &self,
        schema_url: &Url,
        schema: &Value,
        value: &Value,
        path: &Keys,
    ) -> Result<Vec<Arc<Value>>, anyhow::Error> {
        let mut schemas = Vec::new();
        self.collect_schemas(schema_url, schema, value, path, &mut schemas)
            .await?;
        Ok(schemas)
    }

    #[async_recursion(?Send)]
    #[must_use]
    async fn collect_schemas(
        &self,
        root_url: &Url,
        schema: &Value,
        value: &Value,
        path: &Keys,
        schemas: &mut Vec<Arc<Value>>,
    ) -> Result<(), anyhow::Error> {
        if !value.is_object() {
            return Ok(());
        }

        if let Some(r) = schema.schema_ref() {
            let url = reference_url(root_url, r)
                .ok_or_else(|| anyhow!("could not determine schema URL"))?;
            let schema = self.resolve_schema(url).await?;
            return self
                .collect_schemas(root_url, &schema, value, path, schemas)
                .await;
        }

        let validator = self.create_validator(schema)?;
        if !self.validate_impl(&validator, value).await?.is_empty() {
            return Ok(());
        }

        if let Some(one_ofs) = schema["oneOf"].as_array() {
            for one_of in one_ofs {
                self.collect_schemas(root_url, one_of, value, path, schemas)
                    .await?;
            }
        }

        if let Some(any_ofs) = schema["anyOf"].as_array() {
            for any_of in any_ofs {
                self.collect_schemas(root_url, any_of, value, path, schemas)
                    .await?;
            }
        }

        if let Some(all_ofs) = schema["allOf"].as_array() {
            for all_of in all_ofs {
                self.collect_schemas(root_url, all_of, value, path, schemas)
                    .await?;
            }
        }

        let key = match path.iter().next() {
            Some(k) => k,
            None => return Ok(()),
        };

        let child_path = path.skip_left(1);

        match key {
            KeyOrIndex::Key(k) => {
                self.collect_schemas(
                    root_url,
                    &schema["properties"][k.value()],
                    &value[k.value()],
                    &child_path,
                    schemas,
                )
                .await?;

                self.collect_schemas(
                    root_url,
                    &schema["additionalProperties"],
                    &value[k.value()],
                    &child_path,
                    schemas,
                )
                .await?;

                if let Some(pattern_props) = schema["patternProperties"].as_object() {
                    for (pattern, pattern_schema) in pattern_props {
                        if let Ok(re) = Regex::new(pattern) {
                            if re.is_match(k.value()) {
                                self.collect_schemas(
                                    root_url,
                                    pattern_schema,
                                    &value[k.value()],
                                    &child_path,
                                    schemas,
                                )
                                .await?;
                            }
                        }
                    }
                }
            }
            KeyOrIndex::Index(idx) => {
                if schema["items"].is_array() {
                    self.collect_schemas(
                        root_url,
                        &schema["items"][idx],
                        &value[idx],
                        &child_path,
                        schemas,
                    )
                    .await?;
                } else {
                    self.collect_schemas(
                        root_url,
                        &schema["items"],
                        &value[idx],
                        &child_path,
                        schemas,
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }
}

fn reference_url(root_url: &Url, reference: &str) -> Option<Url> {
    if !reference.starts_with('#') {
        return Url::parse(reference).ok();
    }
    let mut url = root_url.clone();
    url.set_fragment(Some(reference.trim_start_matches("#/")));
    Some(url)
}

trait ValueExt {
    fn is_schema_ref(&self) -> bool;
    fn schema_ref(&self) -> Option<&str>;
}

impl ValueExt for Value {
    fn is_schema_ref(&self) -> bool {
        self["$ref"].is_string()
    }

    fn schema_ref(&self) -> Option<&str> {
        self["$ref"].as_str()
    }
}

impl<E: Environment> SchemaResolver for Schemas<E> {
    fn resolve(
        &self,
        _root_schema: &serde_json::Value,
        url: &Url,
        _original_ref: &str,
    ) -> Result<Arc<serde_json::Value>, jsonschema::SchemaResolverError> {
        self.schemas
            .read()
            .get(url)
            .cloned()
            .ok_or_else(|| WouldBlockError.into())
    }
}

#[derive(Debug, Error)]
#[error("retrieving the schema requires external operations")]
struct WouldBlockError;

/// A validation error that contains text ranges as well.
#[derive(Debug)]
pub struct NodeValidationError {
    pub node: dom::Node,
    pub error: ValidationError<'static>,
}

impl NodeValidationError {
    fn new(root: &dom::Node, error: ValidationError<'static>) -> Result<Self, anyhow::Error> {
        let mut node = root.clone();

        for path in &error.instance_path {
            match path {
                jsonschema::paths::PathChunk::Property(p) => {
                    node = node.try_get(&**p).map_err(|_| anyhow!("invalid key"))?;
                }
                jsonschema::paths::PathChunk::Index(idx) => {
                    node = node.try_get(*idx).map_err(|_| anyhow!("invalid key"))?;
                }
                _ => {}
            }
        }

        Ok(Self { node, error })
    }
}
