# mod.from_iter

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 36-38)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn from_iter<T: IntoIterator<Item = (TextRange, OptionsIncomplete)>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn from_iter<T: IntoIterator<Item = (TextRange, OptionsIncomplete)>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
```

## Git History

- Commits: 17
- Authors: 14
- Bug fixes: 5
- Features: 4

## Callers

None

## Callees

None
