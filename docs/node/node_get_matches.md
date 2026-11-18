# node.get_matches

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 114-144)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn get_matches(
        &self,
        pattern: &str,
    ) -> Result<impl ExactSizeIterator<Item = (KeyOrIndex, Node)>, Error> {
        let glob = globset::Glob::new(pattern)
            .map_err(QueryError::from)?
            .compile_matcher();
        let mut matched = Vec::new();

        match self {
            Node::Table(t) => {
                let entries = t.entries().read();
                for (key, node) in entries.iter() {
                    if glob.is_match(key.value()) {
                        matched.push((KeyOrIndex::from(key.clone()), node.clone()));
                    }
                }
            }
            Node::Array(arr) => {
                let items = arr.items().read();
                for (idx, node) in items.iter().enumerate() {
                    if glob.is_match(idx.to_string()) {
                        matched.push((KeyOrIndex::from(idx), node.clone()));
                    }
                }
            }
            _ => {}
        }

        Ok(matched.into_iter())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn get_matches(
        &self,
        pattern: &str,
    ) -> Result<impl ExactSizeIterator<Item = (KeyOrIndex, Node)>, Error> {
        let glob = globset::Glob::new(pattern)
            .map_err(QueryError::from)?
            .compile_matcher();
        let mut matched = Vec::new();

        match self {
            Node::Table(t) => {
                let entries = t.entries().read();
                for (key, node) in entries.iter() {
                    if glob.is_match(key.value()) {
                        matched.push((KeyOrIndex::from(key.clone()), node.clone()));
                    }
                }
            }
            Node::Array(arr) => {
                let items = arr.items().read();
                for (idx, node) in items.iter().enumerate() {
                    if glob.is_match(idx.to_string()) {
                        matched.push((KeyOrIndex::from(idx), node.clone()));
                    }
                }
            }
            _ => {}
        }

        Ok(matched.into_iter())
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
