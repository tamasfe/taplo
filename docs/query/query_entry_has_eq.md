# query.entry_has_eq

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 292-305)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn entry_has_eq(&self) -> bool {
        let Some(key_syntax) = self.entry_key() else {
            return false;
        };

        key_syntax
            .siblings(Direction::Next)
            .find_map(|s| match s.kind() {
                EQ => Some(true),
                WHITESPACE => None,
                _ => Some(false),
            })
            .unwrap_or(false)
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn entry_has_eq(&self) -> bool {
        let Some(key_syntax) = self.entry_key() else {
            return false;
        };

        key_syntax
            .siblings(Direction::Next)
            .find_map(|s| match s.kind() {
                EQ => Some(true),
                WHITESPACE => None,
                _ => Some(false),
            })
            .unwrap_or(false)
    }
```

## Git History

- Commits: 6
- Authors: 3
- Bug fixes: 1
- Features: 2

## Callers

None

## Callees

None
