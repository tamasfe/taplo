# node.try_into_date

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 602-608)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn try_into_date(self) -> Result<DateTime, Self> {
        if let Self::Date(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn try_into_date(self) -> Result<DateTime, Self> {
        if let Self::Date(v) = self {
            Ok(v)
        } else {
            Err(self)
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
