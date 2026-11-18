# Module: rewrite

**File:** `\crates\taplo\src\dom\rewrite.rs`

## Overview

Module `rewrite` contains 10 analyzed entities.

```
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
                retur
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| `rewrite.new` | function | No description |
| `rewrite.add` | function | No description |
| `rewrite.check_overlap` | function | No description |
| `rewrite.fmt` | function | No description |
| `rewrite.std_range` | function | No description |
| `rewrite.rename_keys_array_of_tables` | function | No description |
| `rewrite.patches` | function | No description |
| `rewrite.rename_keys` | function | No description |
| `rewrite.Rewrite` | struct | No description |
| `rewrite.PendingPatch` | struct | No description |

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 1
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
