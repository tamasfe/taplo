# mod.overlaps

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\util\mod.rs` (lines 179-186)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn overlaps(range: TextRange, other: TextRange) -> bool {
    range.contains_range(other)
        || other.contains_range(range)
        || range.contains(other.start())
        || range.contains(other.end())
        || other.contains(range.start())
        || other.contains(range.end())
}
```

## Docstring

_No docstring provided._

## Source

```
pub fn overlaps(range: TextRange, other: TextRange) -> bool {
    range.contains_range(other)
        || other.contains_range(range)
        || range.contains(other.start())
        || range.contains(other.end())
        || other.contains(range.start())
        || other.contains(range.end())
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
