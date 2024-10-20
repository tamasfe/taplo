use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use lsp_async_stub::rpc::Error;
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionList, CompletionParams, CompletionResponse,
    InsertTextFormat,
};
use taplo::dom::{node::TableKind, Keys, Node};

use crate::query::Query;

// we want completions for

static TOP_1000_RECENTLY_DOWNLOADED_CRATES: &str = include_str!("./top_1000_recent_crates.txt");

pub struct IndexCache {
    pub static_crate_rank: Vec<PackageRef<'static>>,
}

#[derive(Debug, Clone)]
pub struct PackageRef<'a> {
    pub name: &'a str,
}

impl IndexCache {
    pub fn new() -> Self {
        let static_crate_rank = TOP_1000_RECENTLY_DOWNLOADED_CRATES
            .trim_end_matches('\n')
            .split('\n')
            .map(|n| PackageRef { name: n })
            .collect();
        Self { static_crate_rank }
    }

    pub fn completion_for_package(&self, query: &str) -> Vec<PackageRef<'_>> {
        self.static_crate_rank
            .iter()
            .filter(|c| c.name.starts_with(query))
            .cloned()
            .collect()
    }

    pub fn versions_for_package(&self, _package: &str) -> Vec<&str> {
        vec!["1.0", "2.0"]
    }

    pub fn features_for_package(&self, _package: &str) -> Vec<&str> {
        vec!["uuid", "postgres", "serde"]
    }
}

static INDEX_CACHE: LazyLock<IndexCache> = LazyLock::new(|| IndexCache::new());

fn crate_index_path(package: &str) -> PathBuf {
    let l = package.len();
    match l {
        1 => Path::new(&package).to_owned(),
        2 => Path::new("2").join(package).to_owned(),
        3 => Path::new("3").join(&package[..1]).join(package).to_owned(),
        _ => Path::new(&package[..2])
            .join(&package[2..4])
            .join(package)
            .to_owned(),
    }
}

pub fn complete_dependencies(
    _params: CompletionParams,
    query: Query,
    path: Keys,
    node: Node,
) -> Result<Option<CompletionResponse>, Error> {
    let mut dotted = path.dotted().split(".").skip(1).peekable();
    let package = dotted.next().unwrap_or_default();

    if dotted.peek().is_none()
        && matches!(&node, Node::Table(t) if matches!(t.kind(), TableKind::Regular))
    {
        // package is in header, e.g. [dependencies.tokio]
        let items = INDEX_CACHE
            .completion_for_package(package)
            .into_iter()
            .take(1)
            .map(|p| {
                let name = p.name.to_string();
                let completion = CompletionItem::new_simple(name.clone(), name.clone());
                // completion.insert_text = Some(format!("{name}$0"));
                // completion.insert_text_format = Some(InsertTextFormat::SNIPPET);
                // completion.kind = Some(CompletionItemKind::MODULE);
                completion
            })
            .collect();
        return Ok(Some(CompletionResponse::List(CompletionList {
            is_incomplete: true,
            items,
        })));
    }

    if dotted.peek().is_none() && matches!(&node, Node::Invalid(_)) {
        // package is in a table. e.g.
        // [dependencies]
        // tokio = "1.0"
        let items = INDEX_CACHE
            .completion_for_package(package)
            .into_iter()
            .take(1)
            .map(|p| {
                let name = p.name.to_string();
                let mut completion = CompletionItem::new_simple(name.clone(), name.clone());
                completion.insert_text = Some(format!("{name} = \"$1\"$0"));
                completion.insert_text_format = Some(InsertTextFormat::SNIPPET);
                completion.kind = Some(CompletionItemKind::MODULE);
                completion
            })
            .collect();
        return Ok(Some(CompletionResponse::List(CompletionList {
            is_incomplete: true,
            items,
        })));
    }

    let next = dotted.next().unwrap_or("version");
    if query.in_inline_table() || !["version", "features", "optional"].contains(&next) {
        // we are in an inline table, or we are not in a known key
        let mut items = Vec::with_capacity(3);
        let k = "version";
        let mut item = CompletionItem::new_simple(k.into(), k.into());
        item.insert_text = Some(format!("{k} = \"$1\"$0"));
        item.insert_text_format = Some(InsertTextFormat::SNIPPET);
        items.push(item);
        let k = "features";
        let mut item = CompletionItem::new_simple(k.into(), k.into());
        item.insert_text = Some(format!("{k} = [\"$1\"]$0"));
        item.insert_text_format = Some(InsertTextFormat::SNIPPET);
        items.push(item);
        let k = "optional";
        let mut item = CompletionItem::new_simple(k.into(), k.into());
        item.insert_text = Some(format!("{k} = true$0"));
        item.insert_text_format = Some(InsertTextFormat::SNIPPET);
        items.push(item);
        return Ok(Some(CompletionResponse::Array(items)));
    }

    match next {
        "version" => {
            dbg!("version");
            let versions = INDEX_CACHE.versions_for_package(package);
            let completions = versions
                .into_iter()
                .map(|v| CompletionItem::new_simple(v.to_string(), v.to_string()))
                .collect();
            Ok(Some(CompletionResponse::Array(completions)))
        }
        "features" => {
            dbg!("features");
            let features = INDEX_CACHE.features_for_package(package);
            let completions = features
                .into_iter()
                .map(|f| {
                    let completion = CompletionItem::new_simple(f.to_string(), f.to_string());
                    completion
                })
                .collect();
            Ok(Some(CompletionResponse::Array(completions)))
        }
        "optional" => {
            dbg!("optional");
            let completions = vec![
                CompletionItem::new_simple("true".into(), "true".into()),
                CompletionItem::new_simple("false".into(), "false".into()),
            ];
            Ok(Some(CompletionResponse::Array(completions)))
        }
        _ => Ok(None),
    }
}
