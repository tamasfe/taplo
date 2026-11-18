# node.is_integer

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 465-467)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(..))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(..))
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
