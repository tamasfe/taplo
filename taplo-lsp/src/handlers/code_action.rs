use std::collections::HashMap;

use crate::{utils::LspExt, World};
use lsp_async_stub::{rpc::Error, Context, Params};
use lsp_types::*;
use rowan::TextRange;
use taplo::{
    analytics::{NodeRef, PositionInfo},
    dom::{ArrayNode, Entries, EntryNode, NodeSyntax, TableNode, ValueNode},
    formatter,
    syntax::SyntaxKind,
    util::{coords::Mapper, syntax::join_ranges},
};

pub(crate) async fn code_action(
    mut context: Context<World>,
    params: Params<CodeActionParams>,
) -> Result<Option<CodeActionResponse>, Error> {
    let p = params.required()?;

    let w = context.world().lock().await;
    let doc = w
        .documents
        .get(&p.text_document.uri)
        .ok_or_else(Error::invalid_params)?
        .clone();
    let format_opts_cfg = w.configuration.formatter.clone();
    drop(w);

    let mut format_opts = formatter::Options::default();
    format_opts.update_camel(format_opts_cfg);

    if p.range.start != p.range.end {
        return Ok(Some(Vec::new()));
    }

    let range = doc
        .mapper
        .text_range(taplo::util::coords::Range::from_lsp(p.range))
        .unwrap();

    let dom = doc.parse.into_dom();

    let query = dom.query_position(range.start());

    let mut actions = Vec::new();

    actions_for_position(
        query.after,
        &mut actions,
        format_opts.clone(),
        &doc.mapper,
        &p,
    );

    if !actions.is_empty() {
        return Ok(Some(actions));
    }

    if let Some(pos) = query.before {
        actions_for_position(pos, &mut actions, format_opts.clone(), &doc.mapper, &p);
    }

    return Ok(Some(actions));
}

