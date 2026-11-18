# query.in_array

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 382-407)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn in_array(&self) -> bool {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return false,
        };

        match syntax.parent() {
            Some(parent) => {
                if parent.kind() != ARRAY {
                    return false;
                }

                parent
                    .children_with_tokens()
                    .find_map(|t| {
                        if t.kind() == BRACKET_END {
                            Some(self.offset <= t.text_range().start())
                        } else {
                            None
                        }
                    })
                    .unwrap_or(true)
            }
            None => false,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn in_array(&self) -> bool {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return false,
        };

        match syntax.parent() {
            Some(parent) => {
                if parent.kind() != ARRAY {
                    return false;
                }

                parent
                    .children_with_tokens()
                    .find_map(|t| {
                        if t.kind() == BRACKET_END {
                            Some(self.offset <= t.text_range().start())
                        } else {
                            None
                        }
                    })
                    .unwrap_or(true)
            }
            None => false,
        }
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
