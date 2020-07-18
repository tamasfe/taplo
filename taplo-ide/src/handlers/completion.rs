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
use schemars::{
    schema::{InstanceType, ObjectValidation, RootSchema, Schema, SchemaObject, SingleOrVec},
    Map,
};
use std::collections::HashSet;
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
        } else if let SyntaxKind::ENTRY | SyntaxKind::VALUE = syntax_node.kind() {
            // FIXME: this should be found in the DOM, but isn't, so we fix it here
            if let SyntaxKind::VALUE = syntax_node.kind() {
                if let Some(key) = syntax_node
                    .parent()
                    .unwrap()
                    .find(SyntaxKind::KEY)
                    .and_then(dom::KeyNode::cast)
                {
                    info.not_completable = false;
                    info.keys
                        .extend(key.keys_str_stripped().map(|s| Key::Property(s.into())));

                    info.node = Some(
                        dom::ValueNode::cast(syntax_node.clone().into())
                            .unwrap()
                            .into(),
                    );
                }
            }

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
                    if info.offset >= eq.text_range().start() {
                        // It's a value
                        if let Some(key) = syntax_node
                            .find(SyntaxKind::KEY)
                            .and_then(dom::KeyNode::cast)
                        {
                            info.not_completable = false;
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
    let entries = match info.node.as_ref().unwrap() {
        dom::Node::Root(r) => Some(r.entries()),
        dom::Node::Table(t) => Some(t.entries()),
        _ => None,
    };

    let mut completions = Vec::new();

    for schema in schemas {
        completions.extend(
            dotted_key_completions(
                info,
                defs,
                Vec::new(),
                String::new(),
                schema.clone(),
                entries,
                HashSet::new(),
            )
            .into_iter()
            .filter(|c| !info.table_header || c.table)
            .filter(|c| !(info.table_array_header && c.table))
            .map(|c| c.completion),
        );
    }

    completions
}

fn is_required(key: &str, obj: &ObjectValidation) -> bool {
    obj.required.iter().any(|k| k == key)
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

// Make sure that it's at the top, so we prefix it
// with an invisible character
fn required_text(key: &str) -> String {
    format!("{}{}", 1 as char, key)
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

#[derive(Debug)]
struct KeyCompletion {
    keys: Vec<Key>,
    table: bool,
    completion: CompletionItem,
}

/// Return all the completions examining
/// the deepest possible values
fn dotted_key_completions(
    info: &PositionInfo,
    defs: &Map<String, Schema>,
    keys: Vec<Key>,
    sort_text: String,
    schema: ExtendedSchema,
    entries: Option<&Entries>,
    mut visited_schemas: HashSet<*const SchemaObject>,
) -> Vec<KeyCompletion> {
    let mut comps: Vec<KeyCompletion> = Vec::new();

    // Prevents infinite recursion
    visited_schemas.insert(schema.schema as *const SchemaObject);

    let insert_range = info
        .ident_range
        .and_then(|range| info.doc.mapper.range(range));

    let full_key_string: String = format_keys(&keys);

    if let Some(obj) = &schema.schema.object {
        for (prop_key, prop_schema) in &obj.properties {
            if let Schema::Object(prop_obj) = prop_schema {
                let resolved_prop = ExtendedSchema::resolved(defs, prop_obj);

                if resolved_prop.ext.hidden.unwrap_or(false) {
                    continue;
                }

                let prop_entries = entries
                    .and_then(|en| {
                        en.iter()
                            .find(|e| e.key().keys_str_stripped().next().unwrap() == prop_key)
                    })
                    .and_then(|entry| match entry.value() {
                        dom::ValueNode::Table(t) => Some(t.entries()),
                        _ => None,
                    });

                if let Some(e) = prop_entries {
                        let exists = e
                            .iter()
                            .any(|entry| entry.key().keys_str_stripped().next().unwrap() == prop_key);
    
                        if resolved_prop.schema.object.is_none() && exists {
                            continue;
                        }
                    }

                let mut new_keys = keys.clone();
                new_keys.push(Key::Property(prop_key.clone()));
                let new_full_key_string = format_keys(&new_keys);

                let required = is_required(prop_key, obj);

                let new_sort_text = {
                    if required {
                        let mut nk = keys.clone();
                        nk.push(Key::Property(required_text(prop_key)));
                        format_keys(&nk)
                    } else {
                        new_full_key_string.clone()
                    }
                };

                if visited_schemas.contains(&(resolved_prop.schema as *const SchemaObject)) {
                    let insert_text = if info.table_header || info.key_only {
                        new_full_key_string.clone()
                    } else {
                        format!("{} = {{$0}}", new_full_key_string.clone())
                    };

                    comps.push(KeyCompletion {
                        keys: new_keys,
                        table: true,
                        completion: CompletionItem {
                            label: new_full_key_string.clone(),
                            kind: Some(CompletionItemKind::Struct),
                            insert_text_format: if info.table_header {
                                None
                            } else {
                                Some(InsertTextFormat::Snippet)
                            },
                            text_edit: insert_range.map(|range| {
                                CompletionTextEdit::Edit(TextEdit {
                                    range,
                                    new_text: insert_text.clone(),
                                })
                            }),
                            sort_text: Some(new_sort_text),
                            detail: detail_text(
                                Some(schema.clone()),
                                if required { Some("required") } else { None },
                            ),
                            documentation: documentation(schema.clone()),
                            preselect: Some(true),
                            ..Default::default()
                        },
                    });
                    continue;
                }

                let prop_completions = dotted_key_completions(
                    info,
                    defs,
                    new_keys.clone(),
                    new_sort_text,
                    resolved_prop.clone(),
                    prop_entries,
                    visited_schemas.clone(),
                );

                comps.extend(prop_completions.into_iter().map(|mut c| {
                    if required {
                        c.completion.detail = match c.completion.detail.take() {
                            Some(d) => Some(format!("required {}", d)),
                            None => Some("required".to_string()),
                        }
                    }

                    c
                }));
            }
        }

        let insert_text = if info.table_header || info.key_only {
            full_key_string.clone()
        } else {
            format!("{} = {{$0}}", full_key_string)
        };

        let additional_props_allowed = obj
            .additional_properties
            .as_ref()
            .map(|s| match resolve_ref(defs, &**s).unwrap() {
                Schema::Bool(b) => *b,
                Schema::Object(_) => true,
            })
            .unwrap_or(false)
            || !obj.pattern_properties.is_empty();

        if !comps.is_empty() || additional_props_allowed {
            let this_comp = KeyCompletion {
                keys,
                table: true,
                completion: CompletionItem {
                    label: full_key_string,
                    kind: Some(CompletionItemKind::Struct),
                    insert_text: Some(insert_text.clone()),
                    insert_text_format: if info.table_header {
                        None
                    } else {
                        Some(InsertTextFormat::Snippet)
                    },
                    text_edit: insert_range.map(|range| {
                        CompletionTextEdit::Edit(TextEdit {
                            range,
                            new_text: insert_text.clone(),
                        })
                    }),
                    detail: detail_text(Some(schema.clone()), None),
                    documentation: documentation(schema.clone()),
                    preselect: Some(true),
                    ..Default::default()
                },
            };

            comps.push(this_comp);
        }
    } else {
        if info.table_array_header {
            if !object_contains_type(InstanceType::Array, schema.schema) {
                return comps;
            }

            let array_of_objects = schema
                .schema
                .array
                .as_ref()
                .map(|arr| match arr.items.as_ref() {
                    Some(items) => match items {
                        SingleOrVec::Single(s) => {
                            contains_type(InstanceType::Object, resolve_ref(defs, s).unwrap())
                        }
                        SingleOrVec::Vec(_) => false,
                    },
                    None => false,
                })
                .unwrap_or_default();

            if !array_of_objects {
                return comps;
            }
        }

        if schema.schema.instance_type.is_none() {
            return comps;
        }

        let insert_text = if info.table_array_header || info.key_only {
            full_key_string.clone()
        } else {
            format!(
                "{} = {}",
                full_key_string,
                empty_value_snippet(schema.clone())
            )
        };

        comps.push(KeyCompletion {
            keys,
            table: false,
            completion: CompletionItem {
                label: full_key_string,
                kind: Some(CompletionItemKind::Variable),
                insert_text: Some(insert_text.clone()),
                insert_text_format: if info.table_header {
                    None
                } else {
                    Some(InsertTextFormat::Snippet)
                },
                text_edit: insert_range.map(|range| {
                    CompletionTextEdit::Edit(TextEdit {
                        range,
                        new_text: insert_text.clone(),
                    })
                }),
                sort_text: if sort_text.is_empty() {
                    None
                } else {
                    Some(sort_text)
                },
                detail: detail_text(Some(schema.clone()), None),
                documentation: documentation(schema.clone()),
                preselect: Some(true),
                ..Default::default()
            },
        });
    }

    comps
}

fn format_keys(keys: &[Key]) -> String {
    keys.iter()
        .map(|k| k.to_string())
        .collect::<Vec<String>>()
        .join(".")
}
