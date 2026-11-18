# mod.parse_table_header

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 392-398)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn parse_table_header(&mut self) -> ParserResult<()> {
        self.must_token_or(BRACKET_START, r#"expected "[""#)?;
        let _ = with_node!(self.builder, KEY, self.parse_key());
        self.must_token_or(BRACKET_END, r#"expected "]""#)?;

        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn parse_table_header(&mut self) -> ParserResult<()> {
        self.must_token_or(BRACKET_START, r#"expected "[""#)?;
        let _ = with_node!(self.builder, KEY, self.parse_key());
        self.must_token_or(BRACKET_END, r#"expected "]""#)?;

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
