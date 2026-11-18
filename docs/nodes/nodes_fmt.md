# nodes.fmt

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node\nodes.rs` (lines 585-590)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegerValue::Negative(v) => v.fmt(f),
            IntegerValue::Positive(v) => v.fmt(f),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegerValue::Negative(v) => v.fmt(f),
            IntegerValue::Positive(v) => v.fmt(f),
        }
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
