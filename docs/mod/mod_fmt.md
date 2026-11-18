# mod.fmt

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 26-28)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:?})", &self.message, &self.range)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:?})", &self.message, &self.range)
    }
```

## Git History

- Commits: 8
- Authors: 5
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
