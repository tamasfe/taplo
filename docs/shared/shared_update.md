# shared.update

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\shared.rs` (lines 55-59)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) fn update(&self, f: impl FnOnce(&mut T)) {
        let mut inner = self.0.load_full().unwrap();
        f(Arc::make_mut(&mut inner));
        self.0.store(Some(inner))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub(crate) fn update(&self, f: impl FnOnce(&mut T)) {
        let mut inner = self.0.load_full().unwrap();
        f(Arc::make_mut(&mut inner));
        self.0.store(Some(inner))
    }
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
