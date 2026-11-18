# cache.set_expiration_times

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\cache.rs` (lines 116-118)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn set_expiration_times(&self, mem: Duration, disk: Duration) {
        self.expiration_times.store(Arc::new((mem, disk)));
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn set_expiration_times(&self, mem: Duration, disk: Duration) {
        self.expiration_times.store(Arc::new((mem, disk)));
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
