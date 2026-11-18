# nodes.join

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 252-254)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn join(&self, key: impl Into<KeyOrIndex>) -> Keys {
        Keys::new(once(self.clone().into()).chain(once(key.into())))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn join(&self, key: impl Into<KeyOrIndex>) -> Keys {
        Keys::new(once(self.clone().into()).chain(once(key.into())))
    }
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
