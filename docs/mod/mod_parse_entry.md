# mod.parse_entry

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 427-433)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn parse_entry(&mut self) -> ParserResult<()> {
        with_node!(self.builder, KEY, self.parse_key())?;
        self.must_token_or(EQ, r#"expected "=""#)?;
        with_node!(self.builder, VALUE, self.parse_value())?;

        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn parse_entry(&mut self) -> ParserResult<()> {
        with_node!(self.builder, KEY, self.parse_key())?;
        self.must_token_or(EQ, r#"expected "=""#)?;
        with_node!(self.builder, VALUE, self.parse_value())?;

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
