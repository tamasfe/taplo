# node.try_into_str

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 578-584)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn try_into_str(self) -> Result<Str, Self> {
        if let Self::Str(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn try_into_str(self) -> Result<Str, Self> {
        if let Self::Str(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
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
