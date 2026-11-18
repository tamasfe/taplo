# query.Query

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 14-21)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct Query {
    /// The offset the query was made for.
    pub offset: TextSize,
    /// Before the cursor.
    pub before: Option<PositionInfo>,
    /// After the cursor.
    pub after: Option<PositionInfo>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct Query {
    /// The offset the query was made for.
    pub offset: TextSize,
    /// Before the cursor.
    pub before: Option<PositionInfo>,
    /// After the cursor.
    pub after: Option<PositionInfo>,
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
