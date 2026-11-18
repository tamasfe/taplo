# mod.find

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\mod.rs` (lines 157-159)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn find(&self, kind: SyntaxKind) -> Option<SyntaxElement> {
        self.descendants_with_tokens().find(|d| d.kind() == kind)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn find(&self, kind: SyntaxKind) -> Option<SyntaxElement> {
        self.descendants_with_tokens().find(|d| d.kind() == kind)
    }
```

## Git History

- Commits: 4
- Authors: 2
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
