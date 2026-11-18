# mod.extend

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 125-136)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn extend<I, K>(&self, keys: I) -> Self
    where
        I: IntoIterator<Item = K>,
        K: Into<KeyOrIndex>,
    {
        Self::new(
            self.keys
                .iter()
                .cloned()
                .chain(keys.into_iter().map(Into::into)),
        )
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn extend<I, K>(&self, keys: I) -> Self
    where
        I: IntoIterator<Item = K>,
        K: Into<KeyOrIndex>,
    {
        Self::new(
            self.keys
                .iter()
                .cloned()
                .chain(keys.into_iter().map(Into::into)),
        )
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
