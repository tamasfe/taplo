# associations.new

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 53-63)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) fn new(env: E, cache: Cache<E>, http: reqwest::Client) -> Self {
        let this = Self {
            concurrent_requests: Arc::new(Semaphore::new(10)),
            cache,
            env,
            http,
            associations: Default::default(),
        };
        this.add_builtins();
        this
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub(crate) fn new(env: E, cache: Cache<E>, http: reqwest::Client) -> Self {
        let this = Self {
            concurrent_requests: Arc::new(Semaphore::new(10)),
            cache,
            env,
            http,
            associations: Default::default(),
        };
        this.add_builtins();
        this
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
