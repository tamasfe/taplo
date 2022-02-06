use anyhow::anyhow;
use arc_swap::ArcSwap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::{path::PathBuf, sync::Arc};
use url::Url;

use crate::environment::Environment;

#[derive(Clone)]
pub struct Cache<E: Environment> {
    env: E,
    cache_path: Arc<ArcSwap<Option<PathBuf>>>,
}

impl<E: Environment> Cache<E> {
    pub fn new(env: E) -> Self {
        Self {
            env,
            cache_path: Default::default(),
        }
    }

    pub fn set_cache_path(&self, path: Option<PathBuf>) {
        self.cache_path.swap(Arc::new(path));
    }

    pub async fn load(&self, schema_url: &Url) -> Result<CachedJson, anyhow::Error> {
        match &**self.cache_path.load() {
            Some(cache_path) => {
                let file_name = cache_hash(schema_url);
                let p = cache_path.join(file_name);
                let schema: CachedJson = serde_json::from_slice(&self.env.read_file(&p).await?)?;
                Ok(schema)
            }
            None => Err(anyhow!("cache path not set")),
        }
    }

    pub async fn store(&self, url: Url, value: Value) -> Result<(), anyhow::Error> {
        match &**self.cache_path.load() {
            Some(cache_path) => {
                let file_name = cache_hash(&url);
                let p = cache_path.join(file_name);
                let bytes = serde_json::to_vec(&CachedJson { url, value })?;
                self.env.write_file(&p, &bytes).await?;
                Ok(())
            }
            None => Err(anyhow!("cache path not set")),
        }
    }

    pub fn is_cache_path_set(&self) -> bool {
        self.cache_path.load().is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedJson {
    pub url: Url,
    pub value: Value,
}

fn cache_hash(url: &Url) -> String {
    let mut hasher = Sha1::new();
    hasher.update(url.as_str().as_bytes());
    hex::encode(&hasher.finalize()[..])
}
