# node.is_table

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 437-439)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_table(&self) -> bool {
        matches!(self, Self::Table(..))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_table(&self) -> bool {
        matches!(self, Self::Table(..))
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
