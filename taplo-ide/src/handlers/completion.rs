use crate::Document;
use lsp_types::*;
use rowan::TextSize;
use schemars::{
    schema::{InstanceType, ObjectValidation, RootSchema, Schema, SchemaObject, SingleOrVec},
    Map,
};
use taplo::{dom, syntax::SyntaxKind};

pub(crate) fn get_completions(
    doc: Document,
    position: Position,
    schema: RootSchema,
) -> Vec<CompletionItem> {
    let dom = doc.clone().parse.into_dom();

    let offset = doc.mapper.offset(position).unwrap();

    let mut info = DomInfo {
        keys: Vec::new(),
        node: None,
        doc,
        offset,
    };

    get_dom_info(&dom.into(), offset, &mut info);

    root_schema_completions(&mut info, &schema)
}

#[derive(Debug, Clone)]
enum Key {
    Index(usize),
    Property(String),
}

#[derive(Clone)]
struct DomInfo {
    keys: Vec<Key>,
    node: Option<dom::Node>,
    doc: Document,
    offset: TextSize,
}

fn get_dom_info(node: &dom::Node, offset: TextSize, info: &mut DomInfo) {
    if !node.text_range().contains(offset) {
        return;
    }

    match node {
        dom::Node::Root(r) => {
            info.node = Some(r.clone().into());
            for entry in r.entries().iter() {
                if entry.text_range().contains(offset) {
                    info.keys.push(Key::Property(entry.key().full_key_string()));

                    get_dom_info(&entry.clone().into(), offset, info);
                    break;
                }
            }
        }
        dom::Node::Table(t) => {
            info.node = Some(t.clone().into());
            for entry in t.entries().iter() {
                if entry.text_range().contains(offset) {
                    info.keys.push(Key::Property(entry.key().full_key_string()));

                    get_dom_info(&entry.clone().into(), offset, info);
                    break;
                }
            }
        }
        dom::Node::Entry(e) => {
            if e.key().text_range().contains(offset) {
                get_dom_info(&e.key().clone().into(), offset, info);
            } else if e.value().text_range().contains(offset) {
                get_dom_info(&e.value().clone().into(), offset, info);
            } else {
                log_debug!("{:?}", e.value().text_range());
                info.node = Some(e.clone().into());
            }
        }
        dom::Node::Key(k) => {
            info.node = Some(k.clone().into());
        }
        dom::Node::Value(v) => match v {
            dom::ValueNode::Array(a) => {
                get_dom_info(&a.clone().into(), offset, info);
            }
            dom::ValueNode::Table(t) => {
                get_dom_info(&t.clone().into(), offset, info);
            }
            _ => {
                info.node = Some(v.clone().into());
            }
        },
        dom::Node::Array(arr) => {
            info.node = Some(arr.clone().into());
            for (i, value) in arr.items().iter().enumerate() {
                if value.text_range().contains(offset) {
                    info.keys.push(Key::Index(i));
                    get_dom_info(&value.clone().into(), offset, info);
                    break;
                }
            }
        }
    }
}

fn root_schema_completions(info: &mut DomInfo, schema: &RootSchema) -> Vec<CompletionItem> {
    schema_object_completions(info, &schema.definitions, &schema.schema)
}

fn schema_completions(
    info: &mut DomInfo,
    defs: &Map<String, Schema>,
    schema: &Schema,
) -> Vec<CompletionItem> {
    match schema {
        Schema::Bool(_) => Vec::new(),
        Schema::Object(o) => schema_object_completions(info, defs, o),
    }
}

fn schema_object_completions(
    info: &mut DomInfo,
    defs: &Map<String, Schema>,
    schema: &SchemaObject,
) -> Vec<CompletionItem> {
    if let Some(r) = &schema.reference {
        if let Some(local_def) = local_definition(r) {
            if let Some(s) = defs.get(local_def) {
                return schema_completions(info, defs, s);
            }
        }
    }

    match info.node.as_ref().unwrap() {
        dom::Node::Root(_r) => match &schema.object {
            Some(o) => object_completions(info, defs, o),
            None => Vec::new(),
        },
        dom::Node::Table(t) => {
            if t.kind() == SyntaxKind::TABLE_HEADER
                && t.syntax().text_range().contains_inclusive(info.offset)
            {
                return Vec::new();
            }
            match &schema.object {
                Some(o) => object_completions(info, defs, o),
                None => Vec::new(),
            }
        }
        dom::Node::Key(_) => todo!(),
        dom::Node::Value(_) => todo!(),
        dom::Node::Array(arr) => {
            if arr.kind() == SyntaxKind::TABLE_ARRAY_HEADER
                && arr.syntax().text_range().contains_inclusive(info.offset)
            {
                return Vec::new();
            }

            todo!()
        }
        dom::Node::Entry(_) => todo!(),
    }
}

