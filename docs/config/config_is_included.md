# config.is_included

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\config.rs` (lines 97-105)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_included(&self, path: &Path) -> bool {
        match &self.file_rule {
            Some(r) => r.is_match(path),
            None => {
                tracing::debug!("no file matches were set up");
                false
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_included(&self, path: &Path) -> bool {
        match &self.file_rule {
            Some(r) => r.is_match(path),
            None => {
                tracing::debug!("no file matches were set up");
                false
            }
        }
    }
```

## Git History

- Commits: 6
- Authors: 3
- Bug fixes: 4
- Features: 1

## Callers

None

## Callees

None
