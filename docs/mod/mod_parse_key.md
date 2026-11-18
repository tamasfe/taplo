# mod.parse_key

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 435-494)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn parse_key(&mut self) -> ParserResult<()> {
        if self.parse_ident().is_err() {
            return self.report_error("expected identifier");
        }

        let mut after_period = false;
        loop {
            let t = match self.get_token() {
                Ok(token) => token,
                Err(_) => {
                    if !after_period {
                        return Ok(());
                    }
                    return self.error("unexpected end of input");
                }
            };

            match t {
                PERIOD => {
                    if after_period {
                        return self.error(r#"unexpected ".""#);
                    } else {
                        self.token()?;
                        after_period = true;
                    }
                }
                BRACKET_START if self.key_pattern_syntax => {
                    self.step();

                    match self.parse_ident() {
                        Ok(_) => {}
                        Err(_) => return self.error("expected identifier"),
                    }

                    let token = self.get_token()?;

                    if !matches!(token, BRACKET_END) {
                        self.error(r#"expected "]""#)?;
                    }
                    self.step();
                    after_period = false;
                }
                _ => {
                    if after_period {
                        match self.parse_ident() {
                            Ok(_) => {}
                            Err(_) => return self.report_error("expected identifier"),
                        }
                        after_period = false;
                    } else if self.key_pattern_syntax {
                        return self.error("unexpected identifier");
                    } else {
                        break;
                    }
                }
            };
        }

        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn parse_key(&mut self) -> ParserResult<()> {
        if self.parse_ident().is_err() {
            return self.report_error("expected identifier");
        }

        let mut after_period = false;
        loop {
            let t = match self.get_token() {
                Ok(token) => token,
                Err(_) => {
                    if !after_period {
                        return Ok(());
                    }
                    return self.error("unexpected end of input");
                }
            };

            match t {
                PERIOD => {
                    if after_period {
                        return self.error(r#"unexpected ".""#);
                    } else {
                        self.token()?;
                        after_period = true;
                    }
                }
                BRACKET_START if self.key_pattern_syntax => {
                    self.step();

                    match self.parse_ident() {
                        Ok(_) => {}
                        Err(_) => return self.error("expected identifier"),
                    }

                    let token = self.get_token()?;

                    if !matches!(token, BRACKET_END) {
                        self.error(r#"expected "]""#)?;
                    }
                    self.step();
                    after_period = false;
                }
                _ => {
                    if after_period {
                        match self.parse_ident() {
                            Ok(_) => {}
                            Err(_) => return self.report_error("expected identifier"),
                        }
                        after_period = false;
                    } else if self.key_pattern_syntax {
                        return self.error("unexpected identifier");
                    } else {
                        break;
                    }
                }
            };
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