fn object_completions(
    info: &mut DomInfo,
    defs: &Map<String, Schema>,
    obj: &Box<ObjectValidation>,
) -> Vec<CompletionItem> {
    let entries = match info.node.as_ref().unwrap() {
        dom::Node::Root(r) => r.entries(),
        dom::Node::Table(t) => t.entries(),
        _ => panic!(),
    };

    if info.keys.is_empty() {
        obj.properties
            .iter()
            .filter_map(|(key, schema)| {
                if entries
                    .iter()
                    .any(|e| e.key().keys_str().next().unwrap() == key.as_str())
                {
                    None
                } else {
                    match schema {
                        Schema::Bool(b) => {
                            if *b {
                                // We don't know anything about it.
                                Some(CompletionItem {
                                    label: key.clone(),
                                    kind: Some(CompletionItemKind::Variable),
                                    insert_text: Some(format!("{} = $0", key)),
                                    insert_text_format: Some(InsertTextFormat::Snippet),
                                    preselect: Some(true),
                                    ..Default::default()
                                })
                            } else {
                                // It's not even allowed.
                                None
                            }
                        }
                        Schema::Object(obj) => Some(CompletionItem {
                            label: key.clone(),
                            kind: Some(CompletionItemKind::Variable),
                            insert_text: Some(format!("{} = {}", key, empty_value_snippet(obj))),
                            insert_text_format: Some(InsertTextFormat::Snippet),
                            documentation: obj
                                .metadata
                                .as_ref()
                                .and_then(|meta| meta.description.clone())
                                .map(|desc| {
                                    Some(Documentation::MarkupContent(MarkupContent {
                                        kind: MarkupKind::Markdown,
                                        value: desc,
                                    }))
                                })
                                .unwrap_or_default(),
                            preselect: Some(true),
                            ..Default::default()
                        }),
                    }
                }
            })
            .collect()
    } else {
        let key = info.keys.remove(0);

        if let Key::Property(k) = key {
            obj.properties
                .iter()
                .find(|(k2, _)| k == **k2)
                .map(|(_, s)| schema_completions(info, defs, s))
                .unwrap_or_default()
        } else {
            Vec::new()
        }
    }
}

fn empty_value_snippet(schema: &SchemaObject) -> String {
    let ty = match &schema.instance_type {
        Some(it) => match it {
            SingleOrVec::Single(s) => **s,
            SingleOrVec::Vec(v) => {
                let filtered = v
                    .iter()
                    .filter(|ty| **ty != InstanceType::Null)
                    .map(|ty| *ty)
                    .collect::<Vec<InstanceType>>();

                if filtered.len() != 1 {
                    match schema
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
        InstanceType::Object => match schema
            .metadata
            .as_ref()
            .and_then(|meta| meta.default.as_ref())
        {
            Some(default) => {
                return format!(
                    r#"${{0:{}}}"#,
                    trim_end(trim_start(&default.to_string(), "{"), "}").to_string(),
                )
            }
            None => "{ $0 }".into(),
        },
        InstanceType::Array => {
            match schema
                .metadata
                .as_ref()
                .and_then(|meta| meta.default.as_ref())
            {
                Some(default) => {
                    return format!(
                        r#"${{0:{}}}"#,
                        trim_end(trim_start(&default.to_string(), "["), "]").to_string(),
                    )
                }
                None => "[ $0 ]".into(),
            }
        }
        InstanceType::String => {
            match schema
                .metadata
                .as_ref()
                .and_then(|meta| meta.default.as_ref())
            {
                Some(default) => {
                    return format!(
                        r#""${{0:{}}}""#,
                        trim_end(trim_start(&default.to_string(), r#"""#), r#"""#).to_string(),
                    )
                }
                None => r#""$0""#.into(),
            }
        }
        _ => {
            match schema
                .metadata
                .as_ref()
                .and_then(|meta| meta.default.as_ref())
            {
                Some(default) => return format!(r#"${{0:{}}}"#, default.to_string()),
                None => "$0".into(),
            }
        }
    }
}

fn trim_start<'a>(val: &'a str, pat: &str) -> &'a str {
    if val.starts_with(pat) {
        &val[pat.len()..]
    } else {
        val
    }
}

fn trim_end<'a>(val: &'a str, pat: &str) -> &'a str {
    if val.ends_with(pat) {
        &val[..val.len() - pat.len()]
    } else {
        val
    }
}

fn local_definition(rf: &str) -> Option<&str> {
    if rf.starts_with("#/definitions/") {
        Some(rf.trim_start_matches("#/definitions/"))
    } else {
        None
    }
}
