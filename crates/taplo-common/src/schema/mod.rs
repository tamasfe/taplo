use self::{associations::SchemaAssociations, builtins::builtin_schema, cache::Cache};
use crate::{environment::Environment, util::ArcHashValue, HashMap, LruCache};
use anyhow::{anyhow, Context};
use async_recursion::async_recursion;
use futures::{stream::FuturesUnordered, StreamExt};
use itertools::Itertools;
use json_value_merge::Merge;
use jsonschema::{error::ValidationErrorKind, JSONSchema, SchemaResolver, ValidationError};
use parking_lot::Mutex;
use regex::Regex;
use serde_json::Value;
use std::{borrow::Cow, num::NonZeroUsize, sync::Arc};
use taplo::{
    dom::{self, node::Key, KeyOrIndex, Keys},
    rowan::TextRange,
};
use thiserror::Error;
use tokio::sync::{watch, Semaphore};
use url::Url;

pub mod associations;
pub mod cache;
pub mod ext;

pub mod builtins {
    use serde_json::Value;
    use std::sync::Arc;
    use url::Url;

    pub const TAPLO_CONFIG_URL: &str = "taplo://taplo.toml";

    #[must_use]
    pub fn taplo_config_schema() -> Arc<Value> {
        Arc::new(serde_json::to_value(schemars::schema_for!(crate::config::Config)).unwrap())
    }

