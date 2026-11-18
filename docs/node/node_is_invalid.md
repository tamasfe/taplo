# node.is_invalid

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 486-488)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(..))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(..))
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
