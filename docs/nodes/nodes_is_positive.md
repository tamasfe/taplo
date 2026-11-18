# nodes.is_positive

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 563-565)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_positive(&self) -> bool {
        matches!(self, Self::Positive(..))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_positive(&self) -> bool {
        matches!(self, Self::Positive(..))
    }
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
