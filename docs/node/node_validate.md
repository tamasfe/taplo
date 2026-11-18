# node.validate

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 147-155)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn validate(&self) -> Result<(), impl Iterator<Item = Error> + core::fmt::Debug> {
        let mut errors = Vec::new();
        self.validate_all_impl(&mut errors);
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.into_iter())
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn validate(&self) -> Result<(), impl Iterator<Item = Error> + core::fmt::Debug> {
        let mut errors = Vec::new();
        self.validate_all_impl(&mut errors);
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.into_iter())
        }
    }
```

## Git History

- Commits: 8
- Authors: 6
- Bug fixes: 2
- Features: 1

## Callers

None

## Callees

None
