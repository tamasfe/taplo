# mod.as_index

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 90-96)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn as_index(&self) -> Option<&usize> {
        if let Self::Index(v) = self {
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
    pub fn as_index(&self) -> Option<&usize> {
        if let Self::Index(v) = self {
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
