# cache.new

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\cache.rs` (lines 26-40)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
