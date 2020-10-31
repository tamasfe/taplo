use super::RewriteNode;
use crate::{
    dom::{self, rewrite},
    syntax::{SyntaxElement, SyntaxNode},
};
use rowan::{GreenNodeBuilder, NodeOrToken, TextRange, TextSize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
struct RewriteInfo {
    removes: HashSet<TextRange>,
    inserts: HashMap<TextSize, rewrite::Node>,
}

fn rewrite_tree(elem: SyntaxElement, info: &mut RewriteInfo, builder: &mut GreenNodeBuilder) {
    match info.removes.take(&elem.text_range()) {
        Some(to_remove) => {
            if let Some(to_insert) = info.inserts.remove(&to_remove.start()) {
                to_insert.into_tree(builder);
            }
        }
        None => match elem {
            NodeOrToken::Node(node) => {
                builder.start_node(node.kind().into());

                for child in node.children_with_tokens() {
                    rewrite_tree(child, info, builder);
                }

                builder.finish_node();
            }
            NodeOrToken::Token(token) => {
                builder.token(token.kind().into(), token.text().clone());
            }
        },
    }
}

impl dom::RootNode {
    pub fn rewrite<F: Fn(dom::Path, dom::Node) -> RewriteNode + 'static>(
        self,
        rewrite_fn: F,
    ) -> String {
        let mut info = RewriteInfo::default();
        let root_syntax = self.syntax.clone();
        self.rewrite_impl(dom::Path::new(), &rewrite_fn, &mut info);
        let mut builder = GreenNodeBuilder::new();
        rewrite_tree(root_syntax, &mut info, &mut builder);
        SyntaxNode::new_root(builder.finish()).to_string()
    }

    fn rewrite_impl(
        self,
        path: dom::Path,
        rewrite_fn: &dyn Fn(dom::Path, dom::Node) -> RewriteNode,
        info: &mut RewriteInfo,
    ) {
        let text_range = self.syntax.text_range();

        match rewrite_fn(dom::Path::new(), self.into()) {
            RewriteNode::Old(n) => match n {
                dom::Node::Root(r) => {
                    for (k, entry) in r.entries {
                        dom::Node::from(entry).rewrite_impl(
                            path.join(k.full_key_string_stripped()),
                            rewrite_fn,
                            info,
                        )
                    }
                }
                _ => unreachable!(),
            },
            RewriteNode::New(n) => {
                info.removes.insert(text_range);
                info.inserts.insert(text_range.start(), n);
            }
        }
    }
}

impl dom::TableNode {
    fn rewrite_impl(
        self,
        path: dom::Path,
        rewrite_fn: &dyn Fn(dom::Path, dom::Node) -> RewriteNode,
        info: &mut RewriteInfo,
    ) {
        match rewrite_fn(path.clone(), self.into()) {
            RewriteNode::Old(n) => match n {
                dom::Node::Table(t) => {
                    if let Some(k) = t.key() {
                        k.clone().rewrite_impl(path.clone(), rewrite_fn, info);
                    }
                    for (key, entry) in t.entries {
                        dom::Node::from(entry).rewrite_impl(
                            path.join(key.full_key_string_stripped()),
                            rewrite_fn,
                            info,
                        )
                    }
                }
                _ => {
                    panic!("the returned old node must be the one received in the rewrite function")
                }
            },
            RewriteNode::New(_) => {}
        }
    }
}

impl dom::EntryNode {
    fn rewrite_impl(
        self,
        path: dom::Path,
        rewrite_fn: &dyn Fn(dom::Path, dom::Node) -> RewriteNode,
        info: &mut RewriteInfo,
    ) {
        let ranges = self.text_ranges();
        let start_range = ranges.first().cloned().unwrap();
        match rewrite_fn(path.clone(), self.into()) {
            RewriteNode::Old(old) => {
                match old {
                    dom::Node::Entry(entry) => {
                        entry
                            .key
                            .rewrite_impl(path.clone(), rewrite_fn, info);
                        entry
                            .value
                            .rewrite_impl(path, rewrite_fn, info);
                    }
                    _ => panic!(
                        "the returned old node must be the one received in the rewrite function"
                    ),
                };
            }
            RewriteNode::New(new) => {
                info.removes.extend(ranges);
                info.inserts.insert(start_range.start(), new);
            }
        }
    }
}

