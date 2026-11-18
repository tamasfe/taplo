# mod.parse_inline_table

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 789-830)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn parse_inline_table(&mut self) -> ParserResult<()> {
        // https://github.com/toml-lang/toml/blob/78fcf9dd7eab7acfbaf147c684b649477e7bdd9c/toml.abnf#L238
        self.must_token_or(BRACE_START, r#"expected "{""#)?;

        let mut expect_comma_or_end = false;

        loop {
            let t = match self.get_token() {
                Ok(t) => t,
                Err(_) => return self.report_error(r#"expected "}""#),
            };

            match t {
                BRACE_END => {
                    self.add_token()?;
                    break;
                }
                WHITESPACE | NEWLINE | COMMENT => {
                    self.add_token()?;
                }
                COMMA => {
                    if !expect_comma_or_end {
                        let _ = self.error(r#"unexpected ",""#);
                    }
                    self.add_token()?;
                    expect_comma_or_end = false;
                }
                _ => {
                    if expect_comma_or_end {
                        let _ = self.error(r#"expected "," or "}""#);
                    }
                    let _ = whitelisted!(
                        self,
                        COMMA,
                        with_node!(self.builder, ENTRY, self.parse_entry())
                    );
                    expect_comma_or_end = true;
                }
            }
        }
        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn parse_inline_table(&mut self) -> ParserResult<()> {
        // https://github.com/toml-lang/toml/blob/78fcf9dd7eab7acfbaf147c684b649477e7bdd9c/toml.abnf#L238
        self.must_token_or(BRACE_START, r#"expected "{""#)?;

        let mut expect_comma_or_end = false;

        loop {
            let t = match self.get_token() {
                Ok(t) => t,
                Err(_) => return self.report_error(r#"expected "}""#),
            };

            match t {
                BRACE_END => {
                    self.add_token()?;
                    break;
                }
                WHITESPACE | NEWLINE | COMMENT => {
                    self.add_token()?;
                }
                COMMA => {
                    if !expect_comma_or_end {
                        let _ = self.error(r#"unexpected ",""#);
                    }
                    self.add_token()?;
                    expect_comma_or_end = false;
                }
                _ => {
                    if expect_comma_or_end {
                        let _ = self.error(r#"expected "," or "}""#);
                    }
                    let _ = whitelisted!(
                        self,
                        COMMA,
                        with_node!(self.builder, ENTRY, self.parse_entry())
                    );
                    expect_comma_or_end = true;
                }
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
