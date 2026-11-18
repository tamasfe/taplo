# node.flat_iter

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 157-177)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn flat_iter(&self) -> impl DoubleEndedIterator<Item = (Keys, Node)> {
        let mut all = Vec::new();

        match self {
            Node::Table(t) => {
                let entries = t.inner.entries.read();
                for (key, entry) in &entries.all {
                    entry.collect_flat(Keys::from(key.clone()), &mut all);
                }
            }
            Node::Array(arr) => {
                let items = arr.inner.items.read();
                for (idx, item) in items.iter().enumerate() {
                    item.collect_flat(Keys::from(idx), &mut all);
                }
            }
            _ => {}
        }

        all.into_iter()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn flat_iter(&self) -> impl DoubleEndedIterator<Item = (Keys, Node)> {
        let mut all = Vec::new();

        match self {
            Node::Table(t) => {
                let entries = t.inner.entries.read();
                for (key, entry) in &entries.all {
                    entry.collect_flat(Keys::from(key.clone()), &mut all);
                }
            }
            Node::Array(arr) => {
                let items = arr.inner.items.read();
                for (idx, item) in items.iter().enumerate() {
                    item.collect_flat(Keys::from(idx), &mut all);
                }
            }
            _ => {}
        }

        all.into_iter()
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