impl dom::KeyNode {
    fn rewrite_impl(
        self,
        path: dom::Path,
        rewrite_fn: &dyn Fn(dom::Path, dom::Node) -> RewriteNode,
        info: &mut RewriteInfo,
    ) {
        let ranges = self.text_ranges();
        match rewrite_fn(path, self.into()) {
            RewriteNode::Old(old) => {
                match old {
                    dom::Node::Key(_) => {}
                    _ => panic!(
                        "the returned old node must be the one received in the rewrite function"
                    ),
                };
            }
            RewriteNode::New(new) => {
                info.removes.extend(ranges.clone());
                for range in ranges {
                    info.inserts.insert(range.start(), new.clone());
                }
            }
        }
    }
}

impl dom::ValueNode {
    fn rewrite_impl(
        self,
        path: dom::Path,
        rewrite_fn: &dyn Fn(dom::Path, dom::Node) -> RewriteNode,
        info: &mut RewriteInfo,
    ) {
        let ranges = self.text_ranges();
        let start_range = ranges.first().cloned().unwrap();
        match rewrite_fn(path.clone(), self.into()) {
            RewriteNode::Old(old) => {
                match old {
                    dom::Node::Value(v) => match v {
                        dom::ValueNode::Array(arr) => arr.rewrite_impl(path, rewrite_fn, info),
                        dom::ValueNode::Table(t) => t.rewrite_impl(path, rewrite_fn, info),
                        dom::ValueNode::Empty => unreachable!(),
                        _ => {}
                    },
                    _ => panic!(
                        "the returned old node must be the one received in the rewrite function"
                    ),
                };
            }
            RewriteNode::New(new) => {
                info.removes.extend(ranges);
                info.inserts.insert(start_range.start(), new);
            }
        }
    }
}

impl dom::ArrayNode {
    fn rewrite_impl(
        self,
        path: dom::Path,
        rewrite_fn: &dyn Fn(dom::Path, dom::Node) -> RewriteNode,
        info: &mut RewriteInfo,
    ) {
        let ranges = self.text_ranges();
        let start_range = ranges.first().cloned().unwrap();
        match rewrite_fn(path.clone(), self.into()) {
            RewriteNode::Old(old) => {
                match old {
                    dom::Node::Array(v) => {
                        for (i, item) in v.items.into_iter().enumerate() {
                            item.rewrite_impl(path.join(i), rewrite_fn, info);
                        }
                    }
                    _ => panic!(
                        "the returned old node must be the one received in the rewrite function"
                    ),
                };
            }
            RewriteNode::New(new) => {
                info.removes.extend(ranges);
                info.inserts.insert(start_range.start(), new);
            }
        }
    }
}

impl dom::Node {
    fn rewrite_impl(
        self,
        path: dom::Path,
        rewrite_fn: &dyn Fn(dom::Path, dom::Node) -> RewriteNode,
        info: &mut RewriteInfo,
    ) {
        match self {
            dom::Node::Root(v) => v.rewrite_impl(path, rewrite_fn, info),
            dom::Node::Table(v) => v.rewrite_impl(path, rewrite_fn, info),
            dom::Node::Entry(v) => v.rewrite_impl(path, rewrite_fn, info),
            dom::Node::Key(v) => v.rewrite_impl(path, rewrite_fn, info),
            dom::Node::Value(v) => v.rewrite_impl(path, rewrite_fn, info),
            dom::Node::Array(v) => v.rewrite_impl(path, rewrite_fn, info),
        }
    }
}
