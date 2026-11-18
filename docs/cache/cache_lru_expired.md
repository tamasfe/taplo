# cache.lru_expired

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\cache.rs` (lines 122-130)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn lru_expired(&self) -> bool {
        let now = self.env.now();
        let expires_by = *self.lru_expires_by.lock();
        let expired = expires_by < now;
        if expired {
            *(self.lru_expires_by.lock()) = now + self.expiration_times.load().0;
        }
        expired
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn lru_expired(&self) -> bool {
        let now = self.env.now();
        let expires_by = *self.lru_expires_by.lock();
        let expired = expires_by < now;
        if expired {
            *(self.lru_expires_by.lock()) = now + self.expiration_times.load().0;
        }
        expired
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
