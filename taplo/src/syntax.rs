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

    #[regex(r"[A-Za-z0-9_-]+")]
    IDENT,

    #[token(".")]
    PERIOD,

    #[token(",")]
    COMMA,

    #[token("=")]
    EQ,

    #[regex(r#"""#, lex_string)] // " this is just to fix my IDE syntax highlight
    STRING,

    #[regex(r#"""""#, lex_multi_line_string)]
    // " this is just to fix my IDE syntax highlight
    MULTI_LINE_STRING,

    #[regex(r#"'"#, lex_string_literal)]
    STRING_LITERAL,

    #[regex(r#"'''"#, lex_multi_line_string_literal)]
    MULTI_LINE_STRING_LITERAL,

    #[regex(r"[+-]?[0-9_]+", priority = 3)]
    INTEGER,

    #[regex(r"0x[0-9A-Fa-f_]+")]
    INTEGER_HEX,

    #[regex(r"0o[0-7_]+")]
    INTEGER_OCT,

    #[regex(r"0b(0|1|_)+")]
    INTEGER_BIN,

    #[regex(r"[-+]?([0-9_]+\.?[0-9_]*([eE][+-]?[0-9_]*)?|nan|inf)", priority = 2)]
    FLOAT,

    #[regex(r"true|false")]
    BOOL,

    // Good luck debugging this
    #[regex(r"(([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])[Tt ]([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?(([Zz])|([\+|\-]([01][0-9]|2[0-3]):[0-5][0-9]))?|([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])|([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?)")]
    DATE,

    #[token("[")]
    BRACKET_START,

    #[token("]")]
    BRACKET_END,

    #[token("{")]
    BRACE_START,

    #[token("}")]
    BRACE_END,

    #[error]
    ERROR,

    // composite types
    KEY,                // e.g.: parent.child
    VALUE,              // e.g.: "2"
    TABLE_HEADER,       // e.g.: [table]
    TABLE_ARRAY_HEADER, // e.g.: [[table]]
    ENTRY,              // e.g.: key = "value"
    ARRAY,              // e.g.: [ 1, 2 ]
    INLINE_TABLE,       // e.g.: { key = "value" }

    ROOT, // root node
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}
impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<Lang>;
pub type SyntaxToken = rowan::SyntaxToken<Lang>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

fn lex_string(lex: &mut Lexer<SyntaxKind>) -> bool {
    let remainder: &str = lex.remainder();
    let mut escaped = false;

    let mut total_len = 0;

    for c in remainder.chars() {
        total_len += c.len_utf8();

        if c == '\\' {
            escaped = !escaped;
            continue;
        }

        if c == '"' && !escaped {
            lex.bump(remainder[0..total_len].as_bytes().len());
            return true;
        }

        escaped = false;
    }
    false
}

fn lex_multi_line_string(lex: &mut Lexer<SyntaxKind>) -> bool {
    let remainder: &str = lex.remainder();

    let mut total_len = 0;
    let mut quote_count = 0;

    let mut escaped = false;

    // As the string can contain ",
    // we can end up with more than 3 "-s at
    // the end, in that case we need to include all
    // in the string.
    let mut quotes_found = false;

    for c in remainder.chars() {
        if quotes_found {
            if c != '"' {
                lex.bump(remainder[0..total_len].as_bytes().len());
                return true;
            } else {
                total_len += c.len_utf8();
                continue;
            }
        }
        total_len += c.len_utf8();

        if c == '\\' {
            escaped = !escaped;
            continue;
        }

        if c == '"' && !escaped {
            quote_count += 1;
        } else {
            quote_count = 0;
        }

        if quote_count == 3 {
            quotes_found = true;
        }

        escaped = false;
    }

    // End of input
    if quotes_found {
        lex.bump(remainder[0..total_len].as_bytes().len());
        true
    } else {
        false
    }
}

fn lex_string_literal(lex: &mut Lexer<SyntaxKind>) -> bool {
    let remainder: &str = lex.remainder();
    let mut total_len = 0;

    for c in remainder.chars() {
        total_len += c.len_utf8();

        if c == '\'' {
            lex.bump(remainder[0..total_len].as_bytes().len());
            return true;
        }
    }
    false
}

fn lex_multi_line_string_literal(lex: &mut Lexer<SyntaxKind>) -> bool {
    let remainder: &str = lex.remainder();

    let mut total_len = 0;
    let mut quote_count = 0;

    // As the string can contain ',
    // we can end up with more than 3 '-s at
    // the end, in that case we need to include all
    // in the string.
    let mut quotes_found = false;

    for c in remainder.chars() {
        if quotes_found {
            if c != '\'' {
                lex.bump(remainder[0..total_len].as_bytes().len());
                return true;
            } else {
                if quote_count > 4 {
                    return false;
                }

                quote_count += 1;
                total_len += c.len_utf8();
                continue;
            }
        }
        total_len += c.len_utf8();

        if c == '\'' {
            quote_count += 1;
        } else {
            quote_count = 0;
        }

        if quote_count == 3 {
            quotes_found = true;
        }
    }

    // End of input
    if quotes_found {
        lex.bump(remainder[0..total_len].as_bytes().len());
        true
    } else {
        false
    }
}
