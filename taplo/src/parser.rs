use crate::{
    dom,
    syntax::{Error, SyntaxKind, SyntaxKind::*, SyntaxNode},
    util::{allowed_chars, check_escape},
};
use dom::Cast;
use logos::{Lexer, Logos};
use rowan::{GreenNode, GreenNodeBuilder, SmolStr, TextRange};
use std::convert::TryInto;

pub fn parse(source: &str) -> Parse {
    Parser::new(source).parse()
}

/// A hand-written parser that uses the Logos lexer
/// to tokenize the source, then constructs
/// a Rowan green tree from them.
///
/// The parsing will not stop at unexpected or invalid tokens.
/// Instead errors will be collected with their character offsets and lengths,
/// and the invalid token(s) will have the `ERROR` kind in the final tree.
///
/// The parser will also validate comment and string contents, looking for
/// invalid escape sequences and invalid characters.
/// These will also be reported as syntax errors.
pub struct Parser<'p> {
    skip_whitespace: bool,
    current_token: Option<SyntaxKind>,
    lexer: Lexer<'p, SyntaxKind>,
    builder: GreenNodeBuilder<'p>,
    errors: Vec<Error>,
}

/// This is just a convenience type during parsing.
/// It allows using "?", making the code cleaner.
type ParserResult<T> = Result<T, ()>;

// FIXME(recursion)
// Deeply nested structures cause stack overflow,
// this probably has to be rewritten into a state machine
// that contains minimal function calls.
impl<'p> Parser<'p> {
    pub fn new(source: &'p str) -> Self {
        Parser {
            current_token: None,
            skip_whitespace: true,
            lexer: SyntaxKind::lexer(source),
            builder: Default::default(),
            errors: Default::default(),
        }
    }

