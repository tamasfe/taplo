# query.PositionInfo

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 449-454)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct PositionInfo {
    /// The narrowest syntax element that contains the position.
    pub syntax: SyntaxToken,
    /// The narrowest node that covers the position.
    pub dom_node: Option<(Keys, Node)>,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct PositionInfo {
    /// The narrowest syntax element that contains the position.
    pub syntax: SyntaxToken,
    /// The narrowest node that covers the position.
    pub dom_node: Option<(Keys, Node)>,
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
