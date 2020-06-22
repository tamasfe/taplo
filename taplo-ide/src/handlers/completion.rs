use crate::Document;
use lsp_types::*;
use rowan::TextSize;
use schemars::{
    schema::{InstanceType, ObjectValidation, RootSchema, Schema, SchemaObject, SingleOrVec},
    Map,
};
use taplo::{
    dom::{self, Common},
    syntax::{SyntaxKind, SyntaxNode},
};

pub(crate) fn get_completions(
    doc: Document,
    position: Position,
    schema: RootSchema,
) -> Vec<CompletionItem> {
    let dom = doc.clone().parse.into_dom();

    let offset = doc
        .mapper
        .offset(position)
        .unwrap_or(doc.parse.green_node.text_len());

    let mut info = DomInfo {
        keys: Vec::new(),
        node: None,
        doc,
        offset,
        position,
        key_only: false,
        table_header: false,
        table_array_header: false,
        part_of_ident: false,
    };

    get_dom_info(&dom.into(), &mut info);

    // Empty table headers ("[]") and empty table array headers ("[[]]") aren't
    // in the DOM.
    for n in info.doc.parse.clone().into_syntax().descendants() {
        if n.text_range().contains(info.offset)
            && (n.kind() == SyntaxKind::TABLE_HEADER || n.kind() == SyntaxKind::TABLE_ARRAY_HEADER)
        {
            if n.kind() == SyntaxKind::TABLE_HEADER {
                info.table_header = true;
            } else {
                info.table_array_header = true;
            }

            // No completion outside the headers
            if !inside_header_brackets(n, offset) {
                return Vec::new();
            }

            info.key_only = true;

            break;
        }
    }

    info.part_of_ident = info
        .node
        .as_ref()
        .map(|n| part_of_ident(n, offset))
        .unwrap_or_default();

    root_schema_completions(&mut info, &schema)
}

#[derive(Debug, Clone)]
enum Key {
    Index(usize),
    Property(String),
}

#[derive(Debug)]
struct DomInfo {
    keys: Vec<Key>,
    node: Option<dom::Node>,
    key_only: bool,
    table_header: bool,
    table_array_header: bool,
    part_of_ident: bool,
    doc: Document,
    position: Position,
    offset: TextSize,
}

fn get_dom_info(node: &dom::Node, info: &mut DomInfo) {
    match node {
        dom::Node::Root(r) => {
            info.node = Some(r.clone().into());
            for entry in r.entries().iter() {
                if entry.text_range().contains(info.offset) {
                    info.keys.push(Key::Property(entry.key().full_key_string()));

                    get_dom_info(&entry.clone().into(), info);
                    break;
                }
            }
        }
        dom::Node::Table(t) => {
            info.node = Some(t.clone().into());
            for entry in t.entries().iter() {
                if entry.text_range().contains(info.offset) {
                    info.keys.push(Key::Property(entry.key().full_key_string()));

                    get_dom_info(&entry.clone().into(), info);
                    break;
                }
            }
        }
        dom::Node::Entry(e) => {
            if e.key().text_range().contains(info.offset) {
                get_dom_info(&e.key().clone().into(), info);
            } else if e.value().text_range().contains(info.offset) {
                get_dom_info(&e.value().clone().into(), info);
            } else {
                // Everything after the eq is considered a value
                if let Some(eq) = e.token_eq_text_range() {
                    if info.offset >= eq.end() {
                        get_dom_info(&e.value().clone().into(), info);
                    } else {
                        info.key_only = true;
                        get_dom_info(&e.key().clone().into(), info);
                    }
                } else if e.value().is_valid() {
                    // It's a table header or array of tables header.
                    get_dom_info(&e.value().clone().into(), info);
                } else {
                    // There's no eq and it's a regular entry, it's all key only.
                    get_dom_info(&e.key().clone().into(), info);
                }
            }
        }
        dom::Node::Key(k) => {
            info.node = Some(k.clone().into());
        }
        dom::Node::Value(v) => match v {
            dom::ValueNode::Array(a) => {
                get_dom_info(&a.clone().into(), info);
            }
            dom::ValueNode::Table(t) => {
                get_dom_info(&t.clone().into(), info);
            }
            _ => {
                info.node = Some(v.clone().into());
            }
        },
        dom::Node::Array(arr) => {
            info.node = Some(arr.clone().into());
            for (i, value) in arr.items().iter().enumerate() {
                if value.text_range().contains(info.offset) {
                    info.keys.push(Key::Index(i));
                    get_dom_info(&value.clone().into(), info);
                    break;
                }
            }
        }
    }
}

