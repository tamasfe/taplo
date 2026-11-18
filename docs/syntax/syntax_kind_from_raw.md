# syntax.kind_from_raw

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\syntax.rs` (lines 116-119)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
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
