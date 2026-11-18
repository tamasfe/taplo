# mod.whitelisted

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 194-196)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn whitelisted(&self, token: SyntaxKind) -> bool {
        self.error_whitelist & token as u16 != 0
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn whitelisted(&self, token: SyntaxKind) -> bool {
        self.error_whitelist & token as u16 != 0
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
