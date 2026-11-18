# cache.get_schema

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\cache.rs` (lines 42-44)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn get_schema(&self, url: &Url) -> Option<Arc<Value>> {
        self.schemas.lock().get(url).cloned()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn get_schema(&self, url: &Url) -> Option<Arc<Value>> {
        self.schemas.lock().get(url).cloned()
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
