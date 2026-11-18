# syntax.from

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\syntax.rs` (lines 107-109)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
```

## Git History

- Commits: 5
- Authors: 3
- Bug fixes: 2
- Features: 1

## Callers

None

## Callees

None
