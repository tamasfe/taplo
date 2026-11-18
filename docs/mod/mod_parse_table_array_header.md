# mod.parse_table_array_header

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 400-425)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn parse_table_array_header(&mut self) -> ParserResult<()> {
        self.skip_whitespace = false;
        self.must_token_or(BRACKET_START, r#"expected "[[""#)?;
        self.must_token_or(BRACKET_START, r#"expected "[[""#)?;
        self.skip_whitespace = true;
        let _ = with_node!(self.builder, KEY, self.parse_key());
        self.skip_whitespace = false;
        let _ = self.must_token_or(BRACKET_END, r#"expected "]]""#);

        // Hack in order to avoid calling `step` after
        // the second closing bracket.
        let token = self.get_token()?;
        match token {
            BRACKET_END => {
                self.token_as_no_step(token)?;
            }
            _ => {
                self.error(r#"expected "]]"#)?;
            }
        }
        self.skip_whitespace = true;

        self.step();

        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn parse_table_array_header(&mut self) -> ParserResult<()> {
        self.skip_whitespace = false;
        self.must_token_or(BRACKET_START, r#"expected "[[""#)?;
        self.must_token_or(BRACKET_START, r#"expected "[[""#)?;
        self.skip_whitespace = true;
        let _ = with_node!(self.builder, KEY, self.parse_key());
        self.skip_whitespace = false;
        let _ = self.must_token_or(BRACKET_END, r#"expected "]]""#);

        // Hack in order to avoid calling `step` after
        // the second closing bracket.
        let token = self.get_token()?;
        match token {
            BRACKET_END => {
                self.token_as_no_step(token)?;
            }
            _ => {
                self.error(r#"expected "]]"#)?;
            }
        }
        self.skip_whitespace = true;

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
