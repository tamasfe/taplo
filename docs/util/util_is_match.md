# util.is_match

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\util.rs` (lines 38-44)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_match(&self, text: impl AsRef<Path>) -> bool {
        if !self.include.is_match(text.as_ref()) {
            return false;
        }

        !self.exclude.is_match(text.as_ref())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_match(&self, text: impl AsRef<Path>) -> bool {
        if !self.include.is_match(text.as_ref()) {
            return false;
        }

        !self.exclude.is_match(text.as_ref())
    }
```

## Git History

- Commits: 11
- Authors: 4
- Bug fixes: 7
- Features: 4

## Callers

None

## Callees

None
