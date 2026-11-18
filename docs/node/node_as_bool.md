# node.as_bool

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 506-512)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn as_bool(&self) -> Option<&Bool> {
        if let Self::Bool(v) = self {
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
    pub fn as_bool(&self) -> Option<&Bool> {
        if let Self::Bool(v) = self {
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
