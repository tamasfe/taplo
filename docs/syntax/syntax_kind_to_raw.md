# syntax.kind_to_raw

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\syntax.rs` (lines 120-122)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
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
