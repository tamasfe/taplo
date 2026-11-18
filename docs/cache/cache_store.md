# cache.store

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\cache.rs` (lines 88-91)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub async fn store(&self, url: Url, value: Arc<Value>) -> Result<(), anyhow::Error> {
        self.schemas.lock().put(url.clone(), value.clone());
        self.save(url, value).await
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub async fn store(&self, url: Url, value: Arc<Value>) -> Result<(), anyhow::Error> {
        self.schemas.lock().put(url.clone(), value.clone());
        self.save(url, value).await
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
