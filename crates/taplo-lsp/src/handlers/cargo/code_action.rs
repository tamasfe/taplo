#![allow(unused)]
use std::{collections::HashMap, path::Path};

use lsp_async_stub::{rpc::Error, Context, Params};
use lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams, CodeActionResponse,
    Position, Range, TextEdit, WorkspaceEdit,
};
use taplo::dom::{Keys, Node};
use taplo_common::environment::Environment;

use crate::world::{DocumentState, World};

pub async fn code_action(
    params: CodeActionParams,
    path: Keys,
    node: Node,
    doc: &DocumentState,
) -> Result<Option<CodeActionResponse>, Error> {
    let document_uri = &params.text_document.uri;
    let position = params.range.start;

    let mut dotted = path.dotted().split(".").peekable();
    let location = dotted.next().unwrap_or_default();
    if !["dependencies", "dev-dependencies", "build-dependencies"].contains(&location) {
        return Ok(None);
    }
    let package = dotted.next().unwrap_or_default();

    dbg!(&node);
    dbg!(path.dotted());
    let mut actions = Vec::new();

    if dotted.peek().is_none() {
        match &node {
            Node::Str(s) => {
                let version = s.value();
                let range = node
                    .text_ranges(true)
                    .next()
                    .and_then(|r| doc.mapper.range(r));
                let Some(range) = range else {
                    return Ok(None);
                };
                let start = Position::new(range.start.line as u32, range.start.character as u32);
                let end = Position::new(range.end.line as u32, range.end.character as u32);
                let range = Range::new(start, end);
                let edit = TextEdit::new(range, format!("{{ version = \"{version}\" }}"));
                let mut map = HashMap::new();
                map.insert(document_uri.clone(), vec![edit]);
                let action = CodeAction {
                    title: "Expand dependency specification".to_string(),
                    kind: Some(CodeActionKind::QUICKFIX),
                    edit: Some(WorkspaceEdit {
                        changes: Some(map),
                        ..WorkspaceEdit::default()
                    }),
                    ..CodeAction::default()
                };
                actions.push(CodeActionOrCommand::CodeAction(action));
            }
            _ => return Ok(None),
        }
    }
    if actions.is_empty() {
        Ok(None)
    } else {
        Ok(Some(actions))
    }
}
