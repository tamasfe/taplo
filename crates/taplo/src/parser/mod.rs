//! TOML document to syntax tree parsing.

use crate::{
    dom::{self, FromSyntax},
    syntax::{SyntaxKind, SyntaxKind::*, SyntaxNode},
    util::{allowed_chars, check_escape},
};
use logos::{Lexer, Logos};
use rowan::{GreenNode, GreenNodeBuilder, TextRange, TextSize};
use std::convert::TryInto;

#[macro_use]
mod macros;

/// A syntax error that can occur during parsing.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Error {
    /// The span of the error.
    pub range: TextRange,

    /// Human-friendly error message.
    pub message: String,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:?})", &self.message, &self.range)
    }
}
impl std::error::Error for Error {}

/// Parse a TOML document into a [Rowan green tree](rowan::GreenNode).
///
/// The parsing will not stop at unexpected or invalid tokens.
/// Instead errors will be collected with their character offsets and lengths,
/// and the invalid token(s) will have the `ERROR` kind in the final tree.
///
/// The parser will also validate comment and string contents, looking for
/// invalid escape sequences and invalid characters.
/// These will also be reported as syntax errors.
///
/// This does not check for semantic errors such as duplicate keys.
pub fn parse(source: &str) -> Parse {
    Parser::new(source).parse()
}

/// A hand-written parser that uses the Logos lexer
/// to tokenize the source, then constructs
/// a Rowan green tree from them.
pub(crate) struct Parser<'p> {
    skip_whitespace: bool,
    // Allow glob patterns as keys and using [] instead of dots.
    key_pattern_syntax: bool,
    current_token: Option<SyntaxKind>,

    // These tokens are not consumed on errors.
    //
    // The syntax error is still reported,
    // but the the surrounding context can still
    // be parsed.
    // FIXME(bit_flags):
    //      This is VERY wrong, as the members of the
    //      enums are not proper bit flags.
    //
    //      However this incorrect behavior marks fewer tokens
    //      as errors making the parser more fault-tolerant.
    //      Instead of fixing this it would probably be better to
    //      remove the ERROR token altogether, or reserving it for
    //      special cases.
    error_whitelist: u16,

    lexer: Lexer<'p, SyntaxKind>,
    builder: GreenNodeBuilder<'p>,
    errors: Vec<Error>,
}

impl Parser<'_> {
    /// Required for patch syntax
    /// and key matches.
    ///
    /// It allows a part of glob syntax in identifiers as well.
    pub(crate) fn parse_key_only(mut self) -> Parse {
        self.key_pattern_syntax = true;
        let _ = with_node!(self.builder, KEY, self.parse_key());

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }
}

/// This is just a convenience type during parsing.
/// It allows using "?", making the code cleaner.
type ParserResult<T> = Result<T, ()>;

// FIXME(recursion)
// Deeply nested structures cause stack overflow,
// this probably has to be rewritten into a state machine
// that contains minimal function calls.
impl<'p> Parser<'p> {
    pub(crate) fn new(source: &'p str) -> Self {
        Parser {
            current_token: None,
            skip_whitespace: true,
            key_pattern_syntax: false,
            error_whitelist: 0,
            lexer: SyntaxKind::lexer(source),
            builder: Default::default(),
            errors: Default::default(),
        }
    }

