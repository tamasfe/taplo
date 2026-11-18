# mod.cmp

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 402-404)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.cleaned_key().cmp(other.cleaned_key())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.cleaned_key().cmp(other.cleaned_key())
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
