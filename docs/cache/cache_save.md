# cache.save

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\cache.rs` (lines 93-110)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
