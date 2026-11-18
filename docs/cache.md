# Module: cache

**File:** `\crates\taplo-common\src\schema\cache.rs`

## Overview

Module `cache` contains 13 analyzed entities.

```
use anyhow::anyhow;
use arc_swap::ArcSwap;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::{num::NonZeroUsize, path::PathBuf, sync::Arc, time::Duration};
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
            schemas: Arc::new(Mutex::new(LruCache::with_hasher(
                NonZeroUsize::new(10).unwrap(),
                ahash::RandomState::new(),
            ))),
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
            self.schemas.lock().clea
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `cache.new` | function | No description |
| `cache.lru_expired` | function | No description |
| `cache.load` | function | No description |
| `cache.save` | function | No description |
| `cache.cache_hash` | function | No description |
| `cache.set_cache_path` | function | No description |
| `cache.contains_schema` | function | No description |
| `cache.get_schema` | function | No description |
| `cache.is_cache_path_set` | function | No description |
| `cache.set_expiration_times` | function | No description |
| `cache.store` | function | No description |
| `cache.Cache` | struct | No description |
| `cache.CachedJson` | struct | No description |

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
