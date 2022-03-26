use self::{associations::SchemaAssociations, cache::Cache, ext::schema_ext_of, plugin::Plugin};
use crate::{environment::Environment, plugin, util::ArcHashValue, HashMap, LruCache};
use anyhow::{anyhow, Context};
use arc_swap::ArcSwap;
use async_recursion::async_recursion;
use futures::{stream::FuturesUnordered, StreamExt};
use itertools::Itertools;
use jsonschema::{error::ValidationErrorKind, JSONSchema, SchemaResolver, ValidationError};
use parking_lot::Mutex;
use regex::Regex;
use serde_json::Value;
use std::{borrow::Cow, sync::Arc};
use taplo::dom::{self, node::Key, KeyOrIndex, Keys};
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
    validators: Arc<Mutex<LruCache<Url, Arc<JSONSchema>>>>,
    cache: Cache<E>,
    #[allow(clippy::type_complexity)]
    plugins: Arc<ArcSwap<HashMap<Cow<'static, str>, Arc<dyn Plugin<E>>>>>,
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
            validators: Arc::new(Mutex::new(LruCache::new(3))),
            plugins: Default::default(),
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

    pub fn env(&self) -> &E {
        &self.env
    }

    pub fn set_plugins(&self, plugins: &[Arc<dyn Plugin<E>>]) {
        self.plugins.store(Arc::new(
            plugins
                .iter()
                .map(|v| (v.name().clone(), v.clone()))
                .collect::<HashMap<_, _>>(),
        ));
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
                let schema = self
                    .load_schema(schema_url)
                    .await
                    .with_context(|| format!("failed to load schema {schema_url}"))?;
                self.add_schema(schema_url, schema.clone()).await;
                self.add_validator(schema_url.clone(), &schema)
                    .with_context(|| format!("invalid schema {schema_url}"))?
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
                                    drop(self.cache.store(url.clone(), value));
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

    pub async fn add_schema(&self, schema_url: &Url, schema: Arc<Value>) {
        drop(self.cache.store(schema_url.clone(), schema).await);
    }

    #[tracing::instrument(level = "debug", skip_all, fields(%schema_url))]
    pub async fn load_schema(&self, schema_url: &Url) -> Result<Arc<Value>, anyhow::Error> {
        if let Ok(s) = self.cache.load(schema_url, false).await {
            tracing::debug!(%schema_url, "schema was found in cache");
            return Ok(s);
        }

        let schema = match self.fetch_external(schema_url).await {
            Ok(s) => Arc::new(s),
            Err(error) => {
                tracing::warn!(?error, "failed to fetch remote schema");
                if let Ok(s) = self.cache.load(schema_url, true).await {
                    tracing::debug!(%schema_url, "expired schema was found in cache");
                    return Ok(s);
                }
                return Err(error);
            }
        };

        if let Err(error) = self.cache.store(schema_url.clone(), schema.clone()).await {
            tracing::warn!(%error, "failed to cache schema");
        }

        Ok(schema)
    }

    fn get_validator(&self, schema_url: &Url) -> Option<Arc<JSONSchema>> {
        if self.cache().lru_expired() {
            self.validators.lock().clear();
        }

        self.validators.lock().get(schema_url).cloned()
    }

    fn add_validator(
        &self,
        schema_url: Url,
        schema: &Value,
    ) -> Result<Arc<JSONSchema>, anyhow::Error> {
        let v = Arc::new(self.create_validator(schema)?);
        self.validators.lock().put(schema_url, v.clone());
        Ok(v)
    }

    #[async_recursion(?Send)]
    #[must_use]
    pub(crate) async fn resolve_schema(&self, url: Url) -> Result<Arc<Value>, anyhow::Error> {
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
                let val = val;
                drop(self.cache.store(url, val.clone()));
                Ok(val)
            }
        }
    }

    fn create_validator(&self, schema: &Value) -> Result<JSONSchema, anyhow::Error> {
        JSONSchema::options()
            .with_resolver(CacheSchemaResolver {
                cache: self.cache().clone(),
            })
            .with_format("semver", formats::semver)
            .with_format("semver-requirement", formats::semver_req)
            .compile(schema)
            .map_err(|err| anyhow!("invalid schema: {err}"))
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
    #[tracing::instrument(level = "debug", skip_all, fields(%schema_url, %path))]
    pub async fn schemas_at_path(
        &self,
        schema_url: &Url,
        value: &Value,
        path: &Keys,
    ) -> Result<Vec<(Keys, Arc<Value>)>, anyhow::Error> {
        let mut schemas = Vec::new();
        let schema = self.load_schema(schema_url).await?;
        self.collect_schemas(
            schema_url,
            &schema,
            value,
            Keys::empty(),
            path,
            &mut schemas,
        )
        .await?;

        schemas = schemas
            .into_iter()
            .unique_by(|(k, s)| (k.clone(), ArcHashValue(s.clone())))
            .collect();

        Ok(schemas)
    }

    #[tracing::instrument(level = "debug", skip_all, fields(%path))]
    #[async_recursion(?Send)]
    #[must_use]
    async fn collect_schemas(
        &self,
        root_url: &Url,
        schema: &Value,
        value: &Value,
        full_path: Keys,
        path: &Keys,
        schemas: &mut Vec<(Keys, Arc<Value>)>,
    ) -> Result<(), anyhow::Error> {
        if !schema.is_object() {
            return Ok(());
        }

        if let Some(r) = schema.schema_ref() {
            let url = reference_url(root_url, r)
                .ok_or_else(|| anyhow!("could not determine schema URL"))?;
            let schema = self.resolve_schema(url).await?;
            return self
                .collect_schemas(root_url, &schema, value, full_path.clone(), path, schemas)
                .await;
        }

        if let Some(one_ofs) = schema["oneOf"].as_array() {
            for one_of in one_ofs {
                self.collect_schemas(root_url, one_of, value, full_path.clone(), path, schemas)
                    .await?;
            }
        }

        if let Some(any_ofs) = schema["anyOf"].as_array() {
            for any_of in any_ofs {
                self.collect_schemas(root_url, any_of, value, full_path.clone(), path, schemas)
                    .await?;
            }
        }

        if let Some(all_ofs) = schema["allOf"].as_array() {
            for all_of in all_ofs {
                self.collect_schemas(root_url, all_of, value, full_path.clone(), path, schemas)
                    .await?;
            }
        }

        let key = match path.iter().next() {
            Some(k) => k,
            None => {
                schemas.push((full_path.clone(), Arc::new(schema.clone())));
                return Ok(());
            }
        };

        let child_path = path.skip_left(1);

        match key {
            KeyOrIndex::Key(k) => {
                // For array of tables.
                self.collect_schemas(
                    root_url,
                    &schema["items"][k.value()],
                    value,
                    full_path.join(k.clone()),
                    &child_path,
                    schemas,
                )
                .await?;

                self.collect_schemas(
                    root_url,
                    &schema["properties"][k.value()],
                    &value[k.value()],
                    full_path.join(k.clone()),
                    &child_path,
                    schemas,
                )
                .await?;

                self.collect_schemas(
                    root_url,
                    &schema["additionalProperties"],
                    &value[k.value()],
                    full_path.join(k.clone()),
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
                                    full_path.join(k.clone()),
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
                        full_path.join(*idx),
                        &child_path,
                        schemas,
                    )
                    .await?;
                } else {
                    self.collect_schemas(
                        root_url,
                        &schema["items"],
                        &value[idx],
                        full_path.join(*idx),
                        &child_path,
                        schemas,
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }

    #[tracing::instrument(level = "debug", skip_all, fields(%schema_url, %path))]
    pub async fn possible_schemas_from(
        &self,
        schema_url: &Url,
        value: &Value,
        path: &Keys,
        max_depth: usize,
    ) -> Result<Vec<(Keys, Keys, Arc<Value>)>, anyhow::Error> {
        let schemas = self.schemas_at_path(schema_url, value, path).await?;

        let mut children = Vec::with_capacity(schemas.len());

        for (path, schema) in schemas {
            self.collect_child_schemas(
                schema_url,
                &*schema,
                &path,
                &Keys::empty(),
                max_depth,
                &mut children,
            )
            .await;
        }

        children = children
            .into_iter()
            .unique_by(|(k1, k2, s)| (k1.clone(), k2.clone(), ArcHashValue(s.clone())))
            .collect();

        Ok(children)
    }

    #[async_recursion(?Send)]
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    async fn collect_child_schemas(
        &self,
        root_url: &Url,
        schema: &Value,
        root_path: &Keys,
        path: &Keys,
        mut depth: usize,
        schemas: &mut Vec<(Keys, Keys, Arc<Value>)>,
    ) {
        if !schema.is_object() || depth == 0 {
            return;
        }

        if let Some(r) = schema.schema_ref() {
            let url = match reference_url(root_url, r)
                .ok_or_else(|| anyhow!("could not determine schema URL"))
            {
                Ok(u) => u,
                Err(error) => {
                    tracing::error!(?error, "failed to resolve schema");
                    return;
                }
            };
            let schema = match self.resolve_schema(url).await {
                Ok(s) => s,
                Err(error) => {
                    tracing::error!(?error, "failed to resolve schema");
                    return;
                }
            };

            return self
                .collect_child_schemas(root_url, &schema, root_path, path, depth, schemas)
                .await;
        }


        let mut subschemas_only = true;
        if let Some(ext) = schema_ext_of(schema) {
            for plugin_name in ext.plugins {
                if let Some(p) = self.plugins.load().get(plugin_name.as_str()) {
                    match p
                        .possible_schemas(self, root_url, schema, root_path, path, schemas)
                        .await
                    {
                        plugin::CollectSchemasAction::Continue => {}
                        plugin::CollectSchemasAction::Stop => return,
                    }
                }
            }

            if let Some(true) = ext.hidden {
                return;
            }

            if let Some(s) = ext.subschemas_only {
                subschemas_only = s;
            }
        }

        let mut had_subschemas = false;

        if let Some(one_ofs) = schema["oneOf"].as_array() {
            had_subschemas = true;
            for one_of in one_ofs {
                self.collect_child_schemas(root_url, one_of, root_path, path, depth, schemas)
                    .await;
            }
        }

        if let Some(any_ofs) = schema["anyOf"].as_array() {
            had_subschemas = true;
            for any_of in any_ofs {
                self.collect_child_schemas(root_url, any_of, root_path, path, depth, schemas)
                    .await;
            }
        }

        // FIXME: somehow merge allofs?
        if let Some(all_ofs) = schema["allOf"].as_array() {
            had_subschemas = true;
            for all_of in all_ofs {
                self.collect_child_schemas(root_url, all_of, root_path, path, depth, schemas)
                    .await;
            }
        }

        if had_subschemas && subschemas_only {
            return;
        }

        schemas.push((
            root_path.extend(path.clone()),
            path.clone(),
            Arc::new(schema.clone()),
        ));

        depth -= 1;

        if let Some(map) = schema["properties"].as_object() {
            for (k, v) in map {
                self.collect_child_schemas(
                    root_url,
                    v,
                    root_path,
                    &path.join(Key::from(k)),
                    depth,
                    schemas,
                )
                .await;
            }
        }
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

pub trait ValueExt {
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

struct CacheSchemaResolver<E: Environment> {
    cache: Cache<E>,
}

impl<E: Environment> SchemaResolver for CacheSchemaResolver<E> {
    fn resolve(
        &self,
        _root_schema: &serde_json::Value,
        url: &Url,
        _original_ref: &str,
    ) -> Result<Arc<serde_json::Value>, jsonschema::SchemaResolverError> {
        self.cache
            .get_schema(url)
            .ok_or_else(|| WouldBlockError.into())
    }
}

#[derive(Debug, Error)]
#[error("retrieving the schema requires external operations")]
struct WouldBlockError;

/// A validation error that contains text ranges as well.
#[derive(Debug)]
pub struct NodeValidationError {
    pub keys: Keys,
    pub node: dom::Node,
    pub error: ValidationError<'static>,
}

impl NodeValidationError {
    fn new(root: &dom::Node, error: ValidationError<'static>) -> Result<Self, anyhow::Error> {
        let mut keys = Keys::empty();
        let mut node = root.clone();

        'outer: for path in &error.instance_path {
            match path {
                jsonschema::paths::PathChunk::Property(p) => match node {
                    dom::Node::Table(t) => {
                        let entries = t.entries().read();
                        for (k, entry) in entries.iter() {
                            if k.value() == &**p {
                                keys = keys.join(k.clone());
                                node = entry.clone();
                                continue 'outer;
                            }
                        }
                        return Err(anyhow!("invalid key"));
                    }
                    _ => return Err(anyhow!("invalid key")),
                },
                jsonschema::paths::PathChunk::Index(idx) => {
                    node = node.try_get(*idx).map_err(|_| anyhow!("invalid index"))?;
                    keys = keys.join(*idx);
                }
                jsonschema::paths::PathChunk::Keyword(_) => {}
            }
        }

        Ok(Self { keys, node, error })
    }
}

mod formats {
    pub(super) fn semver(value: &str) -> bool {
        semver::Version::parse(value).is_ok()
    }

    pub(super) fn semver_req(value: &str) -> bool {
        semver::VersionReq::parse(value).is_ok()
    }
}
