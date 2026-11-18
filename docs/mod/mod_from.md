# mod.from

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\mod.rs` (lines 232-234)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn from(key: Key) -> Self {
        Keys::new(once(key.into()))
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn from(key: Key) -> Self {
        Keys::new(once(key.into()))
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
