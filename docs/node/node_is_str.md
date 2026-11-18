# node.is_str

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 458-460)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_str(&self) -> bool {
        matches!(self, Self::Str(..))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_str(&self) -> bool {
        matches!(self, Self::Str(..))
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
