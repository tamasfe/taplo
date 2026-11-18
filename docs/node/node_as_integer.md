# node.as_integer

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 522-528)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn as_integer(&self) -> Option<&Integer> {
        if let Self::Integer(v) = self {
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
    pub fn as_integer(&self) -> Option<&Integer> {
        if let Self::Integer(v) = self {
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
