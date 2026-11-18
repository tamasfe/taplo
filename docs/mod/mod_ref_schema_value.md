# mod.ref_schema_value

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 601-624)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