    #[must_use]
    pub fn builtin_schema(url: &Url) -> Option<Arc<Value>> {
        if url.as_str() == TAPLO_CONFIG_URL {
            Some(taplo_config_schema())
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Schemas<E: Environment> {
    env: E,
    associations: SchemaAssociations<E>,
    concurrent_requests: Arc<Semaphore>,
    in_flight_remote_fetches:
        Arc<Mutex<HashMap<Url, watch::Sender<Option<InFlightRemoteFetchResult>>>>>,
    http: reqwest::Client,
    validators: Arc<Mutex<LruCache<Url, Arc<JSONSchema>>>>,
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
            in_flight_remote_fetches: Arc::new(Mutex::new(HashMap::default())),
            http,
            validators: Arc::new(Mutex::new(LruCache::with_hasher(
                NonZeroUsize::new(3).unwrap(),
                ahash::RandomState::new(),
            ))),
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
}

impl<E: Environment> Schemas<E> {
    #[tracing::instrument(skip_all, fields(%schema_url))]
    pub async fn validate_root(
        &self,
        schema_url: &Url,
        root: &dom::Node,
    ) -> Result<Vec<NodeValidationError>, anyhow::Error> {
        let value = serde_json::to_value(root)?;
        self.validate(schema_url, &value)
            .await?
            .into_iter()
            .map(|error| NodeValidationError::new(root, error))
            .collect::<Result<Vec<_>, _>>()
    }

    #[tracing::instrument(skip_all, fields(%schema_url))]
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
                Ok(()) => return Ok(Vec::new()),
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
                        external_schema_result?;
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

    #[tracing::instrument(skip_all, fields(%schema_url))]
    pub async fn load_schema(&self, schema_url: &Url) -> Result<Arc<Value>, anyhow::Error> {
        if let Ok(s) = self.cache.load(schema_url, false).await {
            tracing::debug!(%schema_url, "schema was found in cache");
            return Ok(s);
        }

        let use_remote_failure_controls = matches!(schema_url.scheme(), "http" | "https");

        if use_remote_failure_controls && self.cache.recently_failed(schema_url) {
            tracing::debug!(%schema_url, "skipping schema fetch due to recent failure");

            if let Ok(s) = self.cache.load(schema_url, true).await {
                tracing::debug!(%schema_url, "expired schema was found in cache");
                return Ok(s);
            }

            return Err(anyhow!("schema fetch recently failed"));
        }

        if use_remote_failure_controls {
            if let Some(mut completion) = self.join_in_flight_remote_fetch(schema_url) {
                let _ = completion.changed().await;

                match completion.borrow().clone() {
                    Some(Ok(schema)) => {
                        tracing::debug!(%schema_url, "schema fetch completed in another task");
                        return Ok(schema);
                    }
                    Some(Err(error)) => return Err(anyhow!(error)),
                    None => {}
                }

                if self.cache.recently_failed(schema_url) {
                    tracing::debug!(%schema_url, "schema fetch failed while waiting on another task");

                    if let Ok(s) = self.cache.load(schema_url, true).await {
                        tracing::debug!(%schema_url, "expired schema was found in cache");
                        return Ok(s);
                    }

                    return Err(anyhow!("schema fetch recently failed"));
                }
            }
        }

        let _in_flight_remote_fetch = use_remote_failure_controls
            .then(|| InFlightRemoteFetchGuard::new(self, schema_url.clone()));

        let schema = if let Some(builtin) = builtin_schema(schema_url) {
            builtin
        } else {
            let schema = match self.fetch_external(schema_url).await {
                Ok(s) => {
                    if use_remote_failure_controls {
                        self.cache.clear_failure(schema_url);
                    }
                    Arc::new(s)
                }
                Err(error) => {
                    if use_remote_failure_controls {
                        self.cache.remember_failure(schema_url.clone());
                    }
                    tracing::warn!(%error, "failed to fetch schema");
                    if let Ok(s) = self.cache.load(schema_url, true).await {
                        tracing::debug!(%schema_url, "expired schema was found in cache");
                        if use_remote_failure_controls {
                            self.publish_in_flight_remote_fetch(schema_url, Ok(s.clone()));
                        }
                        return Ok(s);
                    }
                    if use_remote_failure_controls {
                        self.publish_in_flight_remote_fetch(schema_url, Err(error.to_string()));
                    }
                    return Err(error);
                }
            };

            if use_remote_failure_controls {
                self.publish_in_flight_remote_fetch(schema_url, Ok(schema.clone()));
            }

            schema
        };

        if let Err(error) = self.cache.store(schema_url.clone(), schema.clone()).await {
            tracing::debug!(%error, "failed to cache schema");
        }

        Ok(schema)
    }

    fn join_in_flight_remote_fetch(
        &self,
        schema_url: &Url,
    ) -> Option<watch::Receiver<Option<InFlightRemoteFetchResult>>> {
        let mut in_flight = self.in_flight_remote_fetches.lock();

        match in_flight.get(schema_url) {
            Some(completion) => Some(completion.subscribe()),
            None => {
                let (completion, _) = watch::channel(None);
                in_flight.insert(schema_url.clone(), completion);
                None
            }
        }
    }

    fn publish_in_flight_remote_fetch(&self, schema_url: &Url, result: InFlightRemoteFetchResult) {
        if let Some(completion) = self.in_flight_remote_fetches.lock().get(schema_url) {
            completion.send_replace(Some(result));
        }
    }

    fn finish_in_flight_remote_fetch(&self, schema_url: &Url) {
        self.in_flight_remote_fetches.lock().remove(schema_url);
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
                            .to_file_path_normalized(schema_url)
                            .ok_or_else(|| anyhow!("invalid file path"))?
                            .as_ref(),
                    )
                    .await?,
            )?),
            scheme => Err(anyhow!("the scheme `{scheme}` is not supported")),
        }
    }
}

type InFlightRemoteFetchResult = Result<Arc<Value>, String>;

struct InFlightRemoteFetchGuard<'a, E: Environment> {
    schemas: &'a Schemas<E>,
    schema_url: Url,
}

impl<'a, E: Environment> InFlightRemoteFetchGuard<'a, E> {
    fn new(schemas: &'a Schemas<E>, schema_url: Url) -> Self {
        Self {
            schemas,
            schema_url,
        }
    }
}

impl<E: Environment> Drop for InFlightRemoteFetchGuard<'_, E> {
    fn drop(&mut self) {
        self.schemas.finish_in_flight_remote_fetch(&self.schema_url);
    }
}

