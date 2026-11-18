# nodes.is_negative

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 556-558)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_negative(&self) -> bool {
        matches!(self, Self::Negative(..))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_negative(&self) -> bool {
        matches!(self, Self::Negative(..))
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
