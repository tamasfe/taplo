use crate::{
    analytics::{Key, PositionInfo},
    schema::{
        contains_type, get_schema_objects, object_contains_type, resolve_object_ref, resolve_ref,
        ExtMeta, ExtendedSchema, EXTENSION_KEY,
    },
    Document,
};
use dom::{Cast, Entries};
use lsp_types::*;
use rowan::TextSize;
use schemars::{
    schema::{InstanceType, ObjectValidation, RootSchema, Schema, SingleOrVec},
    Map,
};
use taplo::{
    dom::{self, Common},
    syntax::{SyntaxElement, SyntaxKind},
    util::SyntaxExt,
};

pub(crate) fn get_completions(
    doc: Document,
    position: Position,
    schema: RootSchema,
) -> Vec<CompletionItem> {
    let mut info = PositionInfo::new(doc, position);

    // Finding everything in the DOM is unreliable, so we also look for edge cases based
    // on only the syntax tree.
    //
    // This can be none only if the document is completely empty.
    if let Some(syntax_node) = info
        .doc
        .parse
        .clone()
        .into_syntax()
        .find_node_deep(info.offset, false)
    {
        // Incomplete dotted keys cannot be reliably retrieved
        // from the DOM.
        if let SyntaxKind::KEY = syntax_node.kind() {
            // If inside an ident, we already handle it.
            if info.ident_range.is_none() {
                if let Some(k) = dom::KeyNode::cast(SyntaxElement::Node(syntax_node)) {
                    info.keys
                        .extend(k.keys_str_stripped().map(|s| Key::Property(s.into())));
                    info.node = Some(k.into());
                }
            }
        } else if let SyntaxKind::ENTRY = syntax_node.kind() {
            let value_kind = info
                .node
                .as_ref()
                .map(|n| match n.kind() {
                    SyntaxKind::STRING
                    | SyntaxKind::MULTI_LINE_STRING
                    | SyntaxKind::STRING_LITERAL
                    | SyntaxKind::MULTI_LINE_STRING_LITERAL
                    | SyntaxKind::INTEGER
                    | SyntaxKind::INTEGER_HEX
                    | SyntaxKind::INTEGER_OCT
                    | SyntaxKind::INTEGER_BIN
                    | SyntaxKind::FLOAT
                    | SyntaxKind::BOOL
                    | SyntaxKind::DATE
                    | SyntaxKind::VALUE => true,
                    _ => false,
                })
                .unwrap_or(false);

            // Only if the value wasn't already found
            if !value_kind {
                // Check if it is after "=".
                if let Some(eq) = syntax_node.find(SyntaxKind::EQ) {
                    if info.offset >= eq.text_range().end() {
                        // It's a value

                        if let Some(key) = syntax_node
                            .find(SyntaxKind::KEY)
                            .and_then(dom::KeyNode::cast)
                        {
                            info.keys
                                .extend(key.keys_str_stripped().map(|s| Key::Property(s.into())));

                            info.node = Some(
                                dom::ValueNode::cast(syntax_node.find(SyntaxKind::VALUE).unwrap())
                                    .unwrap()
                                    .into(),
                            );
                        }
                    }
                }
            }
        }
    }

    log_debug!("node {:#?}", info.node);
    log_debug!("keys {:?}", info.keys);
    log_debug!("offset {:?}", info.offset);
    log_debug!("inside array {:?}", info.inside_array);

    let mut search_keys = info.keys.clone();

    if info.ident_range.is_some() {
        // The last key is incomplete
        search_keys.pop();
    };

    let schemas = get_schema_objects(search_keys, &schema);

    if info.not_completable {
        return Vec::new();
    }

    let node = match &info.node {
        Some(n) => n,
        None => return Vec::new(),
    };

    match node {
        dom::Node::Root(_) | dom::Node::Table(_) | dom::Node::Key(_) => {
            key_completions(&info, &schema.definitions, &schemas)
        }
        dom::Node::Array(_) => {
            if info.key_only {
                key_completions(&info, &schema.definitions, &schemas)
            } else {
                value_completions(&info, &schema.definitions, &schemas)
            }
        }
        dom::Node::Value(_) => value_completions(&info, &schema.definitions, &schemas),
        dom::Node::Entry(_) => panic!("either key or value is expected"),
    }
}

