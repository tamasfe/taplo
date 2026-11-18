# shared.from

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\shared.rs` (lines 63-65)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn from(value: T) -> Self {
        Self(ArcSwapOption::new(Some(Arc::new(value))))
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn from(value: T) -> Self {
        Self(ArcSwapOption::new(Some(Arc::new(value))))
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
