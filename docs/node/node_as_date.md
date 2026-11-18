# node.as_date

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 538-544)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn as_date(&self) -> Option<&DateTime> {
        if let Self::Date(v) = self {
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
    pub fn as_date(&self) -> Option<&DateTime> {
        if let Self::Date(v) = self {
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
