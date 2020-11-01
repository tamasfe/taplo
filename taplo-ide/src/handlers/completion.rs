use crate::{
    schema::{
        contains_type, get_schema_objects, object_contains_type, resolve_object_ref, resolve_ref,
        ExtMeta, ExtendedSchema, EXTENSION_KEY,
    },
    Document,
};
use dom::{Cast, Entries};
use itertools::Itertools;
use lsp_types::*;
use rowan::{TextRange, TextSize};
use schemars::{
    schema::{InstanceType, ObjectValidation, RootSchema, Schema, SchemaObject, SingleOrVec},
    Map,
};
use std::collections::HashSet;
use taplo::{
    analytics::NodeRef,
    dom::{self, NodeSyntax},
    syntax::{SyntaxElement, SyntaxKind},
    util::SyntaxExt,
};

pub(crate) fn get_completions(
    doc: Document,
    position: Position,
    root_schema: RootSchema,
) -> Vec<CompletionItem> {
    let dom = doc.parse.clone().into_dom();

    let offset = doc.mapper.offset(position).unwrap();

    let query = dom.query_position(offset);

    if !query.is_completable() {
        return Vec::new();
    }

    match &query.before {
        Some(before) => {
            if query.is_inside_header() {
                let mut query_path = before.path.clone();
                if query.is_empty_header() {
                    query_path = dom::Path::new();
                } else if query_path.is_empty() {
                    query_path = query.after.path.clone();
                }

                // We always include the current object as well.
                query_path = query_path.skip_right(1);

                let range = before
                    .syntax
                    .range
                    .map(|range| doc.mapper.range(range).unwrap())
                    .or_else(|| {
                        query
                            .after
                            .syntax
                            .range
                            .map(|range| doc.mapper.range(range).unwrap())
                    });

                return get_schema_objects(query_path.clone(), &root_schema, true)
                    .into_iter()
                    .map(|s| s.descendants(&root_schema.definitions, 10))
                    .flatten()
                    .filter(|(_, s, _)| !s.is_hidden())
                    .filter(|(_, s, _)| {
                        if query
                            .after
                            .syntax
                            .syntax_kinds
                            .iter()
                            .any(|kind| *kind == SyntaxKind::TABLE_ARRAY_HEADER)
                        {
                            s.is_array_of_objects(&root_schema.definitions)
                        } else {
                            s.schema.object.is_some()
                        }
                    })
                    .unique_by(|(p, ..)| p.clone())
                    .map(|(path, schema, required)| {
                        key_completion(
                            &root_schema.definitions,
                            query_path.extend(path),
                            schema,
                            required,
                            range,
                            false,
                            None,
                            false,
                        )
                    })
                    .collect();
            } else {
                let node = before
                    .nodes
                    .last()
                    .cloned()
                    .unwrap_or_else(|| query.after.nodes.last().cloned().unwrap());

                let inline_table = query
                    .after
                    .syntax
                    .syntax_kinds
                    .iter()
                    .any(|k| *k == SyntaxKind::INLINE_TABLE);

                match node {
                    node @ NodeRef::Table(_) | node @ NodeRef::Root(_) => {
                        let mut additional_edits = Vec::new();

                        let mut query_path = before.path.clone();

                        if node.is_root() {
                            query_path = query_path.skip_right(1);
                        }

                        let range = before
                            .syntax
                            .range
                            .map(|range| doc.mapper.range(range).unwrap())
                            .or_else(|| {
                                query
                                    .after
                                    .syntax
                                    .range
                                    .map(|range| doc.mapper.range(range).unwrap())
                            });

                        let mut comma_before = false;

                        if inline_table {
                            if let Some((tok_range, tok)) = before.syntax.first_token_before() {
                                if tok.kind() != SyntaxKind::COMMA
                                    && tok.kind() != SyntaxKind::BRACE_START
                                {
                                    let range_after = TextRange::new(
                                        tok_range.start() + TextSize::from(1),
                                        tok_range.end() + TextSize::from(1),
                                    );

                                    additional_edits.push(TextEdit {
                                        range: doc.mapper.range(range_after).unwrap(),
                                        new_text: ",".into(),
                                    })
                                }
                            }

                            let current_token =
                                before.syntax.element.as_ref().unwrap().as_token().unwrap();

                            if current_token.kind() != SyntaxKind::WHITESPACE
                                && current_token.kind() != SyntaxKind::COMMA
                            {
                                comma_before = true;
                            }
                        }

                        return get_schema_objects(query_path.clone(), &root_schema, true)
                            .into_iter()
                            .map(|s| s.descendants(&root_schema.definitions, 10))
                            .flatten()
                            .filter(|(_, s, _)| !s.is_hidden())
                            .unique_by(|(p, ..)| p.clone())
                            .map(|(path, schema, required)| {
                                key_completion(
                                    &root_schema.definitions,
                                    if node.is_root() {
                                        query_path.extend(path)
                                    } else {
                                        path
                                    },
                                    schema,
                                    required,
                                    range,
                                    true,
                                    if !additional_edits.is_empty() {
                                        Some(additional_edits.clone())
                                    } else {
                                        None
                                    },
                                    comma_before,
                                )
                            })
                            .collect();
                    }
                    _ => {
                        // Look for an incomplete key.
                        if let Some(before_node) = before.nodes.last() {
                            if before_node.is_key() {
                                let query_path = before.path.skip_right(1);

                                let mut is_root = true;

                                for node in &before.nodes {
                                    match node {
                                        NodeRef::Table(t) => {
                                            if !t.is_pseudo() {
                                                is_root = false;
                                            }
                                        }
                                        _ => {}
                                    };
                                }

                                let range = before
                                    .syntax
                                    .range
                                    .map(|range| doc.mapper.range(range).unwrap());

                                return get_schema_objects(query_path.clone(), &root_schema, true)
                                    .into_iter()
                                    .map(|s| s.descendants(&root_schema.definitions, 10))
                                    .flatten()
                                    .filter(|(_, s, _)| !s.is_hidden())
                                    .unique_by(|(p, ..)| p.clone())
                                    .map(|(path, schema, required)| {
                                        key_completion(
                                            &root_schema.definitions,
                                            if is_root {
                                                // log_debug!("{}", query_path.extend(path.clone()).dotted());
                                                query_path.extend(path)
                                            } else {
                                                // log_debug!("{}", path.dotted());
                                                path
                                            },
                                            schema,
                                            required,
                                            range,
                                            false,
                                            None,
                                            false,
                                        )
                                    })
                                    .collect();
                            }
                        }
                    }
                }
            }
        }
        None => {
            // Start of the document
            let node = query.after.nodes.last().cloned().unwrap();

            match node {
                NodeRef::Root(_) => {
                    let mut query_path = query.after.path.clone();

                    query_path = query_path.skip_right(1);

                    let range = query
                        .after
                        .syntax
                        .range
                        .map(|range| doc.mapper.range(range).unwrap());

                    return get_schema_objects(query_path.clone(), &root_schema, true)
                        .into_iter()
                        .map(|s| s.descendants(&root_schema.definitions, 10))
                        .flatten()
                        .filter(|(_, s, _)| !s.is_hidden())
                        .unique_by(|(p, ..)| p.clone())
                        .map(|(path, schema, required)| {
                            key_completion(
                                &root_schema.definitions,
                                query_path.extend(path),
                                schema,
                                required,
                                range,
                                true,
                                None,
                                false,
                            )
                        })
                        .collect();
                }
                _ => {
                    // TODO handle more stuff
                }
            }
        }
    }

    return Vec::new();
}

