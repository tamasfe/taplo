# mod.eq

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 388-390)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn eq(&self, other: &Self) -> bool {
        self.cleaned_key().eq(other.cleaned_key())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn eq(&self, other: &Self) -> bool {
        self.cleaned_key().eq(other.cleaned_key())
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
