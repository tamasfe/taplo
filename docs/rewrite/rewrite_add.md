# rewrite.add

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\rewrite.rs` (lines 28-57)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
```

## Git History

- Commits: 2
- Authors: 2
- Bug fixes: 1
- Features: 1

## Callers

None

## Callees

None
