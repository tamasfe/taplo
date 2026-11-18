# query.empty_line

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 246-283)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn empty_line(&self) -> bool {
        let before_syntax = match self.before.as_ref() {
            Some(s) => &s.syntax,
            None => return true,
        };

        match &self.after {
            Some(after) => {
                if matches!(after.syntax.kind(), WHITESPACE | NEWLINE) {
                    let new_line_after = after
                        .syntax
                        .siblings_with_tokens(Direction::Next)
                        .find_map(|s| match s.kind() {
                            NEWLINE => Some(true),
                            WHITESPACE | COMMENT => None,
                            _ => Some(false),
                        })
                        .unwrap_or(true);

                    if !new_line_after {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            None => {}
        }

        before_syntax
            .siblings_with_tokens(Direction::Prev)
            .find_map(|s| match s.kind() {
                NEWLINE => Some(true),
                WHITESPACE | COMMENT => None,
                _ => Some(false),
            })
            .unwrap_or(true)
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn empty_line(&self) -> bool {
        let before_syntax = match self.before.as_ref() {
            Some(s) => &s.syntax,
            None => return true,
        };

        match &self.after {
            Some(after) => {
                if matches!(after.syntax.kind(), WHITESPACE | NEWLINE) {
                    let new_line_after = after
                        .syntax
                        .siblings_with_tokens(Direction::Next)
                        .find_map(|s| match s.kind() {
                            NEWLINE => Some(true),
                            WHITESPACE | COMMENT => None,
                            _ => Some(false),
                        })
                        .unwrap_or(true);

                    if !new_line_after {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            None => {}
        }

        before_syntax
            .siblings_with_tokens(Direction::Prev)
            .find_map(|s| match s.kind() {
                NEWLINE => Some(true),
                WHITESPACE | COMMENT => None,
                _ => Some(false),
            })
            .unwrap_or(true)
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
