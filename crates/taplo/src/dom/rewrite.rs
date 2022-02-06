use std::{ops::Range, sync::Arc};

use crate::{dom, parser::Parser, syntax::SyntaxKind};

use super::{
    error::QueryError,
    from_syntax::keys_from_syntax,
    node::{DomNode, Key, Node},
};
use rowan::TextRange;
use thiserror::Error;

#[derive(Debug)]
pub struct Rewrite {
    root: Node,
    patches: Vec<PendingPatch>,
}

impl Rewrite {
    pub fn new(root: Node) -> Result<Self, Error> {
        if !matches!(root.syntax().map(|s| s.kind()), Some(SyntaxKind::ROOT)) {
            return Err(Error::RootNodeExpected);
        }

        Ok(Self {
            root,
            patches: Default::default(),
        })
    }

    pub fn add(&mut self, patch: impl Into<Patch>) -> Result<&mut Self, Error> {
        let patch = patch.into();
        match patch {
            Patch::RenameKeys { key, to } => {
                let mut keys = parse_keys(&key)?;
                let mut node = self.root.clone();

                while let Some(search_key) = keys.next() {
                    if keys.len() == 0 {
                        if let Node::Table(t) = node {
                            let entries = t.inner.entries.read();
                            let (node_key, _) =
                                entries.lookup.get_key_value(&search_key).ok_or_else(|| {
                                    Error::Dom(dom::Error::Query(QueryError::NotFound {
                                        key: search_key.value().to_string(),
                                    }))
                                })?;

                            let mut ranges = Vec::with_capacity(1);
                            if let Some(s) = node_key.syntax() {
                                ranges.push(s.text_range());
                            }
                            let additional_syntax = node_key.inner.additional_syntaxes.read();
                            ranges.extend(additional_syntax.iter().map(|s| s.text_range()));

                            for range in &ranges {
                                self.check_overlap(*range)?;
                            }

                            self.patches
                                .extend(ranges.into_iter().map(|range| PendingPatch {
                                    range,
                                    kind: AppliedPatchKind::Replace(to.clone()),
                                }));
                        } else {
                            return Err(Error::ExpectedTable);
                        }

                        break;
                    }

                    node = node.try_get(search_key.value())?;
                }
            }
        }

        self.patches
            .sort_by(|a, b| b.range.start().cmp(&a.range.start()));

        Ok(self)
    }

    pub fn patches(&self) -> &[PendingPatch] {
        &self.patches
    }

    fn check_overlap(&self, range: TextRange) -> Result<(), Error> {
        for patch in self.patches() {
            if patch.range.contains_range(range)
                || range.contains_range(patch.range)
                || patch.range.contains(range.start())
                || patch.range.contains(range.end())
            {
                return Err(Error::Overlap);
            }
        }

        Ok(())
    }
}

impl Rewrite {
    pub fn rename_keys(&mut self, key: &str, to: &str) -> Result<&mut Self, Error> {
        self.add(Patch::RenameKeys {
            key: key.into(),
            to: to.into(),
        })
    }
}

impl core::fmt::Display for Rewrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = self.root.syntax().unwrap().to_string();

        for patch in &self.patches {
            match &patch.kind {
                AppliedPatchKind::Replace(to) => {
                    s.replace_range(std_range(patch.range), &*to);
                }
            }
        }

        s.fmt(f)
    }
}

#[derive(Debug)]
pub enum Patch {
    RenameKeys { key: Arc<str>, to: Arc<str> },
}

#[derive(Debug)]
pub struct PendingPatch {
    pub range: TextRange,
    pub kind: AppliedPatchKind,
}

#[derive(Debug)]
pub enum AppliedPatchKind {
    Replace(Arc<str>),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("only the root node can be patched")]
    RootNodeExpected,
    #[error("the given key is invalid: {0}")]
    InvalidKey(crate::parser::Error),
    #[error("expected table")]
    ExpectedTable,
    #[error("new patches would overlap with existing ones")]
    Overlap,
    #[error("{0}")]
    Dom(#[from] dom::error::Error),
}

fn parse_keys(s: &str) -> Result<impl ExactSizeIterator<Item = Key>, Error> {
    let mut p = Parser::new(s).parse_key_only();
    if let Some(err) = p.errors.pop() {
        return Err(Error::InvalidKey(err));
    }
    Ok(keys_from_syntax(&p.into_syntax().into()))
}

#[cfg(test)]
mod tests {
    use super::Rewrite;
    use crate::parser::parse;

    #[test]
    fn rename_keys() {
        let toml = r#"
[table.middle.inner]
[table.middle.inner.inner]
"#;

        let root = parse(toml).into_dom();

        let mut patches = Rewrite::new(root.clone()).unwrap();

        patches.rename_keys("table", "table_new").unwrap();
        patches.rename_keys("table.middle", "middle_new").unwrap();
        patches
            .rename_keys("table.middle.inner", "inner_new")
            .unwrap();
        patches
            .rename_keys("table.middle.inner.inner", "inner2_new")
            .unwrap();

        println!("{patches}");
    }
}

fn std_range(range: TextRange) -> Range<usize> {
    let start: usize = u32::from(range.start()) as usize;
    let end: usize = u32::from(range.end()) as usize;
    start..end
}
