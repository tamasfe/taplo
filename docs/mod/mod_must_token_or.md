# mod.must_token_or

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 202-222)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn must_token_or(&mut self, kind: SyntaxKind, message: &str) -> ParserResult<()> {
        match self.get_token() {
            Ok(t) => {
                if kind == t {
                    self.token()
                } else {
                    self.error(message)
                }
            }
            Err(_) => {
                self.add_error(&Error {
                    range: TextRange::new(
                        self.lexer.span().start.try_into().unwrap(),
                        self.lexer.span().end.try_into().unwrap(),
                    ),
                    message: "unexpected EOF".into(),
                });
                Err(())
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn must_token_or(&mut self, kind: SyntaxKind, message: &str) -> ParserResult<()> {
        match self.get_token() {
            Ok(t) => {
                if kind == t {
                    self.token()
                } else {
                    self.error(message)
                }
            }
            Err(_) => {
                self.add_error(&Error {
                    range: TextRange::new(
                        self.lexer.span().start.try_into().unwrap(),
                        self.lexer.span().end.try_into().unwrap(),
                    ),
                    message: "unexpected EOF".into(),
                });
                Err(())
            }
        }
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
