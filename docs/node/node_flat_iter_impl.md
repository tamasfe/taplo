# node.flat_iter_impl

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 328-348)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn flat_iter_impl(&self) -> Vec<(Keys, Node)> {
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

        all
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn flat_iter_impl(&self) -> Vec<(Keys, Node)> {
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

        all
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
