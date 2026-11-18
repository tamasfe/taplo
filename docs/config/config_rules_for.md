# config.rules_for

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\config.rs` (lines 108-113)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn rules_for<'r>(
        &'r self,
        path: &'r Path,
    ) -> impl DoubleEndedIterator<Item = &'r Rule> + Clone + 'r {
        self.rule.iter().filter(|r| r.is_included(path))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn rules_for<'r>(
        &'r self,
        path: &'r Path,
    ) -> impl DoubleEndedIterator<Item = &'r Rule> + Clone + 'r {
        self.rule.iter().filter(|r| r.is_included(path))
    }
```

## Git History

- Commits: 6
- Authors: 3
- Bug fixes: 4
- Features: 1

## Callers

None

## Callees

None
