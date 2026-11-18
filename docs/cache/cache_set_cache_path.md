# cache.set_cache_path

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\cache.rs` (lines 50-52)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn set_cache_path(&self, path: Option<PathBuf>) {
        self.cache_path.swap(Arc::new(path));
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn set_cache_path(&self, path: Option<PathBuf>) {
        self.cache_path.swap(Arc::new(path));
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
