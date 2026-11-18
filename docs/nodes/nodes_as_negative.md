# nodes.as_negative

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 567-573)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn as_negative(&self) -> Option<i64> {
        if let Self::Negative(v) = self {
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
    pub fn as_negative(&self) -> Option<i64> {
        if let Self::Negative(v) = self {
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
