# node.is_date

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 479-481)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_date(&self) -> bool {
        matches!(self, Self::Date(..))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_date(&self) -> bool {
        matches!(self, Self::Date(..))
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
