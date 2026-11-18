# mod.can_collapse_array

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 952-954)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn can_collapse_array(node: &SyntaxNode) -> bool {
    !node.descendants_with_tokens().any(|n| n.kind() == COMMENT)
}
```

## Docstring

_No docstring provided._

## Source

```
fn can_collapse_array(node: &SyntaxNode) -> bool {
    !node.descendants_with_tokens().any(|n| n.kind() == COMMENT)
}
```

## Git History

- Commits: 17
- Authors: 14
- Bug fixes: 5
- Features: 4

## Callers

None

## Callees

None