fn part_of_ident(node: &dom::Node, offset: TextSize) -> bool {
    match node {
        dom::Node::Key(k) => {
            for t in k.syntax().as_node().unwrap().descendants_with_tokens() {
                match t.kind() {
                    SyntaxKind::IDENT => {
                        if t.text_range().contains(offset) {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
            false
        }
        dom::Node::Table(t) => {
            if t.syntax().kind() == SyntaxKind::TABLE_HEADER
                || t.syntax().kind() == SyntaxKind::TABLE_ARRAY_HEADER
            {
                for tok in t.syntax().as_node().unwrap().descendants_with_tokens() {
                    match tok.kind() {
                        SyntaxKind::IDENT => {
                            if tok.text_range().contains(offset) {
                                return true;
                            }
                        }

                        _ => {}
                    }
                }
            }
            false
        }
        dom::Node::Array(arr) => {
            if arr.syntax().kind() == SyntaxKind::TABLE_ARRAY_HEADER {
                for tok in arr.syntax().as_node().unwrap().children_with_tokens() {
                    match tok.kind() {
                        SyntaxKind::IDENT => {
                            if tok.text_range().contains(offset) {
                                return true;
                            }
                        }

                        _ => {}
                    }
                }
            }
            false
        }
        _ => false,
    }
}

fn inside_header_brackets(node: SyntaxNode, offset: TextSize) -> bool {
    let last_start_bracket = node
        .descendants_with_tokens()
        .filter(|d| d.kind() == SyntaxKind::BRACE_START)
        .last()
        .map(|n| n.text_range())
        .unwrap_or_default();

    let first_ending_bracket = node
        .descendants_with_tokens()
        .find(|d| d.kind() == SyntaxKind::BRACKET_END)
        .map(|n| n.text_range())
        .unwrap_or_default();

    offset >= last_start_bracket.start() && offset < first_ending_bracket.start()
}

fn root_schema_completions(info: &mut DomInfo, schema: &RootSchema) -> Vec<CompletionItem> {
    schema_object_completions(info, &schema.definitions, &schema.schema)
}

fn schema_completions<'s>(
    info: &mut DomInfo,
    defs: &'s Map<String, Schema>,
    mut schema: &'s Schema,
) -> Vec<CompletionItem> {
    schema = resolve_ref(defs, schema).unwrap();

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
    if info.keys.is_empty() || (info.keys.len() == 1 && info.part_of_ident) {
        match info.node.as_ref().unwrap() {
            dom::Node::Root(_) | dom::Node::Table(_) | dom::Node::Key(_) => match &schema.object {
                Some(o) => object_completions(info, defs, o),
                None => Vec::new(),
            },
            dom::Node::Value(_) => value_completions(schema),
            dom::Node::Array(_) => {
                if info.key_only {
                    match &schema.object {
                        Some(o) => object_completions(info, defs, o),
                        None => Vec::new(),
                    }
                } else {
                    if let Some(a) = schema.array.as_ref().and_then(|a| a.items.as_ref()) {
                        match a {
                            SingleOrVec::Single(s) => {
                                if let Schema::Object(o) = &**s {
                                    if info.table_array_header {
                                        if let Some(items) =
                                            o.array.as_ref().and_then(|arr| arr.items.as_ref())
                                        {
                                            match items {
                                                SingleOrVec::Single(s) => {
                                                    let s = resolve_ref(defs, s).unwrap();
                                                    if let Schema::Object(items_obj) = s {
                                                        if let Some(obj_val) = &items_obj.object {
                                                            object_completions(info, defs, obj_val)
                                                        } else {
                                                            Vec::new()
                                                        }
                                                    } else {
                                                        Vec::new()
                                                    }
                                                }
                                                SingleOrVec::Vec(_) => Vec::new(),
                                            }
                                        } else {
                                            Vec::new()
                                        }
                                    } else {
                                        value_completions(resolve_object_ref(defs, o).unwrap())
                                    }
                                } else {
                                    Vec::new()
                                }
                            }
                            SingleOrVec::Vec(_schemas) => Vec::new(),
                        }
                    } else {
                        Vec::new()
                    }
                }
            }
            dom::Node::Entry(_) => panic!("this should never happen"),
        }
    } else {
        let key = info.keys.remove(0);

        match key {
            Key::Property(k) => schema
                .object
                .as_ref()
                .and_then(|obj| {
                    obj.properties
                        .iter()
                        .find(|(k2, _)| k == **k2)
                        .map(|(_, s)| schema_completions(info, defs, s))
                })
                .unwrap_or_default(),
            Key::Index(i) => schema
                .array
                .as_ref()
                .and_then(|arr| {
                    arr.items.as_ref().map(|s| match s {
                        SingleOrVec::Single(s) => schema_completions(info, defs, s),
                        SingleOrVec::Vec(schemas) => schemas
                            .get(i)
                            .map(|s| schema_completions(info, defs, s))
                            .unwrap_or_default(),
                    })
                })
                .unwrap_or_default(),
        }
    }
}

fn object_completions(
    info: &mut DomInfo,
    defs: &Map<String, Schema>,
    obj: &Box<ObjectValidation>,
) -> Vec<CompletionItem> {
    let entries = match info.node.as_ref().unwrap() {
        dom::Node::Root(r) => Some(r.entries()),
        dom::Node::Table(t) => Some(t.entries()),
        _ => None,
    };

    obj.properties
        .iter()
        .filter(|(_, mut schema)| {
            schema = resolve_ref(defs, schema).unwrap();

            if info.table_header && !contains_type(InstanceType::Object, schema) {
                return false;
            }

            if info.table_array_header {
                if !contains_type(InstanceType::Array, schema) {
                    return false;
                }

                if let Schema::Object(o) = schema {
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
        .filter(|(key, _)| {
            !entries
                .map(|es| {
                    es.iter()
                        .any(|e| e.key().keys_str().next().unwrap() == *key)
                })
                .unwrap_or_default()
        })
        .filter_map(|(key, schema)| {
            match schema {
                Schema::Bool(b) => {
                    if *b {
                        // We don't know anything about it.
                        Some(CompletionItem {
                            label: key.clone(),
                            kind: Some(CompletionItemKind::Variable),
                            sort_text: obj_sort_text(key, obj),
                            preselect: Some(true),
                            ..Default::default()
                        })
                    } else {
                        // It's not even allowed.
                        None
                    }
                }
                Schema::Object(o) => {
                    let o = resolve_object_ref(defs, o).unwrap();

                    let (insert_text, insert_text_format) = obj_insert_text(key, info, o);

                    Some(CompletionItem {
                        label: key.clone(),
                        kind: Some(CompletionItemKind::Variable),
                        insert_text,
                        insert_text_format,
                        sort_text: obj_sort_text(key, obj),
                        documentation: obj_doc(o),
                        preselect: Some(true),
                        ..Default::default()
                    })
                }
            }
        })
        .collect()
}

fn obj_insert_text(
    key: &str,
    info: &DomInfo,
    obj: &SchemaObject,
) -> (Option<String>, Option<InsertTextFormat>) {
    if info.key_only || object_contains_type(InstanceType::Object, obj) {
        // Leave just the key so that
        // dotted keys can be easily used.
        (None, None)
    } else {
        (
            Some(format!("{} = {}", key, empty_value_snippet(obj))),
            Some(InsertTextFormat::Snippet),
        )
    }
}

fn obj_doc(obj: &SchemaObject) -> Option<Documentation> {
    obj.metadata
        .as_ref()
        .and_then(|meta| meta.description.clone())
        .map(|desc| {
            Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: desc,
            })
        })
}

fn obj_sort_text(key: &str, obj: &ObjectValidation) -> Option<String> {
    let required = obj.required.iter().any(|k| k == key);

    if required {
        // Make sure that it's at the top, so we prefix it
        // with an invisible character
        Some(format!("{}{}", 1 as char, key))
    } else {
        None
    }
}

fn value_completions(obj: &SchemaObject) -> Vec<CompletionItem> {
    // TODO
    let documentation = obj
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

    if let Some(en) = &obj.enum_values {
        return en
            .iter()
            .map(|v| CompletionItem {
                label: v.to_string(),
                kind: Some(CompletionItemKind::EnumMember),
                documentation: documentation.clone(),
                preselect: Some(true),
                ..Default::default()
            })
            .collect();
    }

    // todo consts

    if let Some(def) = &obj.metadata.as_ref().and_then(|meta| meta.default.as_ref()) {
        return vec![CompletionItem {
            label: def.to_string(),
            kind: Some(CompletionItemKind::Value),
            detail: Some("default value".into()),
            documentation: documentation.clone(),
            preselect: Some(true),
            ..Default::default()
        }];
    }

    let ty = match &obj.instance_type {
        Some(it) => match it {
            SingleOrVec::Single(s) => **s,
            SingleOrVec::Vec(v) => {
                let filtered = v
                    .iter()
                    .filter(|ty| **ty != InstanceType::Null)
                    .map(|ty| *ty)
                    .collect::<Vec<InstanceType>>();

                if filtered.len() != 1 {
                    match obj.metadata.as_ref().and_then(|meta| meta.default.as_ref()) {
                        Some(default) => {
                            return vec![CompletionItem {
                                label: default.to_string(),
                                kind: Some(CompletionItemKind::EnumMember),
                                documentation: documentation.clone(),
                                preselect: Some(true),
                                ..Default::default()
                            }];
                        }
                        None => return Vec::new(),
                    }
                }

                filtered[0]
            }
        },
        None => {
            return Vec::new();
        }
    };

    match ty {
        InstanceType::Boolean => vec![
            CompletionItem {
                label: "true".to_string(),
                kind: Some(CompletionItemKind::Value),
                preselect: Some(true),
                ..Default::default()
            },
            CompletionItem {
                label: "false".to_string(),
                kind: Some(CompletionItemKind::Value),
                preselect: Some(true),
                ..Default::default()
            },
        ],
        InstanceType::Object => vec![CompletionItem {
            label: r#"{}"#.into(),
            insert_text_format: Some(InsertTextFormat::Snippet),
            insert_text: Some(r#"{ $0 }"#.into()),
            kind: Some(CompletionItemKind::Value),
            detail: Some("new empty table".into()),
            preselect: Some(true),
            ..Default::default()
        }],
        InstanceType::Array => vec![CompletionItem {
            label: r#"[]"#.into(),
            insert_text_format: Some(InsertTextFormat::Snippet),
            insert_text: Some(r#"[ $0 ]"#.into()),
            kind: Some(CompletionItemKind::Value),
            detail: Some("new empty array".into()),
            preselect: Some(true),
            ..Default::default()
        }],
        InstanceType::Number => Vec::new(),
        InstanceType::String => vec![CompletionItem {
            label: r#""""#.into(),
            insert_text_format: Some(InsertTextFormat::Snippet),
            insert_text: Some(r#""$0""#.into()),
            kind: Some(CompletionItemKind::Value),
            detail: Some("new empty string".into()),
            preselect: Some(true),
            ..Default::default()
        }],
        InstanceType::Integer => Vec::new(),
        _ => panic!("null value"),
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
                    .copied()
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

fn resolve_ref<'s>(defs: &'s Map<String, Schema>, schema: &'s Schema) -> Option<&'s Schema> {
    match schema {
        Schema::Bool(_) => Some(schema),
        Schema::Object(o) => {
            if let Some(r) = &o.reference {
                local_definition(r)
                    .and_then(|local_def| defs.get(local_def).and_then(|s| resolve_ref(defs, s)))
            } else {
                Some(schema)
            }
        }
    }
}

fn resolve_object_ref<'s>(
    defs: &'s Map<String, Schema>,
    obj: &'s SchemaObject,
) -> Option<&'s SchemaObject> {
    if let Some(r) = &obj.reference {
        local_definition(r).and_then(|local_def| {
            defs.get(local_def).and_then(|s| match s {
                Schema::Bool(_) => None,
                Schema::Object(o) => resolve_object_ref(defs, o),
            })
        })
    } else {
        Some(obj)
    }
}

fn contains_type(ty: InstanceType, schema: &Schema) -> bool {
    match schema {
        Schema::Bool(b) => *b,
        Schema::Object(obj) => object_contains_type(ty, obj),
    }
}

fn object_contains_type(ty: InstanceType, obj: &SchemaObject) -> bool {
    obj.instance_type
        .as_ref()
        .map(|tys| match tys {
            SingleOrVec::Single(tp) => **tp == ty,
            SingleOrVec::Vec(tps) => tps.contains(&ty),
        })
        .unwrap_or_default()
}
