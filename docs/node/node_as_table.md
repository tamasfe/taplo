# node.as_table

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 490-496)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn as_table(&self) -> Option<&Table> {
        if let Self::Table(v) = self {
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
    pub fn as_table(&self) -> Option<&Table> {
        if let Self::Table(v) = self {
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
