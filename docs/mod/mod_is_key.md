# mod.is_key

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 71-73)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_key(&self) -> bool {
        matches!(self, Self::Key(..))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_key(&self) -> bool {
        matches!(self, Self::Key(..))
    }
```

## Git History

- Commits: 3
- Authors: 3
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
