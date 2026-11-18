# mod.get_token

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 313-319)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn get_token(&mut self) -> ParserResult<SyntaxKind> {
        if self.current_token.is_none() {
            self.step();
        }

        self.current_token.ok_or(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn get_token(&mut self) -> ParserResult<SyntaxKind> {
        if self.current_token.is_none() {
            self.step();
        }

        self.current_token.ok_or(())
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
