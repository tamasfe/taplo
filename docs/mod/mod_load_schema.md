# mod.load_schema

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 191-218)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn load_schema(&self, schema_url: &Url) -> Result<Arc<Value>, anyhow::Error> {
        if let Ok(s) = self.cache.load(schema_url, false).await {
            tracing::debug!(%schema_url, "schema was found in cache");
            return Ok(s);
        }

        let schema = if let Some(builtin) = builtin_schema(schema_url) {
            builtin
        } else {
            match self.fetch_external(schema_url).await {
                Ok(s) => Arc::new(s),
                Err(error) => {
                    tracing::warn!(%error, "failed to fetch schema");
                    if let Ok(s) = self.cache.load(schema_url, true).await {
                        tracing::debug!(%schema_url, "expired schema was found in cache");
                        return Ok(s);
                    }
                    return Err(error);
                }
            }
        };

        if let Err(error) = self.cache.store(schema_url.clone(), schema.clone()).await {
            tracing::debug!(%error, "failed to cache schema");
        }

        Ok(schema)
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub async fn load_schema(&self, schema_url: &Url) -> Result<Arc<Value>, anyhow::Error> {
        if let Ok(s) = self.cache.load(schema_url, false).await {
            tracing::debug!(%schema_url, "schema was found in cache");
            return Ok(s);
        }

        let schema = if let Some(builtin) = builtin_schema(schema_url) {
            builtin
        } else {
            match self.fetch_external(schema_url).await {
                Ok(s) => Arc::new(s),
                Err(error) => {
                    tracing::warn!(%error, "failed to fetch schema");
                    if let Ok(s) = self.cache.load(schema_url, true).await {
                        tracing::debug!(%schema_url, "expired schema was found in cache");
                        return Ok(s);
                    }
                    return Err(error);
                }
            }
        };

        if let Err(error) = self.cache.store(schema_url.clone(), schema.clone()).await {
            tracing::debug!(%error, "failed to cache schema");
        }

        Ok(schema)
    }
```

## Git History

- Commits: 13
- Authors: 5
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
