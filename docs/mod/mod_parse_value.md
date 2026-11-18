# mod.parse_value

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 592-787)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn parse_value(&mut self) -> ParserResult<()> {
        let t = match self.get_token() {
            Ok(t) => t,
            Err(_) => return self.error("expected value"),
        };

        match t {
            BOOL | DATE_TIME_OFFSET | DATE_TIME_LOCAL | DATE | TIME => self.token(),
            INTEGER => {
                // This is probably a logos bug or a priority issue,
                // for some reason "1979-05-27" gets lexed as INTEGER.
                if !self.lexer.slice().starts_with('-') && self.lexer.slice().contains('-') {
                    return self.token_as(DATE);
                }

                // FIXME: probably another logos bug.
                if self.lexer.slice().contains(':') {
                    return self.token_as(TIME);
                }

                // This could've been done more elegantly probably.
                if (self.lexer.slice().starts_with('0') && self.lexer.slice() != "0")
                    || (self.lexer.slice().starts_with("+0") && self.lexer.slice() != "+0")
                    || (self.lexer.slice().starts_with("-0") && self.lexer.slice() != "-0")
                {
                    self.error("zero-padded integers are not allowed")
                } else if !check_underscores(self.lexer.slice(), 10) {
                    self.error("invalid underscores")
                } else {
                    self.token()
                }
            }
            INTEGER_BIN => {
                if !check_underscores(self.lexer.slice(), 2) {
                    self.error("invalid underscores")
                } else {
                    self.token()
                }
            }
            INTEGER_HEX => {
                if !check_underscores(self.lexer.slice(), 16) {
                    self.error("invalid underscores")
                } else {
                    self.token()
                }
            }
            INTEGER_OCT => {
                if !check_underscores(self.lexer.slice(), 8) {
                    self.error("invalid underscores")
                } else {
                    self.token()
                }
            }
            FLOAT => {
                // FIXME: probably another logos bug.
                if self.lexer.slice().contains(':') {
                    return self.token_as(TIME);
                }

                let int_slice = if self.lexer.slice().contains('.') {
                    self.lexer.slice().split('.').next().unwrap()
                } else {
                    self.lexer.slice().split('e').next().unwrap()
                };

                if (int_slice.starts_with('0') && int_slice != "0")
                    || (int_slice.starts_with("+0") && int_slice != "+0")
                    || (int_slice.starts_with("-0") && int_slice != "-0")
                {
                    self.error("zero-padded numbers are not allowed")
                } else if !check_underscores(self.lexer.slice(), 10) {
                    self.error("invalid underscores")
                } else {
                    self.token()
                }
            }
            STRING_LITERAL => {
                match allowed_chars::string_literal(self.lexer.slice()) {
                    Ok(_) => {}
                    Err(err_indices) => {
                        for e in err_indices {
                            self.add_error(&Error {
                                range: TextRange::new(
                                    (self.lexer.span().start + e).try_into().unwrap(),
                                    (self.lexer.span().start + e).try_into().unwrap(),
                                ),
                                message: "invalid control character in string literal".into(),
                            });
                        }
                    }
                };
                self.token()
            }
            MULTI_LINE_STRING_LITERAL => {
                match allowed_chars::multi_line_string_literal(self.lexer.slice()) {
                    Ok(_) => {}
                    Err(err_indices) => {
                        for e in err_indices {
                            self.add_error(&Error {
                                range: TextRange::new(
                                    (self.lexer.span().start + e).try_into().unwrap(),
                                    (self.lexer.span().start + e).try_into().unwrap(),
                                ),
                                message: "invalid character in string".into(),
                            });
                        }
                    }
                };
                self.token()
            }
            STRING => {
                match allowed_chars::string(self.lexer.slice()) {
                    Ok(_) => {}
                    Err(err_indices) => {
                        for e in err_indices {
                            self.add_error(&Error {
                                range: TextRange::new(
                                    (self.lexer.span().start + e).try_into().unwrap(),
                                    (self.lexer.span().start + e).try_into().unwrap(),
                                ),
                                message: "invalid character in string".into(),
                            });
                        }
                    }
                };

                match check_escape(self.lexer.slice()) {
                    Ok(_) => self.token(),
                    Err(err_indices) => {
                        for e in err_indices {
                            self.add_error(&Error {
                                range: TextRange::new(
                                    (self.lexer.span().start + e).try_into().unwrap(),
                                    (self.lexer.span().start + e).try_into().unwrap(),
                                ),
                                message: "invalid escape sequence".into(),
                            });
                        }

                        // We proceed normally even if
                        // the string contains invalid escapes.
                        // It shouldn't affect the rest of the parsing.
                        self.token()
                    }
                }
            }
            MULTI_LINE_STRING => {
                match allowed_chars::multi_line_string(self.lexer.slice()) {
                    Ok(_) => {}
                    Err(err_indices) => {
                        for e in err_indices {
                            self.add_error(&Error {
                                range: TextRange::new(
                                    (self.lexer.span().start + e).try_into().unwrap(),
                                    (self.lexer.span().start + e).try_into().unwrap(),
                                ),
                                message: "invalid character in string".into(),
                            });
                        }
                    }
                };

                match check_escape(self.lexer.slice()) {
                    Ok(_) => self.token(),
                    Err(err_indices) => {
                        for e in err_indices {
                            self.add_error(&Error {
                                range: TextRange::new(
                                    (self.lexer.span().start + e).try_into().unwrap(),
                                    (self.lexer.span().start + e).try_into().unwrap(),
                                ),
                                message: "invalid escape sequence".into(),
                            });
                        }

                        // We proceed normally even if
                        // the string contains invalid escapes.
                        // It shouldn't affect the rest of the parsing.
                        self.token()
                    }
                }
            }
            BRACKET_START => {
                with_node!(self.builder, ARRAY, self.parse_array())
            }
            BRACE_START => {
                with_node!(self.builder, INLINE_TABLE, self.parse_inline_table())
            }
            IDENT | BRACE_END => {
                // FIXME(bit_flags): This branch is just a workaround.
                self.report_error("expected value").ok();
                Ok(())
            }
            _ => self.error("expected value"),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn parse_value(&mut self) -> ParserResult<()> {
        let t = match self.get_token() {
            Ok(t) => t,
            Err(_) => return self.error("expected value"),
        };

        match t {
            BOOL | DATE_TIME_OFFSET | DATE_TIME_LOCAL | DATE | TIME => self.token(),
            INTEGER => {
                // This is probably a logos bug or a priority issue,
                // for some reason "1979-05-27" gets lexed as INTEGER.
                if !self.lexer.slice().starts_with('-') && self.lexer.slice().contains('-') {
                    return self.token_as(DATE);
                }

                // FIXME: probably another logos bug.
                if self.lexer.slice().contains(':') {
                    return self.token_as(TIME);
                }

                // This could've been done more elegantly probably.
                if (self.lexer.slice().starts_with('0') && self.lexer.slice() != "0")
                    || (self.lexer.slice().starts_with("+0") && self.lexer.slice() != "+0")
                    || (self.lexer.slice().starts_with("-0") && self.lexer.slice() != "-0")
                {
                    self.error("zero-padded integers are not allowed")
                } else if !check_underscores(self.lexer.slice(), 10) {
                    self.error("invalid underscores")
                } else {
                    self.token()
                }
            }
            INTEGER_BIN => {
                if !check_underscores(self.lexer.slice(), 2) {
                    self.error("invalid underscores")
                } else {
                    self.token()
                }
            }
            INTEGER_HEX => {
                if !check_underscores(self.lexer.slice(), 16) {
                    self.error("invalid underscores")
                } else {
                    self.token()
                }
            }
            INTEGER_OCT => {
                if !check_underscores(self.lexer.slice(), 8) {
                    self.error("invalid underscores")
                } else {
                    self.token()
                }
            }
            FLOAT => {
                // FIXME: probably another logos bug.
                if self.lexer.slice().contains(':') {
                    return self.token_as(TIME);
                }

                let int_slice = if self.lexer.slice().contains('.') {
                    self.lexer.slice().split('.').next().unwrap()
                } else {
                    self.lexer.slice().split('e').next().unwrap()
                };

                if (int_slice.starts_with('0') && int_slice != "0")
                    || (int_slice.starts_with("+0") && int_slice != "+0")
                    || (int_slice.starts_with("-0") && int_slice != "-0")
                {
                    self.error("zero-padded numbers are not allowed")
                } else if !check_underscores(self.lexer.slice(), 10) {
                    self.error("invalid underscores")
                } else {
                    self.token()
                }
            }
            STRING_LITERAL => {
                match allowed_chars::string_literal(self.lexer.slice()) {
                    Ok(_) => {}
                    Err(err_indices) => {
                        for e in err_indices {
                            self.add_error(&Error {
                                range: TextRange::new(
                                    (self.lexer.span().start + e).try_into().unwrap(),
                                    (self.lexer.span().start + e).try_into().unwrap(),
                                ),
                                message: "invalid control character in string literal".into(),
                            });
                        }
                    }
                };
                self.token()
            }
            MULTI_LINE_STRING_LITERAL => {
                match allowed_chars::multi_line_string_literal(self.lexer.slice()) {
                    Ok(_) => {}
                    Err(err_indices) => {
                        for e in err_indices {
                            self.add_error(&Error {
                                range: TextRange::new(
... (truncated)
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
