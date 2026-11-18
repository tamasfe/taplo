# mod.is_array_multiline

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 948-950)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn is_array_multiline(node: &SyntaxNode) -> bool {
    node.descendants_with_tokens().any(|n| n.kind() == NEWLINE)
}
```

## Docstring

_No docstring provided._

## Source

```
fn is_array_multiline(node: &SyntaxNode) -> bool {
    node.descendants_with_tokens().any(|n| n.kind() == NEWLINE)
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
