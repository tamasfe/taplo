# node.comments

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 297-308)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn comments(&self) -> impl Iterator<Item = Comment> {
        if let Some(syntax) = self.syntax().cloned().and_then(|s| s.into_node()) {
            Either::Left(
                syntax
                    .descendants_with_tokens()
                    .filter(|t| t.kind() == SyntaxKind::COMMENT)
                    .map(Comment::from_syntax),
            )
        } else {
            Either::Right(empty())
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn comments(&self) -> impl Iterator<Item = Comment> {
        if let Some(syntax) = self.syntax().cloned().and_then(|s| s.into_node()) {
            Either::Left(
                syntax
                    .descendants_with_tokens()
                    .filter(|t| t.kind() == SyntaxKind::COMMENT)
                    .map(Comment::from_syntax),
            )
        } else {
            Either::Right(empty())
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
