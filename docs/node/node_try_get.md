# node.try_get

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 109-112)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn try_get(&self, idx: impl Index) -> Result<Node, Error> {
        idx.index_into(self)
            .ok_or(Error::Query(QueryError::NotFound))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn try_get(&self, idx: impl Index) -> Result<Node, Error> {
        idx.index_into(self)
            .ok_or(Error::Query(QueryError::NotFound))
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
