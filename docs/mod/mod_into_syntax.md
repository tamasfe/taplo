# mod.into_syntax

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 909-911)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn into_syntax(self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node)
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn into_syntax(self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node)
    }
```

## Git History

- Commits: 8
- Authors: 5
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