fn actions_for_position(
    pos: PositionInfo,
    actions: &mut Vec<CodeActionOrCommand>,
    format_opts: formatter::Options,
    mapper: &Mapper,
    params: &CodeActionParams,
) {
    let mut node_iter = pos.nodes.into_iter().rev();

    if let Some(NodeRef::Key(_)) = node_iter.next() {
        if let Some(NodeRef::Entry(entry)) = node_iter.next() {
            match entry.value() {
                ValueNode::Table(table) => {
                    if table.is_inline() {
                        if let Some((parent_path, _, parent_entries)) = parent_table(node_iter) {
                            let mut edits = Vec::new();
                            extract_inline_table(
                                &mut edits,
                                format_opts.clone(),
                                mapper,
                                parent_path
                                    .map(|p| {
                                        format!("{}", p)
                                            + "."
                                            + &format!("{}", entry.key().syntax())
                                    })
                                    .unwrap_or_else(|| format!("{}", entry.key().syntax())),
                                parent_entries,
                                entry,
                                table.entries(),
                            );

                            let mut changes = HashMap::new();

                            changes.insert(params.text_document.uri.clone(), edits);

                            let action = CodeAction {
                                title: format!("Convert to table"),
                                kind: Some(CodeActionKind::REFACTOR),
                                edit: Some(WorkspaceEdit {
                                    changes: Some(changes),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            };

                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                    }
                }
                ValueNode::Array(arr) => {
                    if !arr.is_array_of_tables() {
                        let array_of_inline_tables = !arr.items().is_empty()
                            && arr.items().iter().all(|it| match it {
                                ValueNode::Table(t) => t.is_inline(),
                                _ => false,
                            });

                        if array_of_inline_tables {
                            if let Some((parent_path, _, parent_entries)) = parent_table(node_iter)
                            {
                                let mut edits = Vec::new();
                                extract_table_of_arrays(
                                    &mut edits,
                                    format_opts.clone(),
                                    mapper,
                                    parent_path
                                        .map(|p| {
                                            format!("{}", p)
                                                + "."
                                                + &format!("{}", entry.key().syntax())
                                        })
                                        .unwrap_or_else(|| format!("{}", entry.key().syntax())),
                                    parent_entries,
                                    entry,
                                    arr,
                                );

                                let mut changes = HashMap::new();

                                changes.insert(params.text_document.uri.clone(), edits);

                                let action = CodeAction {
                                    title: format!("Convert to array of tables"),
                                    kind: Some(CodeActionKind::REFACTOR),
                                    edit: Some(WorkspaceEdit {
                                        changes: Some(changes),
                                        ..Default::default()
                                    }),
                                    ..Default::default()
                                };

                                actions.push(CodeActionOrCommand::CodeAction(action));
                            }
                        }
                    }
                }
                _ => {}
            }
        } else if let Some(NodeRef::Value(value)) = node_iter.next() {
            if has_array_of_tables(value) {
                return;
            }
            if let ValueNode::Table(table) = value {
                let next = node_iter.next();
                if let Some(NodeRef::Array(arr)) = next {
                    node_iter.next();
                    let entry = match node_iter.next() {
                        Some(NodeRef::Entry(entry)) => entry,
                        _ => return,
                    };
                    if let Some((parent_key, parent_range, parent_entries)) =
                        parent_table(node_iter)
                    {
                        let table_key = parent_key
                            .map(|pk| {
                                entry
                                    .key()
                                    .syntax()
                                    .to_string()
                                    .trim_start_matches(&pk)
                                    .trim_start_matches('.')
                                    .to_string()
                            })
                            .unwrap_or_else(|| entry.key().syntax().to_string());

                        let mut edits = Vec::new();
                        inline_array_of_tables(
                            &mut edits,
                            format_opts,
                            mapper,
                            parent_range,
                            parent_entries,
                            table_key,
                            arr,
                        );

                        let mut changes = HashMap::new();
                        changes.insert(params.text_document.uri.clone(), edits);

                        let action = CodeAction {
                            title: format!("Convert to array"),
                            kind: Some(CodeActionKind::REFACTOR),
                            edit: Some(WorkspaceEdit {
                                changes: Some(changes),
                                ..Default::default()
                            }),
                            ..Default::default()
                        };

                        actions.push(CodeActionOrCommand::CodeAction(action));
                    }
                } else if let Some(NodeRef::Entry(entry)) = next {
                    if let Some((parent_key, parent_range, parent_entries)) =
                        parent_table(node_iter)
                    {
                        let mut edits = Vec::new();
                        let table_key = parent_key
                            .map(|pk| {
                                entry
                                    .key()
                                    .syntax()
                                    .to_string()
                                    .trim_start_matches(&pk)
                                    .trim_start_matches('.')
                                    .to_string()
                            })
                            .unwrap_or_else(|| entry.key().syntax().to_string());
                        inline_table(
                            &mut edits,
                            format_opts,
                            mapper,
                            parent_range,
                            parent_entries,
                            table_key,
                            table,
                        );

                        let mut changes = HashMap::new();
                        changes.insert(params.text_document.uri.clone(), edits);

                        let action = CodeAction {
                            title: format!("Convert to inline table"),
                            kind: Some(CodeActionKind::REFACTOR),
                            edit: Some(WorkspaceEdit {
                                changes: Some(changes),
                                ..Default::default()
                            }),
                            ..Default::default()
                        };

                        actions.push(CodeActionOrCommand::CodeAction(action));
                    }
                }
            } else if let ValueNode::Array(arr) = value {
                if arr.is_array_of_tables() {
                    if let (
                        Some(NodeRef::Entry(entry)),
                        Some((parent_key, parent_range, parent_entries)),
                    ) = (node_iter.next(), parent_table(node_iter))
                    {
                        let mut edits = Vec::new();
                        let table_key = parent_key
                            .map(|pk| {
                                entry
                                    .key()
                                    .syntax()
                                    .to_string()
                                    .trim_start_matches(&pk)
                                    .trim_start_matches('.')
                                    .to_string()
                            })
                            .unwrap_or_else(|| entry.key().syntax().to_string());
                        inline_array_of_tables(
                            &mut edits,
                            format_opts,
                            mapper,
                            parent_range,
                            parent_entries,
                            table_key,
                            arr,
                        );

                        let mut changes = HashMap::new();
                        changes.insert(params.text_document.uri.clone(), edits);

                        let action = CodeAction {
                            title: format!("Convert to inline table"),
                            kind: Some(CodeActionKind::REFACTOR),
                            edit: Some(WorkspaceEdit {
                                changes: Some(changes),
                                ..Default::default()
                            }),
                            ..Default::default()
                        };

                        actions.push(CodeActionOrCommand::CodeAction(action));
                    }
                }
            }
        }
    }
}

fn extract_inline_table(
    edits: &mut Vec<TextEdit>,
    format_opts: formatter::Options,
    mapper: &Mapper,
    path: String,
    parent_entries: &Entries,
    entry: &EntryNode,
    entries: &Entries,
) {
    let mut formatted = format_extracted_table(path.trim(), entries, false);
    formatted = formatter::format(&formatted, format_opts)
        .trim_end()
        .to_string();

    if let (range_before, Some(last_range)) = extract_replace_ranges(entry, parent_entries) {
        let insert_pos = mapper.position(last_range.end()).unwrap().into_lsp();

        let insert_range = Range {
            start: Position {
                line: insert_pos.line + 1,
                character: 0,
            },
            end: Position {
                line: insert_pos.line + 1,
                character: 0,
            },
        };

        let mut clear_range = mapper
            .range(entry.text_ranges().first().copied().unwrap())
            .unwrap()
            .into_lsp();

        if let Some(range_before) = range_before {
            clear_range.start = mapper.position(range_before.end()).unwrap().into_lsp();
        }

        let clear_edit = TextEdit {
            range: clear_range,
            new_text: "".into(),
        };

        let insert_edit = TextEdit {
            range: insert_range,
            new_text: formatted,
        };

        edits.push(insert_edit);
        edits.push(clear_edit);
    } else {
        edits.push(TextEdit {
            range: mapper.range(entry.text_ranges()[0]).unwrap().into_lsp(),
            new_text: formatted,
        })
    }
}

fn extract_table_of_arrays(
    edits: &mut Vec<TextEdit>,
    format_opts: formatter::Options,
    mapper: &Mapper,
    path: String,
    parent_entries: &Entries,
    entry: &EntryNode,
    arr: &ArrayNode,
) {
    let mut formatted = String::new();

    for item in arr.items() {
        match item {
            ValueNode::Table(t) => {
                formatted += &format_extracted_table(path.trim(), t.entries(), true);
                formatted += "\n";
            }
            _ => return,
        }
    }

    formatted = formatter::format(&formatted, format_opts)
        .trim_end()
        .to_string();

    if let (range_before, Some(last_range)) = extract_replace_ranges(entry, parent_entries) {
        let insert_pos = mapper.position(last_range.end()).unwrap().into_lsp();

        let insert_range = Range {
            start: Position {
                line: insert_pos.line + 1,
                character: 0,
            },
            end: Position {
                line: insert_pos.line + 1,
                character: 0,
            },
        };

        let mut clear_range = mapper
            .range(entry.text_ranges().first().copied().unwrap())
            .unwrap()
            .into_lsp();

        if let Some(range_before) = range_before {
            clear_range.start = mapper.position(range_before.end()).unwrap().into_lsp();
        }

        let clear_edit = TextEdit {
            range: clear_range,
            new_text: "".into(),
        };

        let insert_edit = TextEdit {
            range: insert_range,
            new_text: formatted,
        };

        edits.push(insert_edit);
        edits.push(clear_edit);
    } else {
        edits.push(TextEdit {
            range: mapper.range(entry.text_ranges()[0]).unwrap().into_lsp(),
            new_text: formatted,
        })
    }
}

fn inline_table(
    edits: &mut Vec<TextEdit>,
    format_opts: formatter::Options,
    mapper: &Mapper,
    parent_range: Option<TextRange>,
    parent_entries: &Entries,
    key: impl AsRef<str>,
    table: &TableNode,
) {
    let s = format!("{}={}\n", key.as_ref(), create_inline_table(table));
    let s = formatter::format(&s, format_opts);

    for range in table.text_ranges().into_iter().skip(1).rev() {
        edits.push(TextEdit {
            range: mapper.range(range).unwrap().into_lsp(),
            new_text: "".into(),
        });
    }

    edits.push(TextEdit {
        range: mapper
            .range(table.syntax().text_range())
            .unwrap()
            .into_lsp(),
        new_text: "".into(),
    });

    let insert_position = mapper
        .position(
            parent_entries
                .iter()
                .filter(|(_, e)| match e.value() {
                    ValueNode::Table(t) => t.is_inline(),
                    ValueNode::Array(arr) => !arr.is_array_of_tables(),
                    _ => true,
                })
                .last()
                .map(|e| e.1.text_ranges()[0].end())
                .unwrap_or_else(|| parent_range.map(|pr| pr.end()).unwrap_or_default()),
        )
        .unwrap();

    let insert_position = Position {
        line: insert_position.line as u32 + 1,
        character: 0,
    };

    let insert_range = Range {
        start: insert_position,
        end: insert_position,
    };

    edits.push(TextEdit {
        range: insert_range,
        new_text: s,
    });
}

fn has_array_of_tables(val: &ValueNode) -> bool {
    match val {
        ValueNode::Array(arr) => {
            if arr.is_array_of_tables() {
                return true;
            }

            for item in arr.items() {
                if has_array_of_tables(item) {
                    return true;
                }
            }
        }
        ValueNode::Table(table) => {
            for (_, entry) in table.entries().iter() {
                if has_array_of_tables(entry.value()) {
                    return true;
                }
            }
        }
        _ => {}
    }

    false
}

fn inline_array_of_tables(
    edits: &mut Vec<TextEdit>,
    format_opts: formatter::Options,
    mapper: &Mapper,
    parent_range: Option<TextRange>,
    parent_entries: &Entries,
    key: impl AsRef<str>,
    arr: &ArrayNode,
) {
    let s = format!(
        "{}=[{}]\n",
        key.as_ref(),
        arr.items()
            .iter()
            .map(|v| match v {
                ValueNode::Table(table) => create_inline_table(table) + ",",
                _ => unreachable!(),
            })
            .collect::<String>()
    );
    let s = formatter::format(&s, format_opts);

    for item in arr.items().iter().rev() {
        match item {
            ValueNode::Table(table) => {
                for range in table.text_ranges().into_iter().skip(1).rev() {
                    edits.push(TextEdit {
                        range: mapper.range(range).unwrap().into_lsp(),
                        new_text: "".into(),
                    });
                }
                edits.push(TextEdit {
                    range: mapper
                        .range(table.syntax().text_range())
                        .unwrap()
                        .into_lsp(),
                    new_text: "".into(),
                });
            }
            _ => {}
        }
    }

    let insert_position = mapper
        .position(
            parent_entries
                .iter()
                .filter(|(_, e)| match e.value() {
                    ValueNode::Table(t) => t.is_inline(),
                    ValueNode::Array(arr) => !arr.is_array_of_tables(),
                    _ => true,
                })
                .last()
                .map(|e| e.1.text_ranges()[0].end())
                .unwrap_or_else(|| parent_range.map(|pr| pr.end()).unwrap_or_default()),
        )
        .unwrap();

    let insert_position = Position {
        line: insert_position.line as u32 + 1,
        character: 0,
    };

    let insert_range = Range {
        start: insert_position,
        end: insert_position,
    };

    edits.push(TextEdit {
        range: insert_range,
        new_text: s,
    });
}

fn create_inline_table(table: &TableNode) -> String {
    let mut s = "{".to_string();

    let mut first = true;
    for (k, entry) in table.entries().iter() {
        if !first {
            s += ",";
        } else {
            first = false;
        }

        s += &format!(
            "{}={}",
            k.full_key_string(),
            create_inline_value(entry.value())
        )
    }

    s += "}";

    s
}

fn create_inline_value(v: &ValueNode) -> String {
    match v {
        ValueNode::Array(arr) => {
            let mut s = "[".to_string();
            for item in arr.items() {
                s += &create_inline_value(item);
                s += ",";
            }
            s += "]";
            s
        }
        ValueNode::Table(t) => create_inline_table(t),
        v => {
            format!("{}", v.syntax())
        }
    }
}

fn parent_table<'n, T: Iterator<Item = NodeRef<'n>>>(
    mut nodes: T,
) -> Option<(Option<String>, Option<TextRange>, &'n Entries)> {
    match nodes.next() {
        Some(NodeRef::Table(mut pt)) => {
            if (pt.is_pseudo() && pt.entries().len() > 1) || pt.is_part_of_array() {
                return None;
            }

            if pt.is_pseudo() {
                while let Some(n) = nodes.next() {
                    if let NodeRef::Table(t) = n {
                        if !t.is_pseudo() {
                            pt = t;
                        } else if t.entries().len() > 1 {
                            return None;
                        }
                    }
                }
            }

            Some((
                pt.key().map(|k| format!("{}", k)),
                pt.key().map(|k| k.syntax().text_range()),
                pt.entries(),
            ))
        }
        Some(NodeRef::Root(root)) => Some((None, None, root.entries())),
        _ => None,
    }
}

// The range of the entry right before the given one,
// and the last entry of the parent if any.
fn extract_replace_ranges(
    entry: &EntryNode,
    entries: &Entries,
) -> (Option<TextRange>, Option<TextRange>) {
    let entry_range = entry.text_ranges()[0];

    let mut tr_before = None;
    let mut tr = None;
    for (_, e) in entries.iter() {
        let e_ranges = e.text_ranges();

        if e_ranges[0].start() > entry_range.start()
            && e.value().syntax().kind() != SyntaxKind::TABLE_HEADER
            && e.value().syntax().kind() != SyntaxKind::TABLE_ARRAY_HEADER
        {
            tr = Some(join_ranges(e_ranges));
        } else if e_ranges[0].start() < entry_range.start() {
            tr_before = Some(e_ranges[0]);
        }
    }

    (tr_before, tr)
}

fn format_extracted_table(path: &str, entries: &Entries, array: bool) -> String {
    let mut s = if array {
        format!("[[{}]]\n", path)
    } else {
        format!("[{}]\n", path)
    };

    let entry_count = entries.len();

    for (i, (_, entry)) in entries.iter().enumerate() {
        s += &format!("{}", entry.syntax());

        if i != entry_count - 1 {
            s += "\n";
        }
    }

    s
}
