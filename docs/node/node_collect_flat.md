# node.collect_flat

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 350-370)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn collect_flat(&self, parent: Keys, all: &mut Vec<(Keys, Node)>) {
        match self {
            Node::Table(t) => {
                all.push((parent.clone(), self.clone()));
                let entries = t.inner.entries.read();
                for (key, entry) in &entries.all {
                    entry.collect_flat(parent.join(key.clone()), all);
                }
            }
            Node::Array(arr) => {
                all.push((parent.clone(), self.clone()));
                let items = arr.inner.items.read();
                for (idx, item) in items.iter().enumerate() {
                    item.collect_flat(parent.join(idx), all);
                }
            }
            _ => {
                all.push((parent, self.clone()));
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn collect_flat(&self, parent: Keys, all: &mut Vec<(Keys, Node)>) {
        match self {
            Node::Table(t) => {
                all.push((parent.clone(), self.clone()));
                let entries = t.inner.entries.read();
                for (key, entry) in &entries.all {
                    entry.collect_flat(parent.join(key.clone()), all);
                }
            }
            Node::Array(arr) => {
                all.push((parent.clone(), self.clone()));
                let items = arr.inner.items.read();
                for (idx, item) in items.iter().enumerate() {
                    item.collect_flat(parent.join(idx), all);
                }
            }
            _ => {
                all.push((parent, self.clone()));
            }
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
