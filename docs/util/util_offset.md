# util.offset

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\util.rs` (lines 72-74)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn offset(&self, position: Position) -> Option<TextSize> {
        self.position_to_offset.get(&position).copied()
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn offset(&self, position: Position) -> Option<TextSize> {
        self.position_to_offset.get(&position).copied()
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
