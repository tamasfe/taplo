# mod.token_as

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 247-251)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn token_as(&mut self, kind: SyntaxKind) -> ParserResult<()> {
        self.token_as_no_step(kind)?;
        self.step();
        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn token_as(&mut self, kind: SyntaxKind) -> ParserResult<()> {
        self.token_as_no_step(kind)?;
        self.step();
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
