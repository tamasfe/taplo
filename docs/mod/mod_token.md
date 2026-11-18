# mod.token

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 236-241)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn token(&mut self) -> ParserResult<()> {
        match self.get_token() {
            Err(_) => Err(()),
            Ok(token) => self.token_as(token),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn token(&mut self) -> ParserResult<()> {
        match self.get_token() {
            Err(_) => Err(()),
            Ok(token) => self.token_as(token),
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
