# Module: escape

**File:** `\crates\taplo\src\util\escape.rs`

## Overview

Module `escape` contains 3 analyzed entities.

```
use logos::{Lexer, Logos};

/// Escaping based on:
///
/// \b         - backspace       (U+0008)
/// \t         - tab             (U+0009)
/// \n         - linefeed        (U+000A)
/// \f         - form feed       (U+000C)
/// \r         - carriage return (U+000D)
/// \"         - quote           (U+0022)
/// \\         - backslash       (U+005C)
/// \uXXXX     - unicode         (U+XXXX)
/// \UXXXXXXXX - unicode         (U+XXXXXXXX)
#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Escape {
    #[token(r#"\b"#)]
    Backspace,

    #[token(r#"\t"#)]
    Tab,

    #[regex(r#"(\\\s*\n)|(\\\s*\r\n)"#)]
    Newline,

    #[token(r#"\n"#)]
    LineFeed,

    #[token(r#"\f"#)]
    FormFeed,

    #[token(r#"\r"#)]
    CarriageReturn,

    #[token(r#"\""#)]
    Quote,

    #[token(r#"\\"#)]
    Backslash,

    // Same thing repeated 4 times, but the {n} repetition syntax is not supported by Logos
    #[regex(r#"\\u[0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_]"#)]
    Unicode,

    // Same thing repeated 8 times, but the {n} repetition syntax is not supported by Logos
    #[regex(r#"\\U[0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_][0-9A-Fa-f_]"#)]
    UnicodeLarge,

    #[regex(r#"\\."#)]
    Unknown,

    #[error]
    UnEscaped,
}
use Escape::*;

/// Escape values in a given string.
pub fn escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '\u{0008}' => escaped.push_str(r#"\b"#),
            '\u{0009}' => escaped.push_str(r#"\t"#),
            '\u{000A}' => escaped.push_str(r#"\n"#),
            '\u{000C}' => escaped.push_str(r#"\f"#),
            '\u{000D}' => escaped.push_str(r#"\r"#),
            '\u{0022}' => escaped.push_str(r#"\""#),
            '\u{005C}' => escaped.push_str(r#"\\"#),
            _ => {
                escaped.push(c);
            }
        }
    }

    escaped
}

/// Unescape all supported 
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `escape.escape` | function | No description |
| `escape.check_escape` | function | No description |
| `escape.unescape` | function | No description |

## Git History

- Commits: 2
- Authors: 1
- Bug fixes: 0
- Features: 2

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
