# query.lookup_keys

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 433-446)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn lookup_keys(root: Node, keys: &Keys) -> Keys {
    let mut node = root;
    let mut new_keys = Keys::empty();

    for key in keys.iter().cloned() {
        node = node.get(&key);
        new_keys = new_keys.join(key);
        if let Some(arr) = node.as_array() {
            new_keys = new_keys.join(arr.items().read().len().saturating_sub(1));
        }
    }

    new_keys
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn lookup_keys(root: Node, keys: &Keys) -> Keys {
    let mut node = root;
    let mut new_keys = Keys::empty();

    for key in keys.iter().cloned() {
        node = node.get(&key);
        new_keys = new_keys.join(key);
        if let Some(arr) = node.as_array() {
            new_keys = new_keys.join(arr.items().read().len().saturating_sub(1));
        }
    }

    new_keys
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