fn detail_text(schema: Option<ExtendedSchema>, text: Option<&str>) -> Option<String> {
    if schema.is_none() && text.is_none() {
        return None;
    }

    let schema_title = schema
        .and_then(|o| o.schema.metadata.as_ref())
        .and_then(|meta| meta.title.clone())
        .unwrap_or_default();

    Some(format!(
        "{text}{schema}",
        schema = if schema_title.is_empty() {
            "".into()
        } else if text.is_none() {
            format!("({})", schema_title)
        } else {
            format!(" ({})", schema_title)
        },
        text = text.map(|t| t.to_string()).unwrap_or_default()
    ))
}

fn documentation(schema: ExtendedSchema) -> Option<Documentation> {
    schema
        .ext
        .docs
        .as_ref()
        .and_then(|docs| docs.main.as_ref())
        .map(|doc| {
            Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: doc.clone(),
            })
        })
        .or_else(|| {
            schema
                .schema
                .metadata
                .as_ref()
                .and_then(|meta| meta.description.clone())
                .map(|desc| {
                    Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: desc,
                    })
                })
        })
}

fn key_completion(
    defs: &Map<String, Schema>,
    path: dom::Path,
    schema: ExtendedSchema,
    required: bool,
    range: Option<Range>,
    eq: bool,
    additional_text_edits: Option<Vec<TextEdit>>,
    comma_before: bool,
) -> CompletionItem {
    let insert_text = if eq {
        with_comma(format!("{} = ", path.dotted()), comma_before)
    } else {
        with_comma(path.dotted(), comma_before)
    };

    CompletionItem {
        label: path.dotted(),
        additional_text_edits,
        sort_text: if required {
            Some(required_text(&path.dotted()))
        } else {
            None
        },
        text_edit: range.map(|range| {
            CompletionTextEdit::Edit(TextEdit {
                range,
                new_text: insert_text.clone(),
            })
        }),
        insert_text: Some(insert_text),
        kind: if schema.is_object() {
            Some(CompletionItemKind::Struct)
        } else if schema.is_array_of_objects(defs) {
            Some(CompletionItemKind::Struct)
        } else {
            Some(CompletionItemKind::Value)
        },
        detail: detail_text(
            Some(schema.clone()),
            if required { Some("required") } else { None },
        ),
        documentation: documentation(schema.clone()),
        preselect: Some(true),
        ..Default::default()
    }
}

fn with_comma(text: String, comma: bool) -> String {
    if comma {
        format!(", {}", text)
    } else {
        text
    }
}

// To make sure required completions are at the top, we prefix it
// with an invisible character
fn required_text(key: &str) -> String {
    format!("{}{}", 1 as char, key)
}
