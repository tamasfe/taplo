# mod.join_ranges

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\mod.rs` (lines 162-170)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn join_ranges<I: IntoIterator<Item = TextRange>>(ranges: I) -> TextRange {
    ranges
        .into_iter()
        .fold(None, |ranges, range| match ranges {
            Some(r) => Some(range.cover(r)),
            None => Some(range),
        })
        .unwrap()
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn join_ranges<I: IntoIterator<Item = TextRange>>(ranges: I) -> TextRange {
    ranges
        .into_iter()
        .fold(None, |ranges, range| match ranges {
            Some(r) => Some(range.cover(r)),
            None => Some(range),
        })
        .unwrap()
}
```

## Git History

- Commits: 4
- Authors: 2
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