#[cfg(test)]
mod tests {
    use super::Schemas;
    use crate::environment::Environment;
    use async_trait::async_trait;
    use futures::future;
    use std::{
        path::{Path, PathBuf},
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
    };
    use time::OffsetDateTime;
    use tokio::{
        io::{empty, sink, AsyncReadExt, AsyncWriteExt, Empty, Sink},
        net::TcpListener,
        time::{sleep, timeout, Duration},
    };
    use url::Url;

    #[derive(Clone, Default)]
    struct TestEnvironment {
        reads: Arc<AtomicUsize>,
    }

    impl TestEnvironment {
        fn read_count(&self) -> usize {
            self.reads.load(Ordering::SeqCst)
        }
    }

    #[async_trait(?Send)]
    impl Environment for TestEnvironment {
        type Stdin = Empty;
        type Stdout = Sink;
        type Stderr = Sink;

        fn now(&self) -> OffsetDateTime {
            OffsetDateTime::now_utc()
        }

        fn spawn<F>(&self, fut: F)
        where
            F: futures::Future + Send + 'static,
            F::Output: Send,
        {
            drop(tokio::spawn(fut));
        }

        fn spawn_local<F>(&self, _fut: F)
        where
            F: futures::Future + 'static,
        {
            panic!("spawn_local is not used in this test");
        }

        fn env_var(&self, _name: &str) -> Option<String> {
            None
        }

        fn env_vars(&self) -> Vec<(String, String)> {
            Vec::new()
        }

        fn atty_stderr(&self) -> bool {
            false
        }

        fn stdin(&self) -> Self::Stdin {
            empty()
        }

        fn stdout(&self) -> Self::Stdout {
            sink()
        }

        fn stderr(&self) -> Self::Stderr {
            sink()
        }

        fn glob_files(&self, _glob: &str) -> Result<Vec<PathBuf>, anyhow::Error> {
            Ok(Vec::new())
        }

        async fn read_file(&self, _path: &Path) -> Result<Vec<u8>, anyhow::Error> {
            self.reads.fetch_add(1, Ordering::SeqCst);
            Ok(br#"not json"#.to_vec())
        }

        async fn write_file(&self, _path: &Path, _bytes: &[u8]) -> Result<(), anyhow::Error> {
            Ok(())
        }

        fn to_file_path(&self, url: &Url) -> Option<PathBuf> {
            url.to_file_path().ok()
        }

        fn is_absolute(&self, path: &Path) -> bool {
            path.is_absolute()
        }

        fn cwd(&self) -> Option<PathBuf> {
            None
        }

        async fn find_config_file(&self, _from: &Path) -> Option<PathBuf> {
            None
        }
    }

    async fn start_invalid_schema_server(request_count: Arc<AtomicUsize>) -> Url {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            for _ in 0..4 {
                let Ok((mut stream, _)) = listener.accept().await else {
                    break;
                };

                request_count.fetch_add(1, Ordering::SeqCst);

                tokio::spawn(async move {
                    let mut buf = [0_u8; 1024];
                    let _ = stream.read(&mut buf).await;
                    let response = concat!(
                        "HTTP/1.1 403 Forbidden\r\n",
                        "content-type: text/plain\r\n",
                        "content-length: 8\r\n",
                        "connection: close\r\n",
                        "\r\n",
                        "not json",
                    );
                    let _ = stream.write_all(response.as_bytes()).await;
                    let _ = stream.shutdown().await;
                });
            }
        });

