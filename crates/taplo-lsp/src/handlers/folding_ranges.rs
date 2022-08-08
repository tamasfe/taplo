#![allow(clippy::cast_possible_truncation)]

use lsp_async_stub::{rpc::Error, util::Mapper, Context, Params};
use lsp_types::{FoldingRange, FoldingRangeKind, FoldingRangeParams};
use taplo::{
    dom::node::DomNode,
    rowan::TextRange,
    syntax::{
        SyntaxElement,
        SyntaxKind::{
            ARRAY, COMMENT, MULTI_LINE_STRING, MULTI_LINE_STRING_LITERAL, NEWLINE,
            TABLE_ARRAY_HEADER, TABLE_HEADER, WHITESPACE,
        },
        SyntaxNode,
    },
};
use taplo_common::environment::Environment;

use crate::world::World;

#[tracing::instrument(skip_all)]
pub(crate) async fn folding_ranges<E: Environment>(
    context: Context<World<E>>,
    params: Params<FoldingRangeParams>,
) -> Result<Option<Vec<FoldingRange>>, Error> {
    let p = params.required()?;

    let workspaces = context.workspaces.read().await;
    let ws = workspaces.by_document(&p.text_document.uri);
    let doc = match ws.document(&p.text_document.uri) {
        Ok(d) => d,
        Err(error) => {
            tracing::debug!(%error, "failed to get document from workspace");
            return Ok(None);
        }
    };

    Ok(Some(create_folding_ranges(
        doc.dom.syntax().unwrap().as_node().unwrap(),
        &doc.mapper,
    )))
}

#[tracing::instrument(skip_all)]
pub fn create_folding_ranges(syntax: &SyntaxNode, mapper: &Mapper) -> Vec<FoldingRange> {
    let mut folding_ranges = Vec::with_capacity(20);

    let mut comments_start: Option<TextRange> = None;
    let mut last_comment: Option<TextRange> = None;
    let mut was_comment: bool = false;

    let mut header_starts: Vec<(String, TextRange)> = Vec::new();

    let mut last_non_header: Option<TextRange> = None;

    for element in syntax.children_with_tokens() {
        let mut is_comment = false;

        match element.kind() {
            TABLE_ARRAY_HEADER | TABLE_HEADER => {
                let key = element
                    .as_node()
                    .unwrap()
                    .first_child()
                    .unwrap()
                    .text()
                    .to_string();

                if let Some(e) = &last_non_header {
                    header_starts.retain(|(k, h)| {
                        if k == &key || !key.starts_with(k) {
                            folding_ranges.push(FoldingRange {
                                start_line: mapper.position(h.start()).unwrap().line as u32,
                                start_character: None,
                                end_line: mapper
                                    .position(e.end().checked_sub(1.into()).unwrap_or_default())
                                    .unwrap()
                                    .line as u32,
                                end_character: None,
                                kind: Some(FoldingRangeKind::Region),
                            });

                            false
                        } else {
                            true
                        }
                    });
                }

                header_starts.push((key, element.text_range()));
                last_non_header = None;
            }
            WHITESPACE => {
                if was_comment {
                    is_comment = true;
                }
            }
            _ => {
                last_non_header = Some(element.text_range());

                match element {
                    SyntaxElement::Node(n) => {
                        for d in n.descendants_with_tokens() {
                            match d.kind() {
                                ARRAY => {
                                    if d.as_node()
                                        .unwrap()
                                        .descendants_with_tokens()
                                        .any(|t| t.kind() == NEWLINE)
                                    {
                                        let start =
                                            mapper.position(d.text_range().start()).unwrap();
                                        let end = mapper
                                            .position(
                                                d.text_range()
                                                    .end()
                                                    .checked_sub(1.into())
                                                    .unwrap_or_default(),
                                            )
                                            .unwrap();

                                        folding_ranges.push(FoldingRange {
                                            start_line: start.line as u32,
                                            start_character: Some(start.character as u32),
                                            end_line: end.line as u32,
                                            end_character: Some(end.character as u32),
                                            kind: Some(FoldingRangeKind::Region),
                                        });
                                    }
                                }
                                MULTI_LINE_STRING | MULTI_LINE_STRING_LITERAL => {
                                    if d.as_token().unwrap().text().contains('\n') {
                                        let start =
                                            mapper.position(d.text_range().start()).unwrap();
                                        let end = mapper
                                            .position(
                                                d.text_range()
                                                    .end()
                                                    .checked_sub(1.into())
                                                    .unwrap_or_default(),
                                            )
                                            .unwrap();

                                        folding_ranges.push(FoldingRange {
                                            start_line: start.line as u32,
                                            start_character: Some(start.character as u32),
                                            end_line: end.line as u32,
                                            end_character: Some(end.character as u32),
                                            kind: Some(FoldingRangeKind::Region),
                                        });
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    SyntaxElement::Token(t) => match t.kind() {
                        COMMENT => {
                            if comments_start.is_none() {
                                comments_start = Some(t.text_range());
                            }
                            last_comment = Some(t.text_range());
                            is_comment = true;
                        }
                        NEWLINE => {
                            if was_comment && t.text().matches('\n').count() == 1 {
                                // Ignore a single newline when counting comments
                                is_comment = true;
                            }
                        }
                        _ => {}
                    },
                }
            }
        }

        was_comment = is_comment;

        if !is_comment && last_comment.is_some() {
            folding_ranges.push(FoldingRange {
                start_line: mapper
                    .position(comments_start.unwrap().start())
                    .unwrap()
                    .line as u32,
                start_character: None,
                end_line: mapper.position(last_comment.unwrap().start()).unwrap().line as u32,
                end_character: None,
                kind: Some(FoldingRangeKind::Comment),
            });
            comments_start = None;
            last_comment = None;
        }
    }

    if let Some(e) = &last_non_header {
        for (_, h) in header_starts {
            folding_ranges.push(FoldingRange {
                start_line: mapper.position(h.start()).unwrap().line as u32,
                start_character: None,
                end_line: mapper
                    .position(e.end().checked_sub(1.into()).unwrap_or_default())
                    .unwrap()
                    .line as u32,
                end_character: None,
                kind: Some(FoldingRangeKind::Region),
            });
        }
    }

    if let Some(c) = comments_start {
        if let Some(l) = last_comment {
            folding_ranges.push(FoldingRange {
                start_line: mapper.position(c.start()).unwrap().line as u32,
                start_character: None,
                end_line: mapper.position(l.start()).unwrap().line as u32,
                end_character: None,
                kind: Some(FoldingRangeKind::Comment),
            });
        }
    }

    folding_ranges
}