    fn parse(mut self) -> Parse {
        let _ = with_node!(self.builder, ROOT, self.parse_root());

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn error(&mut self, message: &str) -> ParserResult<()> {
        let span = self.lexer.span();

        let err = Error {
            range: TextRange::new(
                TextSize::from(span.start as u32),
                TextSize::from(span.end as u32),
            ),
            message: message.into(),
        };

        let same_error = self
            .errors
            .last()
            .map(|e| e.range == err.range)
            .unwrap_or(false);

        if !same_error {
            self.add_error(&Error {
                range: TextRange::new(
                    TextSize::from(span.start as u32),
                    TextSize::from(span.end as u32),
                ),
                message: message.into(),
            });
            if let Some(t) = self.current_token {
                if !self.whitelisted(t) {
                    self.token_as(ERROR).ok();
                }
            }
        } else {
            self.token_as(ERROR).ok();
        }

        Err(())
    }

    // report error without consuming the current the token
    fn report_error(&mut self, message: &str) -> ParserResult<()> {
        let span = self.lexer.span();
        self.add_error(&Error {
            range: TextRange::new(
                TextSize::from(span.start as u32),
                TextSize::from(span.end as u32),
            ),
            message: message.into(),
        });
        Err(())
    }

    fn add_error(&mut self, e: &Error) {
        if let Some(last_err) = self.errors.last_mut() {
            if last_err == e {
                return;
            }
        }

        self.errors.push(e.clone());
    }

    #[inline]
    fn whitelist_token(&mut self, token: SyntaxKind) {
        self.error_whitelist |= token as u16;
    }

    #[inline]
    fn blacklist_token(&mut self, token: SyntaxKind) {
        self.error_whitelist &= !(token as u16);
    }

    #[inline]
    fn whitelisted(&self, token: SyntaxKind) -> bool {
        self.error_whitelist & token as u16 != 0
    }

    fn insert_token(&mut self, kind: SyntaxKind, s: &str) {
        self.builder.token(kind.into(), s)
    }

    fn must_token_or(&mut self, kind: SyntaxKind, message: &str) -> ParserResult<()> {
        match self.get_token() {
            Ok(t) => {
                if kind == t {
                    self.token()
                } else {
                    self.error(message)
                }
            }
            Err(_) => {
                self.add_error(&Error {
                    range: TextRange::new(
                        self.lexer.span().start.try_into().unwrap(),
                        self.lexer.span().end.try_into().unwrap(),
                    ),
                    message: "unexpected EOF".into(),
                });
                Err(())
            }
        }
    }

    // This is the same as `token` but won't consume trailing whitespace.
    fn add_token(&mut self) -> ParserResult<()> {
        match self.get_token() {
            Err(_) => Err(()),
            Ok(token) => {
                self.builder.token(token.into(), self.lexer.slice());
                self.current_token = None;
                Ok(())
            }
        }
    }

    fn token(&mut self) -> ParserResult<()> {
        match self.get_token() {
            Err(_) => Err(()),
            Ok(token) => self.token_as(token),
        }
    }

    /// This function implicitly calls `step`,
    /// it was definitely not a good design decision
    /// but changing this behaviour involves a
    /// different syntax tree and breakages down the line.
    fn token_as(&mut self, kind: SyntaxKind) -> ParserResult<()> {
        self.token_as_no_step(kind)?;
        self.step();
        Ok(())
    }

    fn token_as_no_step(&mut self, kind: SyntaxKind) -> ParserResult<()> {
        match self.get_token() {
            Err(_) => return Err(()),
            Ok(_) => {
                self.builder.token(kind.into(), self.lexer.slice());
            }
        }

        Ok(())
    }

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

    fn get_token(&mut self) -> ParserResult<SyntaxKind> {
        if self.current_token.is_none() {
            self.step();
        }

        self.current_token.ok_or(())
    }

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

    fn parse_table_header(&mut self) -> ParserResult<()> {
        self.must_token_or(BRACKET_START, r#"expected "[""#)?;
        let _ = with_node!(self.builder, KEY, self.parse_key());
        self.must_token_or(BRACKET_END, r#"expected "]""#)?;

        Ok(())
    }

    fn parse_table_array_header(&mut self) -> ParserResult<()> {
        self.skip_whitespace = false;
        self.must_token_or(BRACKET_START, r#"expected "[[""#)?;
        self.must_token_or(BRACKET_START, r#"expected "[[""#)?;
        self.skip_whitespace = true;
        let _ = with_node!(self.builder, KEY, self.parse_key());
        self.skip_whitespace = false;
        let _ = self.must_token_or(BRACKET_END, r#"expected "]]""#);

        // Hack in order to avoid calling `step` after
        // the second closing bracket.
        let token = self.get_token()?;
        match token {
            BRACKET_END => {
                self.token_as_no_step(token)?;
            }
            _ => {
                self.error(r#"expected "]]"#)?;
            }
        }
        self.skip_whitespace = true;

        self.step();

        Ok(())
    }

    fn parse_entry(&mut self) -> ParserResult<()> {
        with_node!(self.builder, KEY, self.parse_key())?;
        self.must_token_or(EQ, r#"expected "=""#)?;
        with_node!(self.builder, VALUE, self.parse_value())?;

        Ok(())
    }

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

    fn parse_inline_table(&mut self) -> ParserResult<()> {
        self.must_token_or(BRACE_START, r#"expected "{""#)?;

        let mut first = true;
        let mut comma_last = false;
        let mut was_newline = false;

        loop {
            let t = match self.get_token() {
                Ok(t) => t,
                Err(_) => return self.report_error(r#"expected "}""#),
            };

            match t {
                BRACE_END => {
                    if comma_last {
                        // it is still reported as a syntax error,
                        // but we can still analyze it as if it was a valid
                        // table.
                        let _ = self.report_error("expected value, trailing comma is not allowed");
                    }
                    break self.add_token()?;
                }
                NEWLINE => {
                    // To avoid infinite loop in case
                    // new lines are whitelisted.
                    if was_newline {
                        break;
                    }

                    let _ = self.error("newline is not allowed in an inline table");
                    was_newline = true;
                }
                COMMA => {
                    if comma_last {
                        let _ = self.report_error(r#"unexpected ",""#);
                    }

                    if first {
                        let _ = self.error(r#"unexpected ",""#);
                    } else {
                        self.token()?;
                    }
                    comma_last = true;
                    was_newline = false;
                }
                _ => {
                    was_newline = false;
                    if !comma_last && !first {
                        let _ = self.error(r#"expected ",""#);
                    }
                    let _ = whitelisted!(
                        self,
                        COMMA,
                        with_node!(self.builder, ENTRY, self.parse_entry())
                    );
                    comma_last = false;
                }
            }

            first = false;
        }
        Ok(())
    }

    fn parse_array(&mut self) -> ParserResult<()> {
        self.must_token_or(BRACKET_START, r#"expected "[""#)?;

        let mut first = true;
        let mut comma_last = false;
        loop {
            let t = match self.get_token() {
                Ok(t) => t,
                Err(_) => {
                    let _ = self.report_error("unexpected EOF");
                    return Err(());
                }
            };

            match t {
                BRACKET_END => break self.add_token()?,
                NEWLINE => {
                    self.token()?;
                    continue; // as if it wasn't there, so it doesn't count as a first token
                }
                COMMA => {
                    if first || comma_last {
                        let _ = self.error(r#"unexpected ",""#);
                    }
                    self.token()?;
                    comma_last = true;
                }
                _ => {
                    if !comma_last && !first {
                        let _ = self.error(r#"expected ",""#);
                    }
                    let _ = whitelisted!(
                        self,
                        COMMA,
                        with_node!(self.builder, VALUE, self.parse_value())
                    );
                    comma_last = false;
                }
            }

            first = false;
        }
        Ok(())
    }
}

fn check_underscores(s: &str, radix: u32) -> bool {
    if s.starts_with('_') || s.ends_with('_') {
        return false;
    }

    let mut last_char = 0 as char;

    for c in s.chars() {
        if c == '_' && !last_char.is_digit(radix) {
            return false;
        }
        if !c.is_digit(radix) && last_char == '_' {
            return false;
        }
        last_char = c;
    }

    true
}

/// The final results of a parsing.
/// It contains the green tree, and
/// the errors that occurred during parsing.
#[derive(Debug, Clone)]
pub struct Parse {
    pub green_node: GreenNode,
    pub errors: Vec<Error>,
}

impl Parse {
    /// Turn the parse into a syntax node.
    pub fn into_syntax(self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node)
    }

    /// Turn the parse into a DOM tree.
    ///
    /// Any semantic errors that occur will be collected
    /// in the returned DOM node.
    pub fn into_dom(self) -> dom::node::Node {
        dom::Node::from_syntax(self.into_syntax().into())
    }
}
