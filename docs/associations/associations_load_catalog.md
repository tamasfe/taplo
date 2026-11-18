# associations.load_catalog

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 327-356)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
```

## Git History

- Commits: 16
- Authors: 5
- Bug fixes: 7
- Features: 4

## Callers

None

## Callees

None
