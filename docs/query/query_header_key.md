# query.header_key

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 167-179)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn header_key(&self) -> Option<SyntaxNode> {
        match (&self.before, &self.after) {
            (Some(before), _) => {
                let header_syntax = before
                    .syntax
                    .parent_ancestors()
                    .find(|s| matches!(s.kind(), TABLE_ARRAY_HEADER | TABLE_HEADER))?;

                header_syntax.descendants().find(|n| n.kind() == KEY)
            }
            _ => None,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn header_key(&self) -> Option<SyntaxNode> {
        match (&self.before, &self.after) {
            (Some(before), _) => {
                let header_syntax = before
                    .syntax
                    .parent_ancestors()
                    .find(|s| matches!(s.kind(), TABLE_ARRAY_HEADER | TABLE_HEADER))?;

                header_syntax.descendants().find(|n| n.kind() == KEY)
            }
            _ => None,
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
