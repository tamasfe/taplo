# mod.parse_ident

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 496-590)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn parse_ident(&mut self) -> ParserResult<()> {
        let t = self.get_token()?;
        match t {
            IDENT => self.token(),
            IDENT_WITH_GLOB => {
                if self.key_pattern_syntax {
                    self.token_as(IDENT)
                } else {
                    self.error("expected identifier")
                }
            }
            INTEGER_HEX | INTEGER_BIN | INTEGER_OCT => self.token_as(IDENT),
            INTEGER => {
                if self.lexer.slice().starts_with('+') {
                    Err(())
                } else {
                    self.token_as(IDENT)
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

                self.token_as(IDENT)
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
                    Ok(_) => self.token_as(IDENT),
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
                        self.token_as(IDENT)
                    }
                }
            }
            FLOAT => {
                if self.lexer.slice().starts_with('0') {
                    self.error("zero-padded numbers are not allowed")
                } else if self.lexer.slice().starts_with('+') {
                    Err(())
                } else {
                    for (i, s) in self.lexer.slice().split('.').enumerate() {
                        if i != 0 {
                            self.insert_token(PERIOD, ".");
                        }

                        self.insert_token(IDENT, s);
                    }
                    self.step();
                    Ok(())
                }
            }
            BOOL => self.token_as(IDENT),
            DATE => self.token_as(IDENT),
            _ => self.error("expected identifier"),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn parse_ident(&mut self) -> ParserResult<()> {
        let t = self.get_token()?;
        match t {
            IDENT => self.token(),
            IDENT_WITH_GLOB => {
                if self.key_pattern_syntax {
                    self.token_as(IDENT)
                } else {
                    self.error("expected identifier")
                }
            }
            INTEGER_HEX | INTEGER_BIN | INTEGER_OCT => self.token_as(IDENT),
            INTEGER => {
                if self.lexer.slice().starts_with('+') {
                    Err(())
                } else {
                    self.token_as(IDENT)
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

                self.token_as(IDENT)
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
                    Ok(_) => self.token_as(IDENT),
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
                        self.token_as(IDENT)
                    }
                }
            }
            FLOAT => {
                if self.lexer.slice().starts_with('0') {
                    self.error("zero-padded numbers are not allowed")
                } else if self.lexer.slice().starts_with('+') {
                    Err(())
                } else {
                    for (i, s) in self.lexer.slice().split('.').enumerate() {
                        if i != 0 {
                            self.insert_token(PERIOD, ".");
                        }

                        self.insert_token(IDENT, s);
                    }
                    self.step();
                    Ok(())
                }
            }
            BOOL => self.token_as(IDENT),
            DATE => self.token_as(IDENT),
            _ => self.error("expected identifier"),
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
