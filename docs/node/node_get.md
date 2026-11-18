# node.get

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 97-107)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn get(&self, idx: impl Index) -> Node {
        idx.index_into(self).unwrap_or_else(|| {
            Node::from(
                InvalidInner {
                    errors: Shared::from(Vec::from([Error::Query(QueryError::NotFound)])),
                    syntax: None,
                }
                .wrap(),
            )
        })
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn get(&self, idx: impl Index) -> Node {
        idx.index_into(self).unwrap_or_else(|| {
            Node::from(
                InvalidInner {
                    errors: Shared::from(Vec::from([Error::Query(QueryError::NotFound)])),
                    syntax: None,
                }
                .wrap(),
            )
        })
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
