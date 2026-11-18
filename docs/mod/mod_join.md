# mod.join

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 121-123)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn join(&self, key: impl Into<KeyOrIndex>) -> Self {
        self.extend(once(key.into()))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn join(&self, key: impl Into<KeyOrIndex>) -> Self {
        self.extend(once(key.into()))
    }
```

## Git History

- Commits: 3
- Authors: 3
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