        Url::parse(&format!("http://{addr}/schema.json")).unwrap()
    }

    async fn start_cancellation_schema_server(request_count: Arc<AtomicUsize>) -> Url {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            for request_index in 0..2 {
                let Ok((mut stream, _)) = listener.accept().await else {
                    break;
                };

                request_count.fetch_add(1, Ordering::SeqCst);

                tokio::spawn(async move {
                    let mut buf = [0_u8; 1024];
                    let _ = stream.read(&mut buf).await;

                    if request_index == 0 {
                        future::pending::<()>().await;
                    } else {
                        let response = concat!(
                            "HTTP/1.1 403 Forbidden\r\n",
                            "content-type: text/plain\r\n",
                            "content-length: 8\r\n",
                            "connection: close\r\n",
                            "\r\n",
                            "not json",
                        );
                        let _ = stream.write_all(response.as_bytes()).await;
                        let _ = stream.shutdown().await;
                    }
                });
            }
        });

        Url::parse(&format!("http://{addr}/schema.json")).unwrap()
    }

    #[test]
    fn recent_remote_schema_failures_are_not_refetched() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            let env = TestEnvironment::default();
            let client = reqwest::Client::builder().build().unwrap();
            let schemas = Schemas::new(env.clone(), client);
            let request_count = Arc::new(AtomicUsize::new(0));
            let schema_url = start_invalid_schema_server(request_count.clone()).await;

            assert!(schemas.load_schema(&schema_url).await.is_err());
            assert_eq!(request_count.load(Ordering::SeqCst), 1);

            assert!(schemas.load_schema(&schema_url).await.is_err());
            assert_eq!(request_count.load(Ordering::SeqCst), 1);
        });
    }

    #[test]
    fn recent_remote_schema_failures_are_coalesced_across_concurrent_requests() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            let env = TestEnvironment::default();
            let client = reqwest::Client::builder().build().unwrap();
            let schemas = Schemas::new(env, client);
            let request_count = Arc::new(AtomicUsize::new(0));
            let schema_url = start_invalid_schema_server(request_count.clone()).await;

            let (left, right) = future::join(
                schemas.load_schema(&schema_url),
                schemas.load_schema(&schema_url),
            )
            .await;

            assert!(left.is_err());
            assert!(right.is_err());
            assert_eq!(request_count.load(Ordering::SeqCst), 1);
        });
    }

    #[test]
    fn local_file_schema_failures_are_retried() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            let env = TestEnvironment::default();
            let client = reqwest::Client::builder().build().unwrap();
            let schemas = Schemas::new(env.clone(), client);
            let schema_url = Url::parse("file:///tmp/invalid-schema.json").unwrap();

            assert!(schemas.load_schema(&schema_url).await.is_err());
            assert_eq!(env.read_count(), 1);

            assert!(schemas.load_schema(&schema_url).await.is_err());
            assert_eq!(env.read_count(), 2);
        });
    }

    #[test]
    fn canceled_remote_fetch_unblocks_waiters() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let local = tokio::task::LocalSet::new();

        runtime.block_on(local.run_until(async {
            let env = TestEnvironment::default();
            let client = reqwest::Client::builder().build().unwrap();
            let schemas = Schemas::new(env, client);
            let request_count = Arc::new(AtomicUsize::new(0));
            let schema_url = start_cancellation_schema_server(request_count.clone()).await;

            let leader_schemas = schemas.clone();
            let leader_url = schema_url.clone();
            let leader =
                tokio::task::spawn_local(
                    async move { leader_schemas.load_schema(&leader_url).await },
                );

            timeout(Duration::from_secs(1), async {
                while request_count.load(Ordering::SeqCst) < 1 {
                    sleep(Duration::from_millis(10)).await;
                }
            })
            .await
            .unwrap();

            let waiter_schemas = schemas.clone();
            let waiter_url = schema_url.clone();
            let waiter = tokio::task::spawn_local(async move {
                timeout(
                    Duration::from_secs(1),
                    waiter_schemas.load_schema(&waiter_url),
                )
                .await
            });

            sleep(Duration::from_millis(50)).await;
            leader.abort();

            let leader_result = leader.await;
            let waiter_result = waiter.await.unwrap().unwrap();
            assert!(leader_result.is_err());
            assert!(waiter_result.is_err());
            assert_eq!(request_count.load(Ordering::SeqCst), 2);
        }));
    }
}

