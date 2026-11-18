# node.path

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 84-95)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn path(&self, keys: &Keys) -> Option<Node> {
        let mut node = self.clone();
        for key in keys.iter() {
            node = node.get(key);
        }

        if node.is_invalid() {
            None
        } else {
            Some(node)
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn path(&self, keys: &Keys) -> Option<Node> {
        let mut node = self.clone();
        for key in keys.iter() {
            node = node.get(key);
        }

        if node.is_invalid() {
            None
        } else {
            Some(node)
        }
    }
```

## Git History

- Commits: 8
- Authors: 6
- Bug fixes: 2
- Features: 1

## Callers

None

## Callees

None
