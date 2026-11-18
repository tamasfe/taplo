# node.try_into_table

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 554-560)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn try_into_table(self) -> Result<Table, Self> {
        if let Self::Table(v) = self {
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
    pub fn try_into_table(self) -> Result<Table, Self> {
        if let Self::Table(v) = self {
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
