# rewrite.check_overlap

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\rewrite.rs` (lines 63-75)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
