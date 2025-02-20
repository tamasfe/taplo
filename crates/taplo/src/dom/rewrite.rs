use super::{
    node::{DomNode, Node},
    Keys,
};
use crate::{dom, syntax::SyntaxKind};
use rowan::TextRange;
use std::{ops::Range, sync::Arc};
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
                let keys = key.parse::<Keys>()?;
                let nodes = self.root.find_all_matches(keys, false)?;

                for (keys, _) in nodes {
                    let key = match keys.iter().last().cloned() {
                        Some(dom::KeyOrIndex::Key(k)) => k,
                        _ => continue,
                    };

                    for range in key.text_ranges() {
                        self.check_overlap(range)?;

                        self.patches.push(PendingPatch {
                            range,
                            kind: PendingPatchKind::Replace(to.clone()),
                        })
                    }
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
                PendingPatchKind::Replace(to) => {
                    s.replace_range(std_range(patch.range), to);
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
    pub kind: PendingPatchKind,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum PendingPatchKind {
    Replace(Arc<str>),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("only the root node can be patched")]
    RootNodeExpected,
    #[error("expected table")]
    ExpectedTable,
    #[error("new patches would overlap with existing ones")]
    Overlap,
    #[error("{0}")]
    Dom(#[from] dom::error::Error),
}

fn std_range(range: TextRange) -> Range<usize> {
    let start: usize = u32::from(range.start()) as usize;
    let end: usize = u32::from(range.end()) as usize;
    start..end
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

        let expected_toml = r#"
[table_new.middle_new.inner_new]
[table_new.middle_new.inner_new.inner2_new]
"#;

        let root = parse(toml).into_dom();

        let mut patches = Rewrite::new(root).unwrap();

        patches.rename_keys("table", "table_new").unwrap();
        patches.rename_keys("table.middle", "middle_new").unwrap();
        patches
            .rename_keys("table.middle.inner", "inner_new")
            .unwrap();
        patches
            .rename_keys("table.middle.inner.inner", "inner2_new")
            .unwrap();

        assert_eq!(expected_toml, patches.to_string());
    }

    #[test]
    fn rename_keys_array_of_tables() {
        let toml = r#"
[[table.middle.inner]]
[[table.middle.inner]]
[table.middle.inner.inner]
"#;

        let expected_toml = r#"
[[table_new.middle_new.inner_new]]
[[table_new.middle_new.inner_new]]
[table_new.middle_new.inner_new.inner2_new]
"#;

        let root = parse(toml).into_dom();

        let mut patches = Rewrite::new(root).unwrap();

        patches.rename_keys("table", "table_new").unwrap();
        patches.rename_keys("table.middle", "middle_new").unwrap();
        patches
            .rename_keys("table.middle.inner", "inner_new")
            .unwrap();
        patches
            .rename_keys("table.middle.inner.*.inner", "inner2_new")
            .unwrap();

        assert_eq!(expected_toml, patches.to_string());
    }
}
