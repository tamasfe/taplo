# query.in_inline_table

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 354-379)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn in_inline_table(&self) -> bool {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return false,
        };

        match syntax.parent() {
            Some(parent) => {
                if parent.kind() != INLINE_TABLE {
                    return false;
                }

                parent
                    .children_with_tokens()
                    .find_map(|t| {
                        if t.kind() == BRACE_END {
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
    pub fn in_inline_table(&self) -> bool {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return false,
        };

        match syntax.parent() {
            Some(parent) => {
                if parent.kind() != INLINE_TABLE {
                    return false;
                }

                parent
                    .children_with_tokens()
                    .find_map(|t| {
                        if t.kind() == BRACE_END {
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
