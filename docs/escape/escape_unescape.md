# escape.unescape

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\escape.rs` (lines 81-117)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn unescape(s: &str) -> Result<String, usize> {
    let mut new_s = String::with_capacity(s.len());
    let mut lexer: Lexer<Escape> = Lexer::new(s);

    while let Some(t) = lexer.next() {
        match t {
            Backspace => new_s += "\u{0008}",
            Tab => new_s += "\u{0009}",
            LineFeed => new_s += "\u{000A}",
            FormFeed => new_s += "\u{000C}",
            CarriageReturn => new_s += "\u{000D}",
            Quote => new_s += "\u{0022}",
            Backslash => new_s += "\u{005C}",
            Newline => {}
            Unicode => {
                new_s += &std::char::from_u32(
                    u32::from_str_radix(&lexer.slice()[2..], 16).map_err(|_| lexer.span().start)?,
                )
                .ok_or(lexer.span().start)?
                .to_string();
            }
            UnicodeLarge => {
                new_s += &std::char::from_u32(
                    u32::from_str_radix(&lexer.slice()[2..], 16).map_err(|_| lexer.span().start)?,
                )
                .ok_or(lexer.span().start)?
                .to_string();
            }
            Unknown => return Err(lexer.span().end),
            UnEscaped => {
                new_s += lexer.slice();
            }
        }
    }

    Ok(new_s + lexer.remainder())
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn unescape(s: &str) -> Result<String, usize> {
    let mut new_s = String::with_capacity(s.len());
    let mut lexer: Lexer<Escape> = Lexer::new(s);

    while let Some(t) = lexer.next() {
        match t {
            Backspace => new_s += "\u{0008}",
            Tab => new_s += "\u{0009}",
            LineFeed => new_s += "\u{000A}",
            FormFeed => new_s += "\u{000C}",
            CarriageReturn => new_s += "\u{000D}",
            Quote => new_s += "\u{0022}",
            Backslash => new_s += "\u{005C}",
            Newline => {}
            Unicode => {
                new_s += &std::char::from_u32(
                    u32::from_str_radix(&lexer.slice()[2..], 16).map_err(|_| lexer.span().start)?,
                )
                .ok_or(lexer.span().start)?
                .to_string();
            }
            UnicodeLarge => {
                new_s += &std::char::from_u32(
                    u32::from_str_radix(&lexer.slice()[2..], 16).map_err(|_| lexer.span().start)?,
                )
                .ok_or(lexer.span().start)?
                .to_string();
            }
            Unknown => return Err(lexer.span().end),
            UnEscaped => {
                new_s += lexer.slice();
            }
        }
    }

    Ok(new_s + lexer.remainder())
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
