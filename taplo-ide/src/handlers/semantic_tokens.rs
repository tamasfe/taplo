use lsp_types::{Range, SemanticToken, SemanticTokenModifier, SemanticTokenType};
use taplo::{
    syntax::{SyntaxElement, SyntaxKind::*, SyntaxNode, SyntaxToken},
    util::coords::{relative_range, Mapper, SplitLines},
};

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum TokenType {
    Function = 0,
    Comment,
    Keyword,
    Namespace,
    String,
    Number,
    Variable,
    TomlArray,
}

impl TokenType {
    pub const LEGEND: &'static [SemanticTokenType] = &[
        SemanticTokenType::FUNCTION,
        SemanticTokenType::COMMENT,
        SemanticTokenType::KEYWORD,
        SemanticTokenType::NAMESPACE,
        SemanticTokenType::STRING,
        SemanticTokenType::NUMBER,
        SemanticTokenType::VARIABLE,
        SemanticTokenType::new("tomlArray"),
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
            SyntaxElement::Node(_node) => {
                // let n: SyntaxNode = _node;
            }
            SyntaxElement::Token(token) => match token.kind() {
                IDENT => {
                    let parent_node: SyntaxNode = token.parent();
                    let mut ty = TokenType::Variable;
                    if let Some(p) = parent_node.parent() {
                        match p.kind() {
                            TABLE_HEADER => ty = TokenType::Namespace,
                            TABLE_ARRAY_HEADER => ty = TokenType::TomlArray,
                            _ => {}
                        }
                    }

                    // look for an inline table value
                    let is_table_key = token
                        .parent()
                        .next_sibling()
                        .and_then(|t| t.first_child())
                        .map(|t| t.kind() == INLINE_TABLE)
                        .unwrap_or(false);

                    if is_table_key {
                        ty = TokenType::Namespace;
                    }

                    // look for an array
                    let is_array_key = token
                        .parent()
                        .next_sibling()
                        .and_then(|t| t.first_child())
                        .map(|t| t.kind() == ARRAY)
                        .unwrap_or(false);

                    if is_array_key {
                        ty = TokenType::TomlArray;
                    }

                    builder.add_token(&token, ty, &[]);
                }
                COMMENT => builder.add_token(&token, TokenType::Comment, &[]),
                FLOAT | INTEGER | INTEGER_BIN | INTEGER_HEX | INTEGER_OCT => {
                    builder.add_token(&token, TokenType::Number, &[])
                }
                STRING | MULTI_LINE_STRING | STRING_LITERAL | MULTI_LINE_STRING_LITERAL => {
                    builder.add_token(&token, TokenType::String, &[])
                }
                DATE => builder.add_token(&token, TokenType::Function, &[]),
                BOOL => builder.add_token(&token, TokenType::Keyword, &[]),
                _ => {}
            },
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
            let relative = relative_range(range, self.last_range.unwrap_or_default());

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

            self.last_range = Some(range);
        } else {
            let ranges = range.split_lines(&self.mapper);

            for r in ranges {
                let relative = relative_range(r, self.last_range.unwrap_or_default());
                self.tokens.push(SemanticToken {
                    delta_line: relative.start.line as u32,
                    delta_start: relative.start.character as u32,
                    length: (relative.end.character - relative.start.character) as u32,
                    token_type: ty as u32,
                    token_modifiers_bitset: 0,
                });

                self.last_range = Some(r);
            }
        }
    }

    fn build(self) -> Vec<SemanticToken> {
        self.tokens
    }
}
