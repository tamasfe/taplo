# index.index_into

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\index.rs` (lines 21-28)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn index_into(&self, v: &Node) -> Option<Node> {
        if let Node::Array(arr) = v {
            let items = arr.items().read();
            items.get(*self).cloned()
        } else {
            None
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn index_into(&self, v: &Node) -> Option<Node> {
        if let Node::Array(arr) = v {
            let items = arr.items().read();
            items.get(*self).cloned()
        } else {
            None
        }
    }
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
