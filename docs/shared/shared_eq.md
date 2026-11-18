# shared.eq

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\shared.rs` (lines 23-25)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn eq(&self, other: &Self) -> bool {
        *self.0.load() == *other.0.load()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn eq(&self, other: &Self) -> bool {
        *self.0.load() == *other.0.load()
    }
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
