# mod.find_node_deep

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\mod.rs` (lines 126-138)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
