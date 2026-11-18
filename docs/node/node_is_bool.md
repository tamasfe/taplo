# node.is_bool

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 451-453)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(..))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(..))
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
