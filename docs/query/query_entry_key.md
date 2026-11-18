# query.entry_key

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 182-194)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn entry_key(&self) -> Option<SyntaxNode> {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return None,
        };

        let keys = syntax
            .parent_ancestors()
            .find(|n| n.kind() == ENTRY)
            .and_then(|entry| entry.children().find(|c| c.kind() == KEY))?;

        Some(keys)
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn entry_key(&self) -> Option<SyntaxNode> {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return None,
        };

        let keys = syntax
            .parent_ancestors()
            .find(|n| n.kind() == ENTRY)
            .and_then(|entry| entry.children().find(|c| c.kind() == KEY))?;

        Some(keys)
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
