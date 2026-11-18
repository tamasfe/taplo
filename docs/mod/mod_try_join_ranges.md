# mod.try_join_ranges

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\mod.rs` (lines 172-177)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn try_join_ranges<I: IntoIterator<Item = TextRange>>(ranges: I) -> Option<TextRange> {
    ranges.into_iter().fold(None, |ranges, range| match ranges {
        Some(r) => Some(range.cover(r)),
        None => Some(range),
    })
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn try_join_ranges<I: IntoIterator<Item = TextRange>>(ranges: I) -> Option<TextRange> {
    ranges.into_iter().fold(None, |ranges, range| match ranges {
        Some(r) => Some(range.cover(r)),
        None => Some(range),
    })
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
