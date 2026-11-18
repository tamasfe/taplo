# util.mappings

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\util.rs` (lines 94-96)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn mappings(&self) -> (&BTreeMap<TextSize, Position>, &BTreeMap<Position, TextSize>) {
        (&self.offset_to_position, &self.position_to_offset)
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn mappings(&self) -> (&BTreeMap<TextSize, Position>, &BTreeMap<Position, TextSize>) {
        (&self.offset_to_position, &self.position_to_offset)
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
