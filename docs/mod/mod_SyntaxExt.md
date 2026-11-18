# mod.SyntaxExt

- **Type:** trait
- **Language:** rust
- **File:** `\crates\taplo\src\util\mod.rs` (lines 121-142)
- **Role:** protocol
- **Stability:** unknown

## Signature

```
pub trait SyntaxExt {
    /// Return a syntax node that contains the given offset.
    fn find_node(&self, offset: TextSize, inclusive: bool) -> Option<SyntaxNode>;

    /// Find the deepest node that contains the given offset.
    fn find_node_deep(&self, offset: TextSize, inclusive: bool) -> Option<SyntaxNode> {
        let mut node = self.find_node(offset, inclusive);
        while let Some(n) = &node {
            let new_node = n.find_node(offset, inclusive);
            if new_node.is_some() {
                node = new_node;
            } else {
                break;
            }
        }

        node
    }

    /// Find a node or token by its kind.
    fn find(&self, kind: SyntaxKind) -> Option<SyntaxElement>;
}
```

## Docstring

_No docstring provided._

## Source

```
pub trait SyntaxExt {
    /// Return a syntax node that contains the given offset.
    fn find_node(&self, offset: TextSize, inclusive: bool) -> Option<SyntaxNode>;

    /// Find the deepest node that contains the given offset.
    fn find_node_deep(&self, offset: TextSize, inclusive: bool) -> Option<SyntaxNode> {
        let mut node = self.find_node(offset, inclusive);
        while let Some(n) = &node {
            let new_node = n.find_node(offset, inclusive);
            if new_node.is_some() {
                node = new_node;
            } else {
                break;
            }
        }

        node
    }

    /// Find a node or token by its kind.
    fn find(&self, kind: SyntaxKind) -> Option<SyntaxElement>;
}
```

## Git History

- Commits: 4
- Authors: 2
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
