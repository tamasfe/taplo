# nodes.as_positive

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 575-581)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn as_positive(&self) -> Option<u64> {
        if let Self::Positive(v) = self {
            Some(*v)
        } else {
            None
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn as_positive(&self) -> Option<u64> {
        if let Self::Positive(v) = self {
            Some(*v)
        } else {
            None
        }
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
