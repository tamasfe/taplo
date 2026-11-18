# shared.get

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\shared.rs` (lines 43-45)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn get(&self) -> Arc<T> {
        self.0.load_full().unwrap()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn get(&self) -> Arc<T> {
        self.0.load_full().unwrap()
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
