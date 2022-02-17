use anyhow::anyhow;
use arc_swap::ArcSwap;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::{path::PathBuf, sync::Arc, time::Duration};
use time::OffsetDateTime;
use url::Url;

use crate::{environment::Environment, LruCache};

pub const DEFAULT_LRU_CACHE_EXPIRATION_TIME: Duration = Duration::from_secs(60);
pub const DEFAULT_CACHE_EXPIRATION_TIME: Duration = Duration::from_secs(60 * 10);

#[derive(Clone)]
pub struct Cache<E: Environment> {
    env: E,
    expiration_times: Arc<ArcSwap<(Duration, Duration)>>,
    lru_expires_by: Arc<Mutex<OffsetDateTime>>,
    schemas: Arc<Mutex<LruCache<Url, Arc<Value>>>>,
    cache_path: Arc<ArcSwap<Option<PathBuf>>>,
}

impl<E: Environment> Cache<E> {
    pub fn new(env: E) -> Self {
        Self {
            expiration_times: Arc::new(ArcSwap::new(Arc::new((
                DEFAULT_LRU_CACHE_EXPIRATION_TIME,
                DEFAULT_CACHE_EXPIRATION_TIME,
            )))),
            lru_expires_by: Arc::new(Mutex::new(env.now() + DEFAULT_LRU_CACHE_EXPIRATION_TIME)),
            env,
            schemas: Arc::new(Mutex::new(LruCache::new(10))),
            cache_path: Default::default(),
        }
    }

    pub fn get_schema(&self, url: &Url) -> Option<Arc<Value>> {
        self.schemas.lock().get(url).cloned()
    }

    pub fn contains_schema(&self, url: &Url) -> bool {
        self.schemas.lock().contains(url)
    }

    pub fn set_cache_path(&self, path: Option<PathBuf>) {
        self.cache_path.swap(Arc::new(path));
    }

    pub async fn load(
        &self,
        value_url: &Url,
        include_expired: bool,
    ) -> Result<Arc<Value>, anyhow::Error> {
        let now = self.env.now();

        // We invalidate the in-memory cache at a regular interval.
        if self.lru_expired() {
            self.schemas.lock().clear();
        }

        if let Some(s) = self.schemas.lock().get(value_url) {
            return Ok(s.clone());
        }

        match &**self.cache_path.load() {
            Some(cache_path) => {
                let file_name = cache_hash(value_url);
                let p = cache_path.join(file_name);
                let schema: CachedJson = serde_json::from_slice(&self.env.read_file(&p).await?)?;

                if !include_expired && schema.expires_by < now {
                    return Err(anyhow!("document expired"));
                }

                let s = Arc::new(schema.value);
                self.schemas.lock().put(value_url.clone(), s.clone());
                Ok(s)
            }
            None => Err(anyhow!("cache path not set")),
        }
    }

    pub async fn store(&self, url: Url, value: Arc<Value>) -> Result<(), anyhow::Error> {
        self.schemas.lock().put(url.clone(), value.clone());
        self.save(url, value).await
    }

    pub async fn save(&self, url: Url, value: Arc<Value>) -> Result<(), anyhow::Error> {
        let expires_by = self.env.now() + self.expiration_times.load().1;

        match &**self.cache_path.load() {
            Some(cache_path) => {
                let file_name = cache_hash(&url);
                let p = cache_path.join(file_name);
                let bytes = serde_json::to_vec(&CachedJson {
                    expires_by,
                    url,
                    value: (*value).clone(),
                })?;
                self.env.write_file(&p, &bytes).await?;
                Ok(())
            }
            None => Err(anyhow!("cache path not set")),
        }
    }

    pub fn is_cache_path_set(&self) -> bool {
        self.cache_path.load().is_some()
    }

    pub fn set_expiration_times(&self, mem: Duration, disk: Duration) {
        self.expiration_times.store(Arc::new((mem, disk)));
    }

    /// Reports whether the LRU cache is expired, and also resets
    /// the expiration timer in that case.
    pub fn lru_expired(&self) -> bool {
        let now = self.env.now();
        let expires_by = *self.lru_expires_by.lock();
        let expired = expires_by < now;
        if expired {
            *(self.lru_expires_by.lock()) = now + self.expiration_times.load().0;
        }
        expired
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedJson {
    pub expires_by: OffsetDateTime,
    pub url: Url,
    pub value: Value,
}

fn cache_hash(url: &Url) -> String {
    let mut hasher = Sha1::new();
    hasher.update(url.as_str().as_bytes());
    hex::encode(&hasher.finalize()[..])
}
