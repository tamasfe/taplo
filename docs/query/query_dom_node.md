# query.dom_node

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 420-425)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn dom_node(&self) -> Option<&(Keys, Node)> {
        self.before
            .as_ref()
            .and_then(|p| p.dom_node.as_ref())
            .or_else(|| self.after.as_ref().and_then(|p| p.dom_node.as_ref()))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn dom_node(&self) -> Option<&(Keys, Node)> {
        self.before
            .as_ref()
            .and_then(|p| p.dom_node.as_ref())
            .or_else(|| self.after.as_ref().and_then(|p| p.dom_node.as_ref()))
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
