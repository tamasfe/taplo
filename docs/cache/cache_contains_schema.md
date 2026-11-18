# cache.contains_schema

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\cache.rs` (lines 46-48)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn contains_schema(&self, url: &Url) -> bool {
        self.schemas.lock().contains(url)
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn contains_schema(&self, url: &Url) -> bool {
        self.schemas.lock().contains(url)
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
