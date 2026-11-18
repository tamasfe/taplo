# mod.strip_quotes

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\mod.rs` (lines 111-117)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn strip_quotes(self) -> Self {
        if self.starts_with('\"') || self.starts_with('\'') {
            &self[1..self.len() - 1]
        } else {
            self
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn strip_quotes(self) -> Self {
        if self.starts_with('\"') || self.starts_with('\'') {
            &self[1..self.len() - 1]
        } else {
            self
        }
    }
```

## Git History

- Commits: 4
- Authors: 2
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
