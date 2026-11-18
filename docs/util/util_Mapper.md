# util.Mapper

- **Type:** struct
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\util.rs` (lines 41-53)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub struct Mapper {
    /// Mapping offsets to positions.
    offset_to_position: BTreeMap<TextSize, Position>,

    /// Mapping positions to offsets.
    position_to_offset: BTreeMap<Position, TextSize>,

    /// Line count.
    lines: usize,

    /// Ending position.
    end: Position,
}
```

## Docstring

_No docstring provided._

## Source

```
pub struct Mapper {
    /// Mapping offsets to positions.
    offset_to_position: BTreeMap<TextSize, Position>,

    /// Mapping positions to offsets.
    position_to_offset: BTreeMap<Position, TextSize>,

    /// Line count.
    lines: usize,

    /// Ending position.
    end: Position,
}
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
