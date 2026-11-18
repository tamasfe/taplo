# cache.Cache

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\cache.rs` (lines 17-23)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct Cache<E: Environment> {
    env: E,
    expiration_times: Arc<ArcSwap<(Duration, Duration)>>,
    lru_expires_by: Arc<Mutex<OffsetDateTime>>,
    schemas: Arc<Mutex<LruCache<Url, Arc<Value>>>>,
    cache_path: Arc<ArcSwap<Option<PathBuf>>>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct Cache<E: Environment> {
    env: E,
    expiration_times: Arc<ArcSwap<(Duration, Duration)>>,
    lru_expires_by: Arc<Mutex<OffsetDateTime>>,
    schemas: Arc<Mutex<LruCache<Url, Arc<Value>>>>,
    cache_path: Arc<ArcSwap<Option<PathBuf>>>,
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
