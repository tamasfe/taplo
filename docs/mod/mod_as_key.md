# mod.as_key

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 82-88)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn as_key(&self) -> Option<&Key> {
        if let Self::Key(v) = self {
            Some(v)
        } else {
            None
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn as_key(&self) -> Option<&Key> {
        if let Self::Key(v) = self {
            Some(v)
        } else {
            None
        }
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
