# escape.check_escape

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\escape.rs` (lines 121-177)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn check_escape(s: &str) -> Result<(), Vec<usize>> {
    let mut lexer: Lexer<Escape> = Lexer::new(s);
    let mut invalid = Vec::new();

    while let Some(t) = lexer.next() {
        match t {
            Backspace => {}
            Tab => {}
            LineFeed => {}
            FormFeed => {}
            CarriageReturn => {}
            Quote => {}
            Backslash => {}
            Newline => {}
            Unicode => {
                let char_val = match u32::from_str_radix(&lexer.slice()[2..], 16) {
                    Ok(v) => v,
                    Err(_) => {
                        invalid.push(lexer.span().start);
                        continue;
                    }
                };

                match std::char::from_u32(char_val) {
                    None => {
                        invalid.push(lexer.span().start);
                    }
                    Some(_) => {}
                };
            }
            UnicodeLarge => {
                let char_val = match u32::from_str_radix(&lexer.slice()[2..], 16) {
                    Ok(v) => v,
                    Err(_) => {
                        invalid.push(lexer.span().start);
                        continue;
                    }
                };

                match std::char::from_u32(char_val) {
                    None => {
                        invalid.push(lexer.span().start);
                    }
                    Some(_) => {}
                };
            }
            Unknown => invalid.push(lexer.span().start),
            UnEscaped => {}
        }
    }

    if invalid.is_empty() {
        Ok(())
    } else {
        Err(invalid)
    }
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn check_escape(s: &str) -> Result<(), Vec<usize>> {
    let mut lexer: Lexer<Escape> = Lexer::new(s);
    let mut invalid = Vec::new();

    while let Some(t) = lexer.next() {
        match t {
            Backspace => {}
            Tab => {}
            LineFeed => {}
            FormFeed => {}
            CarriageReturn => {}
            Quote => {}
            Backslash => {}
            Newline => {}
            Unicode => {
                let char_val = match u32::from_str_radix(&lexer.slice()[2..], 16) {
                    Ok(v) => v,
                    Err(_) => {
                        invalid.push(lexer.span().start);
                        continue;
                    }
                };

                match std::char::from_u32(char_val) {
                    None => {
                        invalid.push(lexer.span().start);
                    }
                    Some(_) => {}
                };
            }
            UnicodeLarge => {
                let char_val = match u32::from_str_radix(&lexer.slice()[2..], 16) {
                    Ok(v) => v,
                    Err(_) => {
                        invalid.push(lexer.span().start);
                        continue;
                    }
                };

                match std::char::from_u32(char_val) {
                    None => {
                        invalid.push(lexer.span().start);
                    }
                    Some(_) => {}
                };
            }
            Unknown => invalid.push(lexer.span().start),
            UnEscaped => {}
        }
    }

    if invalid.is_empty() {
        Ok(())
    } else {
        Err(invalid)
    }
}
```

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
