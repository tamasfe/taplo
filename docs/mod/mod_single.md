# mod.single

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 111-113)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn single(key: impl Into<KeyOrIndex>) -> Self {
        Self::new(once(key.into()))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn single(key: impl Into<KeyOrIndex>) -> Self {
        Self::new(once(key.into()))
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
