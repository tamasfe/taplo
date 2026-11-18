# node.is_array

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 444-446)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(..))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(..))
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
