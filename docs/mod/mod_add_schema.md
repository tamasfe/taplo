# mod.add_schema

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\mod.rs` (lines 186-188)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn add_schema(&self, schema_url: &Url, schema: Arc<Value>) {
        drop(self.cache.store(schema_url.clone(), schema).await);
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub async fn add_schema(&self, schema_url: &Url, schema: Arc<Value>) {
        drop(self.cache.store(schema_url.clone(), schema).await);
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