fn key_completions(
    info: &PositionInfo,
    defs: &Map<String, Schema>,
    schemas: &[ExtendedSchema],
) -> Vec<CompletionItem> {
    let single_schema = schemas.len() == 1;

    let entries = match info.node.as_ref().unwrap() {
        dom::Node::Root(r) => Some(r.entries()),
        dom::Node::Table(t) => Some(t.entries()),
        _ => None,
    };

    let mut completions = Vec::new();

    for schema in schemas {
        if let Some(obj) = &schema.schema.object {
            completions.extend(
                obj.properties
                    .iter()
                    .filter(|(_, mut prop_schema)| {
                        prop_schema = match resolve_ref(defs, prop_schema) {
                            Some(s) => s,
                            None => {
                                return false;
                            }
                        };

                        if info.table_header && !contains_type(InstanceType::Object, prop_schema) {
                            return false;
                        }

                        if info.table_array_header {
                            if !contains_type(InstanceType::Array, prop_schema) {
                                return false;
                            }

                            // We only complete it if we surely know that it contains
                            // objects.
                            if let Schema::Object(o) = prop_schema {
                                return o
                                    .array
                                    .as_ref()
                                    .map(|arr| match arr.items.as_ref() {
                                        Some(items) => match items {
                                            SingleOrVec::Single(s) => contains_type(
                                                InstanceType::Object,
                                                resolve_ref(defs, s).unwrap(),
                                            ),
                                            SingleOrVec::Vec(_) => false,
                                        },
                                        None => false,
                                    })
                                    .unwrap_or_default();
                            }
                        }

                        true
                    })
                    // .filter(|(key, _)| should_complete_key(*key, defs, schema.clone(), entries))
                    .filter_map(|(prop_key, prop_schema)| {
                        let required = is_required(prop_key, obj);

                        let current_schema = if single_schema {
                            None
                        } else {
                            Some(schema.clone())
                        };

                        match prop_schema {
                            Schema::Bool(b) => {
                                let text_edit = match info.ident_range {
                                    Some(ident_range) => Some(CompletionTextEdit::Edit(TextEdit {
                                        new_text: prop_key.clone(),
                                        range: info.doc.mapper.range(ident_range).unwrap(),
                                    })),
                                    None => None,
                                };

                                if *b {
                                    // We don't know anything about it.
                                    Some(CompletionItem {
                                        label: prop_key.clone(),
                                        kind: Some(CompletionItemKind::Variable),
                                        detail: detail_text(
                                            current_schema,
                                            if required { Some("required") } else { None },
                                        ),
                                        sort_text: sort_text(prop_key, required),
                                        text_edit,
                                        preselect: Some(true),
                                        ..Default::default()
                                    })
                                } else {
                                    // It's not even allowed.
                                    None
                                }
                            }
                            Schema::Object(prop_schema_obj) => {
                                let prop_schema_obj =
                                    match resolve_object_ref(defs, prop_schema_obj.into()) {
                                        Some(o) => o,
                                        None => return None,
                                    };

                                if prop_schema_obj.ext.hidden.unwrap_or_default() {
                                    return None;
                                }

                                let (insert_text, insert_text_format, text_edit) =
                                    insert_text(prop_key, info, prop_schema_obj.clone());

                                Some(CompletionItem {
                                    label: prop_key.clone(),
                                    kind: Some(CompletionItemKind::Variable),
                                    insert_text,
                                    insert_text_format,
                                    text_edit,
                                    sort_text: sort_text(prop_key, required),
                                    detail: detail_text(
                                        current_schema,
                                        if required { Some("required") } else { None },
                                    ),
                                    documentation: documentation(prop_schema_obj),
                                    preselect: Some(true),
                                    ..Default::default()
                                })
                            }
                        }
                    }),
            );
        }
    }

    completions
}

