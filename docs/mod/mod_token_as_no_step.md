# mod.token_as_no_step

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 253-262)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn token_as_no_step(&mut self, kind: SyntaxKind) -> ParserResult<()> {
        match self.get_token() {
            Err(_) => return Err(()),
            Ok(_) => {
                self.builder.token(kind.into(), self.lexer.slice());
            }
        }

        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn token_as_no_step(&mut self, kind: SyntaxKind) -> ParserResult<()> {
        match self.get_token() {
            Err(_) => return Err(()),
            Ok(_) => {
                self.builder.token(kind.into(), self.lexer.slice());
            }
        }

        Ok(())
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
