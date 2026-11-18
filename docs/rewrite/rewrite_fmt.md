# rewrite.fmt

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\rewrite.rs` (lines 88-100)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
