# query.in_table_array_header

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 118-164)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn in_table_array_header(&self) -> bool {
        match (&self.before, &self.after) {
            (Some(before), Some(after)) => {
                let Some(header_syntax) = before
                    .syntax
                    .parent_ancestors()
                    .find(|s| s.kind() == TABLE_ARRAY_HEADER)
                else {
                    return false;
                };

                if !after.syntax.parent_ancestors().any(|a| a == header_syntax) {
                    return false;
                }

                let Some(bracket_start) = header_syntax
                    .children_with_tokens()
                    .filter_map(|t| {
                        if t.kind() == BRACKET_START {
                            t.into_token()
                        } else {
                            None
                        }
                    })
                    .nth(1)
                else {
                    return false;
                };

                let Some(bracket_end) = header_syntax.children_with_tokens().find_map(|t| {
                    if t.kind() == BRACKET_END {
                        t.into_token()
                    } else {
                        None
                    }
                }) else {
                    return false;
                };

                (before.syntax == bracket_start
                    || before.syntax.text_range().start() >= bracket_start.text_range().end())
                    && (after.syntax == bracket_end
                        || after.syntax.text_range().end() <= bracket_end.text_range().start())
            }
            _ => false,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn in_table_array_header(&self) -> bool {
        match (&self.before, &self.after) {
            (Some(before), Some(after)) => {
                let Some(header_syntax) = before
                    .syntax
                    .parent_ancestors()
                    .find(|s| s.kind() == TABLE_ARRAY_HEADER)
                else {
                    return false;
                };

                if !after.syntax.parent_ancestors().any(|a| a == header_syntax) {
                    return false;
                }

                let Some(bracket_start) = header_syntax
                    .children_with_tokens()
                    .filter_map(|t| {
                        if t.kind() == BRACKET_START {
                            t.into_token()
                        } else {
                            None
                        }
                    })
                    .nth(1)
                else {
                    return false;
                };

                let Some(bracket_end) = header_syntax.children_with_tokens().find_map(|t| {
                    if t.kind() == BRACKET_END {
                        t.into_token()
                    } else {
                        None
                    }
                }) else {
                    return false;
                };

                (before.syntax == bracket_start
                    || before.syntax.text_range().start() >= bracket_start.text_range().end())
                    && (after.syntax == bracket_end
                        || after.syntax.text_range().end() <= bracket_end.text_range().start())
            }
            _ => false,
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
