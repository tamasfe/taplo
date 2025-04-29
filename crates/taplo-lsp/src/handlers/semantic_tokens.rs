use crate::World;
use lsp_async_stub::{
    rpc::Error,
    util::{relative_range, LspExt, Mapper},
    Context, Params,
};
use lsp_types::{
    Range, SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens,
    SemanticTokensParams, SemanticTokensResult,
};
use taplo::{
    dom::node::DomNode,
    syntax::{
        SyntaxElement,
        SyntaxKind::{ARRAY, IDENT, INLINE_TABLE},
        SyntaxNode, SyntaxToken,
    },
};
use taplo_common::environment::Environment;

#[tracing::instrument(skip_all)]
pub(crate) async fn semantic_tokens<E: Environment>(
    context: Context<World<E>>,
    params: Params<SemanticTokensParams>,
) -> Result<Option<SemanticTokensResult>, Error> {
    let p = params.required()?;

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&p.text_document.uri);

    if !ws.config.syntax.semantic_tokens {
        return Ok(None);
    }

    let doc = match ws.document(&p.text_document.uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: create_tokens(doc.dom.syntax().unwrap().as_node().unwrap(), &doc.mapper),
    })))
}

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

#[tracing::instrument(skip_all)]
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
                        .and_then(|p| p.next_sibling())
                        .and_then(|t| t.first_child())
                        .is_some_and(|t| t.kind() == INLINE_TABLE);

                    if is_table_key {
                        builder.add_token(&token, TokenType::TomlTableKey, &[]);
                        continue;
                    }

                    // look for an array
                    let is_array_key = token
                        .parent()
                        .and_then(|p| p.next_sibling())
                        .and_then(|t| t.first_child())
                        .is_some_and(|t| t.kind() == ARRAY);

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

        let relative = relative_range(
            range,
            lsp_async_stub::util::Range::from_lsp(self.last_range.unwrap_or_default()),
        );

        #[allow(clippy::cast_possible_truncation)]
        self.tokens.push(SemanticToken {
            delta_line: relative.start.line as u32,
            delta_start: relative.start.character as u32,
            length: (relative.end.character - relative.start.character) as u32,
            token_type: ty as u32,
            token_modifiers_bitset: modifiers.iter().enumerate().fold(0, |mut total, (i, _)| {
                total += 1 << i;
                total
            }),
        });

        self.last_range = Some(range.into_lsp());
    }

    fn build(self) -> Vec<SemanticToken> {
        self.tokens
    }
}
