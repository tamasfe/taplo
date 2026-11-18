# shared.default

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\shared.rs` (lines 34-36)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn default() -> Self {
        Self(ArcSwapOption::new(Some(Arc::new(T::default()))))
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn default() -> Self {
        Self(ArcSwapOption::new(Some(Arc::new(T::default()))))
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
