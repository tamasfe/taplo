# Module: syntax

**File:** `\crates\taplo\src\syntax.rs`

## Overview

Module `syntax` contains 1 analyzed entities.

```
//! Declaration of the syntax tokens and lexer implementation.

#![allow(non_camel_case_types)]

use logos::{Lexer, Logos};

/// Enum containing all the tokens in a syntax tree.
#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    #[regex(r"([ \t])+")]
    WHITESPACE = 0,

    #[regex(r"(\n|\r\n)+")]
    NEWLINE,

    #[regex(r"#[^\n\r]*")]
    COMMENT,

    #[regex(r"[A-Za-z0-9_-]+", priority = 2)]
    IDENT,

    /// Not part of the regular TOML syntax, only used to allow
    /// glob patterns in keys.
    #[regex(r"[*?A-Za-z0-9_-]+")]
    IDENT_WITH_GLOB,

    #[token(".")]
    PERIOD,

    #[token(",")]
    COMMA,

    #[token("=")]
    EQ,

    #[regex(r#"""#, lex_string)]
    STRING,

    #[regex(r#"""""#, lex_multi_line_string)]
    MULTI_LINE_STRING,

    #[regex(r#"'"#, lex_string_literal)]
    STRING_LITERAL,

    #[regex(r#"'''"#, lex_multi_line_string_literal)]
    MULTI_LINE_STRING_LITERAL,

    #[regex(r"[+-]?[0-9_]+", priority = 4)]
    INTEGER,

    #[regex(r"0x[0-9A-Fa-f_]+")]
    INTEGER_HEX,

    #[regex(r"0o[0-7_]+")]
    INTEGER_OCT,

    #[regex(r"0b(0|1|_)+")]
    INTEGER_BIN,

    #[regex(r"[-+]?([0-9_]+(\.[0-9_]+)?([eE][+-]?[0-9_]+)?|nan|inf)", priority = 3)]
    FLOAT,

    #[regex(r"true|false")]
    BOOL,

    #[regex(r#"(?:[0-9][0-9][0-9][0-9]-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:(?:[0-9][0-9](?:0[48]|[2468][048]|[13579][26]))|(?:[2468][048]|[13579][26])00)-02-29)(?:T|t| )(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:(?:\.|,)\d+)?(?:[Zz]|[+-][01]\d:[0-5]\d)"#)]
    DATE_TIME_OFFSET,

    #[regex(r#"(?:[0-9][0-9][0-9][0-9]-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:(?:[0-9][0-9](?:0[48]|[2468][048]|[13579][26]))|(?:[2468][048]|[13579][26])00)-02-29)(?:T|t| )(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d(?:(?:\.|,)\d+)?"#)]
    DATE_TIME_LOCAL,

    #[regex(r#"(?:[0-9][0-9][0-9][0-9]-(?:(
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `syntax.add_all` | function | No description |

## Git History

- Commits: 5
- Authors: 3
- Bug fixes: 2
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
