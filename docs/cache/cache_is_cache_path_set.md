# cache.is_cache_path_set

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\cache.rs` (lines 112-114)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_cache_path_set(&self) -> bool {
        self.cache_path.load().is_some()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_cache_path_set(&self) -> bool {
        self.cache_path.load().is_some()
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
