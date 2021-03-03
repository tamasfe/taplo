use std::collections::HashMap;

use crate::{utils::LspExt, World};
use lsp_async_stub::{rpc::Error, Context, Params};
use lsp_types::*;
use rowan::TextRange;
use taplo::{
    analytics::NodeRef,
    dom::{ArrayNode, Entries, EntryNode, NodeSyntax, ValueNode},
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

    let mut node_iter = query.after.nodes.into_iter().rev();

    let mut actions = Vec::new();

    if let Some(NodeRef::Key(_)) = node_iter.next() {
        if let Some(NodeRef::Entry(entry)) = node_iter.next() {
            match entry.value() {
                ValueNode::Table(table) => {
                    if table.is_inline() {
                        if let Some((parent_path, parent_entries)) = parent_table(node_iter) {
                            let mut edits = Vec::new();
                            extract_inline_table(
                                &mut edits,
                                format_opts.clone(),
                                &doc.mapper,
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

                            changes.insert(p.text_document.uri, edits);

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
                            if let Some((parent_path, parent_entries)) = parent_table(node_iter) {
                                let mut edits = Vec::new();
                                extract_table_of_arrays(
                                    &mut edits,
                                    format_opts.clone(),
                                    &doc.mapper,
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

                                changes.insert(p.text_document.uri, edits);

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
        }
    }

    return Ok(Some(actions));
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
    formatted = formatter::format(&formatted, format_opts);

    if let (range_before, Some(last_range)) = replace_ranges(entry, parent_entries) {
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
            _ => unreachable!(),
        }
    }

    formatted = formatter::format(&formatted, format_opts);

    if let (range_before, Some(last_range)) = replace_ranges(entry, parent_entries) {
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

fn parent_table<'n, T: Iterator<Item = NodeRef<'n>>>(
    mut nodes: T,
) -> Option<(Option<String>, &'n Entries)> {
    match nodes.next() {
        Some(NodeRef::Table(mut pt)) => {
            if pt.is_pseudo() && pt.entries().len() > 1 {
                return None;
            }
            while let Some(n) = nodes.next() {
                if let NodeRef::Table(t) = n {
                    if !t.is_pseudo() {
                        pt = t;
                    } else if t.entries().len() > 1 {
                        return None;
                    }
                }
            }

            if pt.is_inline() {
                return None;
            }

            Some((pt.key().map(|k| format!("{}", k)), pt.entries()))
        }
        Some(NodeRef::Root(root)) => Some((None, root.entries())),
        _ => None,
    }
}

// The range of the entry right before the given one,
// and the last entry of the parent if any.
fn replace_ranges(entry: &EntryNode, entries: &Entries) -> (Option<TextRange>, Option<TextRange>) {
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
