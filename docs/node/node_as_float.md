# node.as_float

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 530-536)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn as_float(&self) -> Option<&Float> {
        if let Self::Float(v) = self {
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
    pub fn as_float(&self) -> Option<&Float> {
        if let Self::Float(v) = self {
            Some(v)
        } else {
            None
        }
    }
```

## Git History

- Commits: 8
- Authors: 6
- Bug fixes: 2
- Features: 1

## Callers

None

## Callees

None
