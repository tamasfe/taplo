# mod.parse_array

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 832-875)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn parse_array(&mut self) -> ParserResult<()> {
        self.must_token_or(BRACKET_START, r#"expected "[""#)?;

        let mut first = true;
        let mut comma_last = false;
        loop {
            let t = match self.get_token() {
                Ok(t) => t,
                Err(_) => {
                    let _ = self.report_error("unexpected EOF");
                    return Err(());
                }
            };

            match t {
                BRACKET_END => break self.add_token()?,
                NEWLINE => {
                    self.token()?;
                    continue; // as if it wasn't there, so it doesn't count as a first token
                }
                COMMA => {
                    if first || comma_last {
                        let _ = self.error(r#"unexpected ",""#);
                    }
                    self.token()?;
                    comma_last = true;
                }
                _ => {
                    if !comma_last && !first {
                        let _ = self.error(r#"expected ",""#);
                    }
                    let _ = whitelisted!(
                        self,
                        COMMA,
                        with_node!(self.builder, VALUE, self.parse_value())
                    );
                    comma_last = false;
                }
            }

            first = false;
        }
        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn parse_array(&mut self) -> ParserResult<()> {
        self.must_token_or(BRACKET_START, r#"expected "[""#)?;

        let mut first = true;
        let mut comma_last = false;
        loop {
            let t = match self.get_token() {
                Ok(t) => t,
                Err(_) => {
                    let _ = self.report_error("unexpected EOF");
                    return Err(());
                }
            };

            match t {
                BRACKET_END => break self.add_token()?,
                NEWLINE => {
                    self.token()?;
                    continue; // as if it wasn't there, so it doesn't count as a first token
                }
                COMMA => {
                    if first || comma_last {
                        let _ = self.error(r#"unexpected ",""#);
                    }
                    self.token()?;
                    comma_last = true;
                }
                _ => {
                    if !comma_last && !first {
                        let _ = self.error(r#"expected ",""#);
                    }
                    let _ = whitelisted!(
                        self,
                        COMMA,
                        with_node!(self.builder, VALUE, self.parse_value())
                    );
                    comma_last = false;
                }
            }

            first = false;
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
