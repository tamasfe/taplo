use lsp_types::{Range, SemanticToken, SemanticTokenModifier, SemanticTokenType};
use taplo::{
    syntax::{SyntaxElement, SyntaxKind::*, SyntaxNode, SyntaxToken},
    util::coords::{relative_range, Mapper, SplitLines},
};

use crate::utils::LspExt;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum TokenType {
    TomlArrayKey,
    TomlTableKey,
}

impl TokenType {
    pub const LEGEND: &'static [SemanticTokenType] = &[
        SemanticTokenType::new("tomlArrayKey"),
        SemanticTokenType::new("tomlTableKey"),
    ];
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum TokenModifier {
    ReadOnly,
}

impl TokenModifier {
    pub const MODIFIERS: &'static [SemanticTokenModifier] = &[SemanticTokenModifier::READONLY];
}

pub fn create_tokens(syntax: &SyntaxNode, mapper: &Mapper) -> Vec<SemanticToken> {
    let mut builder = SemanticTokensBuilder::new(mapper);

    for element in syntax.descendants_with_tokens() {
        match element {
            SyntaxElement::Node(_node) => {}
            SyntaxElement::Token(token) => {
                if let IDENT = token.kind() {
                    // look for an inline table value
                    let is_table_key = token
                        .parent()
                        .next_sibling()
                        .and_then(|t| t.first_child())
                        .map(|t| t.kind() == INLINE_TABLE)
                        .unwrap_or(false);

                    if is_table_key {
                        builder.add_token(&token, TokenType::TomlTableKey, &[]);
                        continue;
                    }

                    // look for an array
                    let is_array_key = token
                        .parent()
                        .next_sibling()
                        .and_then(|t| t.first_child())
                        .map(|t| t.kind() == ARRAY)
                        .unwrap_or(false);

                    if is_array_key {
                        builder.add_token(&token, TokenType::TomlArrayKey, &[]);
                    }
                }
            }
        }
    }

    builder.build()
}

struct SemanticTokensBuilder<'b> {
    tokens: Vec<SemanticToken>,
    mapper: &'b Mapper,
    last_range: Option<Range>,
}

impl<'b> SemanticTokensBuilder<'b> {
    fn new(mapper: &'b Mapper) -> Self {
        Self {
            tokens: Vec::new(),
            mapper,
            last_range: None,
        }
    }

    fn add_token(
        &mut self,
        token: &SyntaxToken,
        ty: TokenType,
        modifiers: &[SemanticTokenModifier],
    ) {
        let range = self.mapper.range(token.text_range()).unwrap();

        if range.is_single_line() {
            let relative = relative_range(
                range,
                taplo::util::coords::Range::from_lsp(self.last_range.unwrap_or_default()),
            );

            self.tokens.push(SemanticToken {
                delta_line: relative.start.line as u32,
                delta_start: relative.start.character as u32,
                length: (relative.end.character - relative.start.character) as u32,
                token_type: ty as u32,
                token_modifiers_bitset: modifiers.iter().enumerate().fold(
                    0,
                    |mut total, (i, _)| {
                        total += 1 << i;
                        total
                    },
                ),
            });

            self.last_range = Some(range.into_lsp());
        } else {
            let ranges = range.split_lines(&self.mapper);

            for r in ranges {
                let relative = relative_range(
                    r,
                    taplo::util::coords::Range::from_lsp(self.last_range.unwrap_or_default()),
                );
                self.tokens.push(SemanticToken {
                    delta_line: relative.start.line as u32,
                    delta_start: relative.start.character as u32,
                    length: (relative.end.character - relative.start.character + 1) as u32,
                    token_type: ty as u32,
                    token_modifiers_bitset: modifiers.iter().enumerate().fold(
                        0,
                        |mut total, (i, _)| {
                            total += 1 << i;
                            total
                        },
                    ),
                });

                self.last_range = Some(r.into_lsp());
            }
        }
    }

    fn build(self) -> Vec<SemanticToken> {
        self.tokens
    }
}
