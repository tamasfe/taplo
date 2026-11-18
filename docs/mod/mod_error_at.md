# mod.error_at

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 227-235)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn error_at(&self, range: TextRange) -> bool {
        for error_range in self.errors.iter().copied() {
            if overlaps(range, error_range) {
                return true;
            }
        }

        false
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn error_at(&self, range: TextRange) -> bool {
        for error_range in self.errors.iter().copied() {
            if overlaps(range, error_range) {
                return true;
            }
        }

        false
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