impl<E: Environment> Schemas<E> {
    #[tracing::instrument(skip_all, fields(%schema_url, %path))]
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

    #[tracing::instrument(skip_all, fields(%path))]
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

        let include_self = schema["allOf"].is_null();

        let Some(key) = path.iter().next() else {
            if include_self {
                schemas.push((full_path.clone(), Arc::new(schema.clone())));
            }
            return Ok(());
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

    #[tracing::instrument(skip_all, fields(%schema_url, %path))]
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
                &schema,
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

        if let Some(schema) = self.ref_schema_value(root_url, schema).await {
            return self
                .collect_child_schemas(root_url, &schema, root_path, path, depth, schemas)
                .await;
        }

        if let Some(one_ofs) = schema["oneOf"].as_array() {
            for one_of in one_ofs {
                self.collect_child_schemas(root_url, one_of, root_path, path, depth, schemas)
                    .await;
            }
        }

        if let Some(any_ofs) = schema["anyOf"].as_array() {
            for any_of in any_ofs {
                self.collect_child_schemas(root_url, any_of, root_path, path, depth, schemas)
                    .await;
            }
        }

        // Deal with the { "description": "Foo", "allOf": [{ "$ref": "Bar" }] }
        // pattern.
        let composed = [
            !schema["allOf"].is_null(),
            !schema["oneOf"].is_null(),
            !schema["anyOf"].is_null(),
        ]
        .into_iter()
        .filter(|b| *b)
        .count()
            == 1
            && schema["properties"].is_null();

        if let Some(all_ofs) = schema["allOf"].as_array() {
            if !all_ofs.is_empty() && composed {
                let mut schema = schema.clone();
                if let Some(obj) = schema["allOf"].as_object_mut() {
                    obj.remove("allOf");
                }

                let mut merged_all_of = Value::Object(serde_json::Map::default());

                for all_of in all_ofs {
                    merged_all_of.merge(match self.ref_schema_value(root_url, all_of).await {
                        Some(ref schema) => schema,
                        None => all_of,
                    });
                }

                merged_all_of.merge(&schema);

                self.collect_child_schemas(
                    root_url,
                    &merged_all_of,
                    root_path,
                    path,
                    depth,
                    schemas,
                )
                .await;
            }
            // TODO: handle allOfs in regular schemas.
            // doing so currently will overflow the stack.
        }

        let include_self = !composed;

        if include_self {
            schemas.push((
                root_path.extend(path.clone()),
                path.clone(),
                Arc::new(schema.clone()),
            ));
        }

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

    async fn ref_schema_value(&self, root_url: &Url, schema: &Value) -> Option<Arc<Value>> {
        if let Some(r) = schema.schema_ref() {
            let url = match reference_url(root_url, r)
                .ok_or_else(|| anyhow!("could not determine schema URL"))
            {
                Ok(u) => u,
                Err(error) => {
                    tracing::error!(?error, "failed to resolve schema");
                    return None;
                }
            };
            let schema = match self.resolve_schema(url).await {
                Ok(s) => s,
                Err(error) => {
                    tracing::error!(?error, "failed to resolve schema");
                    return None;
                }
            };

            Some(schema)
        } else {
            None
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

        match &error.kind {
            ValidationErrorKind::AdditionalProperties { unexpected } => {
                keys = keys.extend(unexpected.iter().map(Key::from).map(KeyOrIndex::Key));
            }
            _ => {}
        }

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

    #[must_use]
    pub fn text_ranges(&self) -> Box<dyn Iterator<Item = TextRange> + '_> {
        match self.error.kind {
            ValidationErrorKind::AdditionalProperties { .. } => {
                let include_children = false;

                if self.keys.is_empty() {
                    return Box::new(self.node.text_ranges(include_children));
                }

                Box::new(
                    self.keys
                        .clone()
                        .into_iter()
                        .flat_map(move |key| self.node.get(key).text_ranges(include_children)),
                )
            }
            _ => Box::new(self.node.text_ranges(true)),
        }
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
