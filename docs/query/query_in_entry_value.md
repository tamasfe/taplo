# query.in_entry_value

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 308-331)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn in_entry_value(&self) -> bool {
        let in_value = self
            .entry_value()
            // We are inside the value even if the cursor is right after it.
            .is_some_and(|k| k.text_range().contains_inclusive(self.offset));

        if in_value {
            return true;
        }

        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return false,
        };

        syntax
            .siblings_with_tokens(Direction::Prev)
            .find_map(|s| match s.kind() {
                EQ => Some(true),
                WHITESPACE | COMMENT | NEWLINE => None,
                _ => Some(false),
            })
            .unwrap_or(false)
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn in_entry_value(&self) -> bool {
        let in_value = self
            .entry_value()
            // We are inside the value even if the cursor is right after it.
            .is_some_and(|k| k.text_range().contains_inclusive(self.offset));

        if in_value {
            return true;
        }

        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return false,
        };

        syntax
            .siblings_with_tokens(Direction::Prev)
            .find_map(|s| match s.kind() {
                EQ => Some(true),
                WHITESPACE | COMMENT | NEWLINE => None,
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