fn is_required(key: &str, obj: &ObjectValidation) -> bool {
    obj.required.iter().any(|k| k == key)
}

fn insert_text(
    key: &str,
    info: &PositionInfo,
    obj: ExtendedSchema,
) -> (
    Option<String>,
    Option<InsertTextFormat>,
    Option<CompletionTextEdit>,
) {
    let edit_range = info
        .ident_range
        .and_then(|ident_range| info.doc.mapper.range(ident_range));

    if info.key_only || object_contains_type(InstanceType::Object, obj.schema) {
        // Leave just the key so that
        // dotted keys can be easily used.

        match edit_range {
            Some(range) => (
                None,
                None,
                Some(CompletionTextEdit::Edit(TextEdit {
                    new_text: key.to_string(),
                    range,
                })),
            ),
            None => (None, None, None),
        }
    } else {
        match edit_range {
            Some(range) => (
                None,
                Some(InsertTextFormat::Snippet),
                Some(CompletionTextEdit::Edit(TextEdit {
                    new_text: format!("{} = {}", key, empty_value_snippet(obj)),
                    range,
                })),
            ),
            None => (
                Some(format!("{} = {}", key, empty_value_snippet(obj))),
                Some(InsertTextFormat::Snippet),
                None,
            ),
        }
    }
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

fn sort_text(key: &str, required: bool) -> Option<String> {
    if required {
        // Make sure that it's at the top, so we prefix it
        // with an invisible character
        Some(format!("{}{}", 1 as char, key))
    } else {
        None
    }
}

fn empty_value_snippet(schema: ExtendedSchema) -> String {
    if let Some(en) = &schema.schema.enum_values {
        if !en.is_empty() {
            return "$0".into();
        }
    }

    if let Some(def) = schema
        .schema
        .metadata
        .as_ref()
        .and_then(|meta| meta.default.as_ref())
    {
        return format!(r#"${{0:{}}}"#, def.to_string());
    }

    let ty = match &schema.schema.instance_type {
        Some(it) => match it {
            SingleOrVec::Single(s) => **s,
            SingleOrVec::Vec(v) => {
                let filtered = v
                    .iter()
                    .filter(|ty| **ty != InstanceType::Null)
                    .copied()
                    .collect::<Vec<InstanceType>>();

                if filtered.len() != 1 {
                    match schema
                        .schema
                        .metadata
                        .as_ref()
                        .and_then(|meta| meta.default.as_ref())
                    {
                        Some(default) => return default.to_string() + "$0",
                        None => return "$0".into(),
                    }
                }

                filtered[0]
            }
        },
        None => {
            return "$0".into();
        }
    };

    match ty {
        InstanceType::Object => "{$0}".into(),
        InstanceType::Array => "[$0]".into(),
        InstanceType::String => r#""$0""#.into(),
        _ => "$0".into(),
    }
}

fn value_completions(
    info: &PositionInfo,
    defs: &Map<String, Schema>,
    schemas: &[ExtendedSchema],
) -> Vec<CompletionItem> {
    let single_schema = schemas.len() == 1;
    let mut completions = Vec::new();

    for (schema_idx, schema) in schemas.iter().enumerate() {
        let current_schema = if single_schema {
            None
        } else {
            Some(schema.clone())
        };

        let ebt_ext: ExtMeta = schema
            .schema
            .extensions
            .get(EXTENSION_KEY)
            .and_then(|v| {
                serde_json::from_value(v.clone())
                    .map_err(|e| {
                        log_error!("invalid schema extension: {}", e);
                    })
                    .ok()
            })
            .unwrap_or_default();

        let docs = schema
            .schema
            .metadata
            .as_ref()
            .and_then(|meta| meta.description.clone())
            .map(|desc| {
                Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: desc,
                }))
            })
            .unwrap_or_default();

        let insert_range = info.node.as_ref().and_then(|n| match n {
            dom::Node::Value(v) => match v {
                dom::ValueNode::Bool(_)
                | dom::ValueNode::String(_)
                | dom::ValueNode::Integer(_)
                | dom::ValueNode::Float(_)
                | dom::ValueNode::Date(_)
                | dom::ValueNode::Invalid(_)
                | dom::ValueNode::Empty => info.doc.mapper.range(v.text_range()),
                _ => None,
            },
            _ => None,
        });

        if let Some(en) = &schema.schema.enum_values {
            completions.extend(en.iter().enumerate().map(|(idx, v)| {
                let insert_text = to_snippet(v.to_string());

                let text_edit = insert_range.map(|range| {
                    CompletionTextEdit::Edit(TextEdit {
                        range,
                        new_text: insert_text.clone(),
                    })
                });

                CompletionItem {
                    label: v.to_string(),
                    kind: Some(CompletionItemKind::EnumMember),
                    detail: detail_text(current_schema.clone(), None),
                    documentation: ebt_ext
                        .docs
                        .as_ref()
                        .and_then(|docs| docs.enum_values.as_ref())
                        .and_then(|v| v.get(idx).and_then(|o| o.clone()))
                        .map(|value| {
                            Documentation::MarkupContent(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value,
                            })
                        })
                        .or_else(|| docs.clone()),
                    insert_text: Some(insert_text),
                    insert_text_format: Some(InsertTextFormat::Snippet),
                    text_edit,
                    preselect: Some(true),
                    ..Default::default()
                }
            }));
            if schema_idx == 0 {
                // It's the "parent" schema, not any of its
                // subschemas.
                break;
            }
            continue;
        }

        if let Some(def) = &schema
            .schema
            .metadata
            .as_ref()
            .and_then(|meta| meta.default.as_ref())
        {
            let insert_text = to_snippet(def.to_string());

            let text_edit = insert_range.map(|range| {
                CompletionTextEdit::Edit(TextEdit {
                    range,
                    new_text: insert_text.clone(),
                })
            });

            completions.push(CompletionItem {
                label: def.to_string(),
                kind: Some(CompletionItemKind::Value),
                detail: detail_text(current_schema.clone(), Some("default")),
                insert_text: Some(insert_text),
                insert_text_format: Some(InsertTextFormat::Snippet),
                text_edit,
                documentation: ebt_ext
                    .docs
                    .as_ref()
                    .and_then(|docs| docs.default_value.as_ref())
                    .map(|value| {
                        Documentation::MarkupContent(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: value.clone(),
                        })
                    })
                    .or_else(|| docs.clone()),
                preselect: Some(true),
                ..Default::default()
            });
            if schema_idx == 0 {
                // It's the "parent" schema, not any of its
                // subschemas.
                break;
            }
            continue;
        }

        if let Some(it) = &schema.schema.instance_type {
            let tys = match it {
                SingleOrVec::Single(i) => vec![**i],
                SingleOrVec::Vec(v) => v.clone(),
            };

            for ty in tys {
                match ty {
                    InstanceType::Boolean => {
                        completions.push(CompletionItem {
                            label: "true".into(),
                            kind: Some(CompletionItemKind::Value),
                            detail: detail_text(current_schema.clone(), None),
                            insert_text: Some(to_snippet("true".into())),
                            insert_text_format: Some(InsertTextFormat::Snippet),
                            text_edit: insert_range.map(|range| {
                                CompletionTextEdit::Edit(TextEdit {
                                    range,
                                    new_text: to_snippet("true".into()),
                                })
                            }),
                            documentation: docs.clone(),
                            preselect: Some(true),
                            ..Default::default()
                        });

                        completions.push(CompletionItem {
                            label: "false".into(),
                            kind: Some(CompletionItemKind::Value),
                            detail: detail_text(current_schema.clone(), None),
                            insert_text: Some(to_snippet("false".into())),
                            insert_text_format: Some(InsertTextFormat::Snippet),
                            text_edit: insert_range.map(|range| {
                                CompletionTextEdit::Edit(TextEdit {
                                    range,
                                    new_text: to_snippet("false".into()),
                                })
                            }),
                            documentation: docs.clone(),
                            preselect: Some(true),
                            ..Default::default()
                        });
                    }
                    InstanceType::Object => {
                        completions.push(CompletionItem {
                            label: "{}".into(),
                            kind: Some(CompletionItemKind::Value),
                            detail: detail_text(current_schema.clone(), Some("empty table")),
                            insert_text: Some(r#"{$0}"#.into()),
                            insert_text_format: Some(InsertTextFormat::Snippet),
                            text_edit: insert_range.map(|range| {
                                CompletionTextEdit::Edit(TextEdit {
                                    range,
                                    new_text: r#"{$0}"#.into(),
                                })
                            }),
                            documentation: docs.clone(),
                            preselect: Some(true),
                            ..Default::default()
                        });
                    }
                    InstanceType::Array => {
                        if info.inside_array {
                            if let Some(arr) = &schema.schema.array {
                                if let Some(item) = &arr.items {
                                    if let SingleOrVec::Single(s) = item {
                                        if let Schema::Object(obj) = &**s {
                                            if let Some(o) =
                                                resolve_object_ref(defs, ExtendedSchema::from(obj))
                                            {
                                                completions.extend(value_completions(
                                                    info,
                                                    defs,
                                                    &[o],
                                                ))
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            completions.push(CompletionItem {
                                label: "[]".into(),
                                kind: Some(CompletionItemKind::Value),
                                detail: detail_text(current_schema.clone(), Some("empty array")),
                                insert_text: Some(r#"[$0]"#.into()),
                                insert_text_format: Some(InsertTextFormat::Snippet),
                                text_edit: insert_range.map(|range| {
                                    CompletionTextEdit::Edit(TextEdit {
                                        range,
                                        new_text: r#"[$0]"#.into(),
                                    })
                                }),
                                documentation: docs.clone(),
                                preselect: Some(true),
                                ..Default::default()
                            });
                        }
                    }
                    InstanceType::String => {
                        completions.push(CompletionItem {
                            label: r#""""#.into(),
                            kind: Some(CompletionItemKind::Value),
                            detail: detail_text(current_schema.clone(), Some("empty string")),
                            insert_text: Some(r#""$0""#.into()),
                            insert_text_format: Some(InsertTextFormat::Snippet),
                            text_edit: insert_range.map(|range| {
                                CompletionTextEdit::Edit(TextEdit {
                                    range,
                                    new_text: r#""$0""#.into(),
                                })
                            }),
                            documentation: docs.clone(),
                            preselect: Some(true),
                            ..Default::default()
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    completions
}

fn to_snippet(value: String) -> String {
    format!(r#"${{0:{}}}"#, value)
}

// Traverse the entire tree to see
// if the key should be completed or not.
//
// If there is no missing key left anywhere, we shouldn't show this suggestion.
fn should_complete_key(
    key: &str,
    defs: &Map<String, Schema>,
    schema: ExtendedSchema,
    entries: Option<&Entries>,
) -> bool {
    let resolved_schema = match resolve_object_ref(defs, schema.clone()) {
        Some(s) => s,
        None => return false,
    };

    match &resolved_schema.schema.object {
        Some(obj) => {
            let mut prop_schema = match (&obj.properties).get(key) {
                Some(s) => s,
                None => return false,
            };

            prop_schema = match resolve_ref(defs, prop_schema) {
                Some(s) => s,
                None => return false,
            };

            let obj_schema = match prop_schema {
                Schema::Object(o) => o,
                Schema::Bool(b) => return *b,
            };

            let inner_entries = entries.and_then(|en| {
                for e in en.iter() {
                    if e.key().keys_str_stripped().next().unwrap() == key {
                        if let dom::ValueNode::Table(t) = e.value() {
                            return Some(t.entries());
                        }
                    }
                }

                None
            });

            if let Some(obj_val) = &obj_schema.object {
                // for (k, _) in &obj_val.properties {
                // if should_complete_key(k, defs, schema.clone(), inner_entries) {
                //     return true;
                // }
                // TODO segfault here?
                // }
                true
            } else {
                entries
                    .map(|en| {
                        !en.iter()
                            .any(|e| e.key().keys_str_stripped().next().unwrap() == key)
                    })
                    .unwrap_or(false)
            }
        }
        None => false,
    }
}
