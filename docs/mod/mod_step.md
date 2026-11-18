# mod.step

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 264-311)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn step(&mut self) {
        self.current_token = None;
        while let Some(token) = self.lexer.next() {
            match token {
                COMMENT => {
                    match allowed_chars::comment(self.lexer.slice()) {
                        Ok(_) => {}
                        Err(err_indices) => {
                            for e in err_indices {
                                self.add_error(&Error {
                                    range: TextRange::new(
                                        (self.lexer.span().start + e).try_into().unwrap(),
                                        (self.lexer.span().start + e).try_into().unwrap(),
                                    ),
                                    message: "invalid character in comment".into(),
                                });
                            }
                        }
                    };

                    self.insert_token(token, self.lexer.slice());
                }
                WHITESPACE => {
                    if self.skip_whitespace {
                        self.insert_token(token, self.lexer.slice());
                    } else {
                        self.current_token = Some(token);
                        break;
                    }
                }
                ERROR => {
                    self.insert_token(token, self.lexer.slice());
                    let span = self.lexer.span();
                    self.add_error(&Error {
                        range: TextRange::new(
                            span.start.try_into().unwrap(),
                            span.end.try_into().unwrap(),
                        ),
                        message: "unexpected token".into(),
                    })
                }
                _ => {
                    self.current_token = Some(token);
                    break;
                }
            }
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn step(&mut self) {
        self.current_token = None;
        while let Some(token) = self.lexer.next() {
            match token {
                COMMENT => {
                    match allowed_chars::comment(self.lexer.slice()) {
                        Ok(_) => {}
                        Err(err_indices) => {
                            for e in err_indices {
                                self.add_error(&Error {
                                    range: TextRange::new(
                                        (self.lexer.span().start + e).try_into().unwrap(),
                                        (self.lexer.span().start + e).try_into().unwrap(),
                                    ),
                                    message: "invalid character in comment".into(),
                                });
                            }
                        }
                    };

                    self.insert_token(token, self.lexer.slice());
                }
                WHITESPACE => {
                    if self.skip_whitespace {
                        self.insert_token(token, self.lexer.slice());
                    } else {
                        self.current_token = Some(token);
                        break;
                    }
                }
                ERROR => {
                    self.insert_token(token, self.lexer.slice());
                    let span = self.lexer.span();
                    self.add_error(&Error {
                        range: TextRange::new(
                            span.start.try_into().unwrap(),
                            span.end.try_into().unwrap(),
                        ),
                        message: "unexpected token".into(),
                    })
                }
                _ => {
                    self.current_token = Some(token);
                    break;
                }
            }
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
