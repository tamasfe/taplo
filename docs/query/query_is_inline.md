# query.is_inline

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 342-351)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_inline(&self) -> bool {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return false,
        };

        syntax
            .parent_ancestors()
            .any(|a| matches!(a.kind(), INLINE_TABLE | ARRAY))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_inline(&self) -> bool {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return false,
        };

        syntax
            .parent_ancestors()
            .any(|a| matches!(a.kind(), INLINE_TABLE | ARRAY))
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
