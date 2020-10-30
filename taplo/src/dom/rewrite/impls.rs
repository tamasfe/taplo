use std::rc::Rc;

use super::RewriteNode;
use crate::dom::{self, rewrite};
use rowan::{TextRange, TextSize};

#[derive(Debug, Default)]
pub(crate) struct RewriteInfo {
    pub(crate) removes: Vec<TextRange>,
    pub(crate) inserts: Vec<(TextSize, rewrite::Node)>,
}

impl dom::RootNode {
    pub fn rewrite<F: Fn(dom::Node) -> RewriteNode + 'static>(self, rewrite_fn: F) -> String {
        let f = Rc::new(rewrite_fn);
        let mut info = RewriteInfo::default();

        self.rewrite_impl(f, &mut info);

        // Rewrite rowan tree removing old stuff and adding new.
        todo!()
    }

    fn rewrite_impl(
        self,
        rewrite_fn: Rc<dyn Fn(dom::Node) -> RewriteNode>,
        info: &mut RewriteInfo,
    ) {
        let text_range = self.syntax.text_range();

        match rewrite_fn(self.into()) {
            RewriteNode::Old(n) => match n {
                dom::Node::Root(r) => {
                    for (_, entry) in r.entries {
                        dom::Node::from(entry).rewrite_impl(rewrite_fn.clone(), info)
                    }
                }
                _ => unreachable!(),
            },
            RewriteNode::New(n) => {
                info.removes.push(text_range);
                info.inserts.push((text_range.start(), n));
            }
        }
    }
}

impl dom::TableNode {
    fn rewrite_impl(
        self,
        rewrite_fn: Rc<dyn Fn(dom::Node) -> RewriteNode>,
        info: &mut RewriteInfo,
    ) {
        match rewrite_fn(self.into()) {
            RewriteNode::Old(n) => match n {
                dom::Node::Table(r) => {
                    for (_, entry) in r.entries {
                        dom::Node::from(entry).rewrite_impl(rewrite_fn.clone(), info)
                    }
                }
                _ => unreachable!(),
            },
            RewriteNode::New(_) => {}
        }
    }
}

impl dom::EntryNode {
    fn rewrite_impl(
        self,
        rewrite_fn: Rc<dyn Fn(dom::Node) -> RewriteNode>,
        info: &mut RewriteInfo,
    ) {
        match rewrite_fn(self.into()) {
            RewriteNode::Old(n) => {}
            RewriteNode::New(_) => {}
        }
    }
}

impl dom::KeyNode {
    fn rewrite_impl(
        self,
        rewrite_fn: Rc<dyn Fn(dom::Node) -> RewriteNode>,
        info: &mut RewriteInfo,
    ) {
        match rewrite_fn(self.into()) {
            RewriteNode::Old(n) => {}
            RewriteNode::New(_) => {}
        }
    }
}

impl dom::ValueNode {
    fn rewrite_impl(
        self,
        rewrite_fn: Rc<dyn Fn(dom::Node) -> RewriteNode>,
        info: &mut RewriteInfo,
    ) {
        match rewrite_fn(self.into()) {
            RewriteNode::Old(n) => {}
            RewriteNode::New(_) => {}
        }
    }
}

impl dom::ArrayNode {
    fn rewrite_impl(
        self,
        rewrite_fn: Rc<dyn Fn(dom::Node) -> RewriteNode>,
        info: &mut RewriteInfo,
    ) {
        match rewrite_fn(self.into()) {
            RewriteNode::Old(n) => {}
            RewriteNode::New(_) => {}
        }
    }
}

impl dom::Node {
    fn rewrite_impl(
        self,
        rewrite_fn: Rc<dyn Fn(dom::Node) -> RewriteNode>,
        info: &mut RewriteInfo,
    ) {
        match self {
            dom::Node::Root(v) => v.rewrite_impl(rewrite_fn, info),
            dom::Node::Table(v) => v.rewrite_impl(rewrite_fn, info),
            dom::Node::Entry(v) => v.rewrite_impl(rewrite_fn, info),
            dom::Node::Key(v) => v.rewrite_impl(rewrite_fn, info),
            dom::Node::Value(v) => v.rewrite_impl(rewrite_fn, info),
            dom::Node::Array(v) => v.rewrite_impl(rewrite_fn, info),
        }
    }
}
