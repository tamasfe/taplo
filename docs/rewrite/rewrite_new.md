# rewrite.new

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\rewrite.rs` (lines 17-26)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn new(root: Node) -> Result<Self, Error> {
        if !matches!(root.syntax().map(|s| s.kind()), Some(SyntaxKind::ROOT)) {
            return Err(Error::RootNodeExpected);
        }

        Ok(Self {
            root,
            patches: Default::default(),
        })
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn new(root: Node) -> Result<Self, Error> {
        if !matches!(root.syntax().map(|s| s.kind()), Some(SyntaxKind::ROOT)) {
            return Err(Error::RootNodeExpected);
        }

        Ok(Self {
            root,
            patches: Default::default(),
        })
    }
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
