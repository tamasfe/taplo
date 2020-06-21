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

    let offset = doc
        .mapper
        .offset(position)
        .unwrap_or(doc.parse.green_node.text_len());

    let mut info = DomInfo {
        keys: Vec::new(),
        node: None,
        doc,
        offset,
        table_array_header: false,
        position,
        table_header: false,
    };

    get_dom_info(&dom.into(), &mut info);

    // Empty table headers ("[]") and empty table array headers ("[[]]") aren't
    // in the DOM.
    for n in info.doc.parse.clone().into_syntax().descendants() {
        if info
            .doc
            .mapper
            .position(n.text_range().start())
            .unwrap_or_default()
            .line
            == position.line
        {
            if n.kind() == SyntaxKind::TABLE_HEADER {
                info.table_header = true;
                break;
            }
            if n.kind() == SyntaxKind::TABLE_ARRAY_HEADER {
                info.table_array_header = true;
                break;
            }
        }
    }

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
    table_header: bool,
    table_array_header: bool,
    doc: Document,
    position: Position,
    offset: TextSize,
}

fn get_dom_info(node: &dom::Node, info: &mut DomInfo) {
    match node {
        dom::Node::Root(r) => {
            info.node = Some(r.clone().into());
            for entry in r.entries().iter() {
                if entry.text_range().contains(info.offset)
                    || info
                        .doc
                        .mapper
                        .position(entry.text_range().start())
                        .unwrap()
                        .line
                        == info.position.line
                {
                    info.keys.push(Key::Property(entry.key().full_key_string()));

                    get_dom_info(&entry.clone().into(), info);
                    break;
                }
            }
        }
        dom::Node::Table(t) => {
            info.node = Some(t.clone().into());
            for entry in t.entries().iter() {
                if entry.text_range().contains(info.offset)
                    || info
                        .doc
                        .mapper
                        .position(entry.text_range().start())
                        .unwrap()
                        .line
                        == info.position.line
                {
                    info.keys.push(Key::Property(entry.key().full_key_string()));

                    get_dom_info(&entry.clone().into(), info);
                    break;
                }
            }
        }
        dom::Node::Entry(e) => {
            if e.key().text_range().contains(info.offset) {
                get_dom_info(&e.key().clone().into(), info);
            } else if e.value().text_range().contains_inclusive(info.offset) {
                get_dom_info(&e.value().clone().into(), info);
            } else {
                // Everything after the eq is considered a value
                if let Some(eq) = e.token_eq_text_range() {
                    if info.offset >= eq.end() {
                        get_dom_info(&e.value().clone().into(), info);
                    } else {
                        info.table_header = true; // so that only the key is completed
                        get_dom_info(&e.key().clone().into(), info);
                    }
                } else {
                    // There's no eq, it's all key only.
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

    // see if it is part of an existing key ident like my_key => my_
    let is_part_of_key = match info.node.as_ref().unwrap() {
        dom::Node::Key(k) => {
            let mut part_of_ident = false;
            let mut contains_error = false;

            for t in k.syntax().descendants_with_tokens() {
                match t.kind() {
                    SyntaxKind::IDENT => {
                        if !part_of_ident {
                            part_of_ident = t.text_range().contains(info.offset);
                        }
                    }
                    SyntaxKind::ERROR => {
                        contains_error = true;
                    }
                    _ => {}
                }
            }

            if contains_error {
                info.table_header = true; // so that we complete the key only
            }

            part_of_ident
        }
        dom::Node::Table(t) => {
            if t.kind() == SyntaxKind::TABLE_HEADER {
                let mut part_of_ident = false;

                for tok in t.syntax().descendants_with_tokens() {
                    match tok.kind() {
                        SyntaxKind::IDENT => {
                            if tok.text_range().contains_inclusive(info.offset) {
                                part_of_ident = true;
                                break;
                            }
                        }

                        _ => {}
                    }
                }

                part_of_ident
            } else {
                false
            }
        }
        dom::Node::Array(arr) => {
            if arr.kind() == SyntaxKind::TABLE_ARRAY_HEADER {
                let mut part_of_ident = false;

                for tok in arr.syntax().children_with_tokens() {
                    match tok.kind() {
                        SyntaxKind::IDENT => {
                            if tok.text_range().contains_inclusive(info.offset) {
                                part_of_ident = true;
                                break;
                            }
                        }

                        _ => {}
                    }
                }

                part_of_ident
            } else {
                false
            }
        }
        _ => false,
    };

    if info.keys.is_empty() || (info.keys.len() == 1 && is_part_of_key) {
        match info.node.as_ref().unwrap() {
            dom::Node::Root(_r) => match &schema.object {
                Some(o) => object_completions(info, o),
                None => Vec::new(),
            },
            dom::Node::Table(t) => {
                if info.table_header {
                    if info.offset <= t.text_range().start() || info.offset >= t.text_range().end()
                    {
                        return Vec::new();
                    }
                }
                match &schema.object {
                    Some(o) => object_completions(info, o),
                    None => Vec::new(),
                }
            }
            dom::Node::Key(_) => match &schema.object {
                Some(o) => object_completions(info, o),
                None => Vec::new(),
            },
            dom::Node::Value(_) => value_completions(schema),
            dom::Node::Array(arr) => {
                if info.table_array_header {
                    // See if we're after the last period
                    let last_period_offset = arr
                        .syntax()
                        .descendants_with_tokens()
                        .filter(|n| n.kind() == SyntaxKind::PERIOD)
                        .last()
                        .map(|p| p.text_range().end())
                        .unwrap_or_default();

                    if last_period_offset != info.offset {
                        return Vec::new();
                    }

                    match &schema.object {
                        Some(o) => object_completions(info, o),
                        None => Vec::new(),
                    }
                } else {
                    if let Some(a) = schema.array.as_ref().and_then(|a| a.items.as_ref()) {
                        match a {
                            SingleOrVec::Single(s) => {
                                if let Schema::Object(o) = &**s {
                                    value_completions(o)
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
            dom::Node::Entry(e) => {
                let after_eq = e
                    .token_eq_text_range()
                    .map(|t| t.start() <= info.offset)
                    .unwrap_or_default();

                if !after_eq {
                    Vec::new()
                } else {
                    value_completions(schema)
                }
            }
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

fn object_completions(info: &mut DomInfo, obj: &Box<ObjectValidation>) -> Vec<CompletionItem> {
    let entries = match info.node.as_ref().unwrap() {
        dom::Node::Root(r) => Some(r.entries()),
        dom::Node::Table(t) => Some(t.entries()),
        _ => None,
    };

    if info.table_header {
        return obj
            .properties
            .iter()
            .filter(|(_, s)| match s {
                Schema::Bool(_) => true,
                Schema::Object(o) => {
                    if let Some(_) = &o.object {
                        true
                    } else {
                        false
                    }
                }
            })
            .filter_map(|(key, schema)| {
                if entries
                    .map(|es| {
                        es.iter()
                            .any(|e| e.key().keys_str().next().unwrap() == key.as_str())
                    })
                    .unwrap_or_default()
                {
                    return None;
                }

                match &schema {
                    Schema::Bool(b) => {
                        if *b {
                            Some(CompletionItem {
                                label: key.clone(),
                                kind: Some(CompletionItemKind::Variable),
                                preselect: Some(true),
                                ..Default::default()
                            })
                        } else {
                            None
                        }
                    }
                    Schema::Object(o) => {
                        if let Some(ty) = &o.instance_type {
                            match ty {
                                SingleOrVec::Single(t) => {
                                    if **t != InstanceType::Object {
                                        return None;
                                    }
                                }
                                SingleOrVec::Vec(tys) => {
                                    if !tys.contains(&InstanceType::Object) {
                                        return None;
                                    }
                                }
                            }
                        }

                        Some(CompletionItem {
                            label: key.clone(),
                            kind: Some(CompletionItemKind::Variable),
                            preselect: Some(true),
                            ..Default::default()
                        })
                    }
                }
            })
            .collect();
    }

    if info.table_array_header {
        return obj
            .properties
            .iter()
            .filter(|(_, s)| match s {
                Schema::Bool(_) => true,
                Schema::Object(o) => {
                    if let Some(arr) = &o.array {
                        if let Some(items) = &arr.items {
                            match items {
                                SingleOrVec::Single(s) => match &**s {
                                    Schema::Bool(_) => true,
                                    Schema::Object(o) => {
                                        if let Some(_) = &o.object {
                                            true
                                        } else {
                                            false
                                        }
                                    }
                                },
                                SingleOrVec::Vec(_) => false,
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            })
            .filter_map(|(key, schema)| {
                if entries
                    .map(|es| {
                        es.iter()
                            .any(|e| e.key().keys_str().next().unwrap() == key.as_str())
                    })
                    .unwrap_or_default()
                {
                    return None;
                }

                match &schema {
                    Schema::Bool(b) => {
                        if *b {
                            Some(CompletionItem {
                                label: key.clone(),
                                kind: Some(CompletionItemKind::Variable),
                                preselect: Some(true),
                                ..Default::default()
                            })
                        } else {
                            None
                        }
                    }
                    Schema::Object(o) => {
                        if let Some(ty) = &o.instance_type {
                            match ty {
                                SingleOrVec::Single(t) => {
                                    if **t != InstanceType::Object {
                                        return None;
                                    }
                                }
                                SingleOrVec::Vec(tys) => {
                                    if !tys.contains(&InstanceType::Object) {
                                        return None;
                                    }
                                }
                            }
                        }

                        Some(CompletionItem {
                            label: key.clone(),
                            kind: Some(CompletionItemKind::Variable),
                            preselect: Some(true),
                            ..Default::default()
                        })
                    }
                }
            })
            .collect();
    }

    obj.properties
        .iter()
        .filter_map(|(key, schema)| {
            if entries
                .map(|es| {
                    es.iter()
                        .any(|e| e.key().keys_str().next().unwrap() == key.as_str())
                })
                .unwrap_or_default()
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
