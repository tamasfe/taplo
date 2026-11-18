# cache.load

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\cache.rs` (lines 54-86)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
```

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
