# mod.parse_root

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\parser\mod.rs` (lines 321-390)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn parse_root(&mut self) -> ParserResult<()> {
        // Ensure we have newlines between entries
        let mut not_newline = false;

        // We want to make sure that an entry spans the
        // entire line, so we start/close its node manually.
        let mut entry_started = false;

        while let Ok(token) = self.get_token() {
            match token {
                BRACKET_START => {
                    if entry_started {
                        self.builder.finish_node();
                        entry_started = false;
                    }

                    if not_newline {
                        let _ = self.error("expected new line");
                        continue;
                    }

                    not_newline = true;

                    if self.lexer.remainder().starts_with('[') {
                        let _ = whitelisted!(
                            self,
                            NEWLINE,
                            with_node!(
                                self.builder,
                                TABLE_ARRAY_HEADER,
                                self.parse_table_array_header()
                            )
                        );
                    } else {
                        let _ = whitelisted!(
                            self,
                            NEWLINE,
                            with_node!(self.builder, TABLE_HEADER, self.parse_table_header())
                        );
                    }
                }
                NEWLINE => {
                    not_newline = false;
                    if entry_started {
                        self.builder.finish_node();
                        entry_started = false;
                    }
                    let _ = self.token();
                }
                _ => {
                    if not_newline {
                        let _ = self.error("expected new line");
                        continue;
                    }
                    if entry_started {
                        self.builder.finish_node();
                    }
                    not_newline = true;
                    self.builder.start_node(ENTRY.into());
                    entry_started = true;
                    let _ = whitelisted!(self, NEWLINE, self.parse_entry());
                }
            }
        }
        if entry_started {
            self.builder.finish_node();
        }

        Ok(())
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn parse_root(&mut self) -> ParserResult<()> {
        // Ensure we have newlines between entries
        let mut not_newline = false;

        // We want to make sure that an entry spans the
        // entire line, so we start/close its node manually.
        let mut entry_started = false;

        while let Ok(token) = self.get_token() {
            match token {
                BRACKET_START => {
                    if entry_started {
                        self.builder.finish_node();
                        entry_started = false;
                    }

                    if not_newline {
                        let _ = self.error("expected new line");
                        continue;
                    }

                    not_newline = true;

                    if self.lexer.remainder().starts_with('[') {
                        let _ = whitelisted!(
                            self,
                            NEWLINE,
                            with_node!(
                                self.builder,
                                TABLE_ARRAY_HEADER,
                                self.parse_table_array_header()
                            )
                        );
                    } else {
                        let _ = whitelisted!(
                            self,
                            NEWLINE,
                            with_node!(self.builder, TABLE_HEADER, self.parse_table_header())
                        );
                    }
                }
                NEWLINE => {
                    not_newline = false;
                    if entry_started {
                        self.builder.finish_node();
                        entry_started = false;
                    }
                    let _ = self.token();
                }
                _ => {
                    if not_newline {
                        let _ = self.error("expected new line");
                        continue;
                    }
                    if entry_started {
                        self.builder.finish_node();
                    }
                    not_newline = true;
                    self.builder.start_node(ENTRY.into());
                    entry_started = true;
                    let _ = whitelisted!(self, NEWLINE, self.parse_entry());
                }
            }
        }
        if entry_started {
            self.builder.finish_node();
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
