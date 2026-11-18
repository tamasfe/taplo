# mod.add_token

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 225-234)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn add_token(&mut self) -> ParserResult<()> {
        match self.get_token() {
            Err(_) => Err(()),
            Ok(token) => {
                self.builder.token(token.into(), self.lexer.slice());
                self.current_token = None;
                Ok(())
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn add_token(&mut self) -> ParserResult<()> {
        match self.get_token() {
            Err(_) => Err(()),
            Ok(token) => {
                self.builder.token(token.into(), self.lexer.slice());
                self.current_token = None;
                Ok(())
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