    pub fn parse(mut self) -> Parse {
        self.with_node(ROOT, Self::parse_root).ok();

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn error(&mut self, message: &str) -> ParserResult<()> {
        let span = self.lexer.span();
        self.add_error(&Error {
            range: TextRange::new(span.start.try_into().unwrap(), span.end.try_into().unwrap()),
            message: message.into(),
        });
        self.token_as(ERROR).ok();
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

    fn insert_token(&mut self, kind: SyntaxKind, s: SmolStr) {
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

    fn token(&mut self) -> ParserResult<()> {
        match self.get_token() {
            Err(_) => Err(()),
            Ok(token) => self.token_as(token),
        }
    }

    fn token_as(&mut self, kind: SyntaxKind) -> ParserResult<()> {
        match self.get_token() {
            Err(_) => return Err(()),
            Ok(_) => {
                self.builder.token(kind.into(), self.lexer.slice().into());
            }
        }

        self.step();
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

                    self.insert_token(token, self.lexer.slice().into());
                }
                WHITESPACE => {
                    if self.skip_whitespace {
                        self.insert_token(token, self.lexer.slice().into());
                    } else {
                        self.current_token = Some(token);
                        break;
                    }
                }
                ERROR => {
                    self.insert_token(token, self.lexer.slice().into());
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

    fn with_node<F: FnOnce(&mut Self) -> ParserResult<()>>(
        &mut self,
        kind: SyntaxKind,
        f: F,
    ) -> ParserResult<()> {
        self.builder.start_node(kind.into());
        let res = f(self);
        self.builder.finish_node();
        res
    }

    fn parse_root(&mut self) -> ParserResult<()> {
        // Ensure we have newlines between entries
        let mut not_newline = false;

        while let Ok(token) = self.get_token() {
            match token {
                BRACKET_START => {
                    if not_newline {
                        self.error("expected new line").ok();
                        continue;
                    }

                    not_newline = true;

                    if self.lexer.remainder().starts_with("[") {
                        self.with_node(TABLE_ARRAY_HEADER, Self::parse_table_array_header)
                    } else {
                        self.with_node(TABLE_HEADER, Self::parse_table_header)
                    }
                }
                NEWLINE => {
                    not_newline = false;
                    self.token()
                }
                _ => {
                    if not_newline {
                        self.error("expected new line").ok();
                        continue;
                    }
                    not_newline = true;
                    self.with_node(ENTRY, Self::parse_entry)
                }
            }
            .ok();
        }

        Ok(())
    }

    fn parse_table_header(&mut self) -> ParserResult<()> {
        self.must_token_or(BRACKET_START, r#"expected "[""#)?;
        self.with_node(KEY, Self::parse_key)?;
        self.must_token_or(BRACKET_END, r#"expected "]""#)?;

        Ok(())
    }

    fn parse_table_array_header(&mut self) -> ParserResult<()> {
        self.must_token_or(BRACKET_START, r#"expected "[[""#)?;
        self.must_token_or(BRACKET_START, r#"expected "[[""#)?;
        self.with_node(KEY, Self::parse_key)?;
        self.must_token_or(BRACKET_END, r#"expected "]]""#)?;
        self.must_token_or(BRACKET_END, r#"expected "]]""#)?;

        Ok(())
    }

    fn parse_entry(&mut self) -> ParserResult<()> {
        self.with_node(KEY, Self::parse_key)?;
        self.must_token_or(EQ, r#"expected "=""#)?;
        if let Err(_) = self.with_node(VALUE, Self::parse_value) {
            self.error("expected value")?;
        }

        Ok(())
    }

    fn parse_key(&mut self) -> ParserResult<()> {
        if let Err(_) = self.parse_ident() {
            return self.error("expected identifier");
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
                _ => {
                    if after_period {
                        match self.parse_ident() {
                            Ok(_) => {}
                            Err(_) => return self.error("expected identifier"),
                        }
                        after_period = false;
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
            INTEGER => {
                if self.lexer.slice().starts_with("+") {
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
                if self.lexer.slice().starts_with("+") {
                    Err(())
                } else {
                    for (i, s) in self.lexer.slice().split(".").enumerate() {
                        if i != 0 {
                            self.insert_token(PERIOD, ".".into());
                        }

                        self.insert_token(IDENT, s.into());
                    }
                    self.step();
                    Ok(())
                }
            }
            _ => Err(()),
        }
    }

    fn parse_value(&mut self) -> ParserResult<()> {
        let t = self.get_token()?;

        match t {
            INTEGER_HEX | INTEGER_OCT | INTEGER_BIN | FLOAT | BOOL | DATE => self.token(),
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
            INTEGER => {
                // This could've been done more elegantly probably.
                if (self.lexer.slice().starts_with("0") && self.lexer.slice() != "0")
                    || (self.lexer.slice().starts_with("+0") && self.lexer.slice() != "+0")
                    || (self.lexer.slice().starts_with("-0") && self.lexer.slice() != "-0")
                {
                    self.error("zero-padded integers are not allowed")
                } else {
                    self.token()
                }
            }
            BRACKET_START => self.with_node(ARRAY, Self::parse_array),
            BRACE_START => self.with_node(INLINE_TABLE, Self::parse_inline_table),
            _ => self.error("expected value"),
        }
    }

    fn parse_inline_table(&mut self) -> ParserResult<()> {
        self.must_token_or(BRACE_START, r#"expected "{""#)?;

        let mut first = true;
        let mut comma_last = false;
        Ok(loop {
            let t = self.get_token()?;

            match t {
                BRACE_END => {
                    if comma_last {
                        break self.error("expected value")?;
                    }
                    break self.token()?;
                }
                NEWLINE => break self.error("newline is not allowed in an inline table")?,
                COMMA => {
                    if first {
                        break self.error(r#"unexpected ",""#)?;
                    }
                    self.token()?;
                    comma_last = true;
                }
                _ => {
                    if !comma_last && !first {
                        break self.error(r#"expected ",""#)?;
                    }
                    self.with_node(ENTRY, Self::parse_entry)?;
                    comma_last = false;
                }
            }

            first = false;
        })
    }

    fn parse_array(&mut self) -> ParserResult<()> {
        self.must_token_or(BRACKET_START, r#"expected "[""#)?;

        let mut first = true;
        let mut comma_last = false;
        Ok(loop {
            let t = self.get_token()?;

            match t {
                BRACKET_END => break self.token()?,
                NEWLINE => {
                    self.token()?;
                    continue; // as if it wasn't there, so it doesn't count as a first token
                }
                COMMA => {
                    if first {
                        break self.error(r#"unexpected ",""#)?;
                    }
                    self.token()?;
                    comma_last = true;
                }
                _ => {
                    if !comma_last && !first {
                        break self.error(r#"expected ",""#)?;
                    }
                    self.with_node(VALUE, Self::parse_value)?;
                    comma_last = false;
                }
            }

            first = false;
        })
    }
}

/// The final results of a parsing.
/// It contains the green tree, and
/// the syntax errors that ocurred during parsing.
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
    pub fn into_dom(self) -> dom::RootNode {
        dom::RootNode::cast(rowan::NodeOrToken::Node(self.into_syntax())).unwrap()
    }
}
