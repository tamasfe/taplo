# mod.insert_token

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 198-200)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn insert_token(&mut self, kind: SyntaxKind, s: &str) {
        self.builder.token(kind.into(), s)
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn insert_token(&mut self, kind: SyntaxKind, s: &str) {
        self.builder.token(kind.into(), s)
    }
```

## Git History

- Commits: 8
- Authors: 5
- Bug fixes: 4
- Features: 3

## Callers

None

## Callees

None
