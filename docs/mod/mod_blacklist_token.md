# mod.blacklist_token

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 189-191)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn blacklist_token(&mut self, token: SyntaxKind) {
        self.error_whitelist &= !(token as u16);
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn blacklist_token(&mut self, token: SyntaxKind) {
        self.error_whitelist &= !(token as u16);
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
