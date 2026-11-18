# util.all_range

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\util.rs` (lines 104-112)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn all_range(&self) -> Range {
        Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: self.end,
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn all_range(&self) -> Range {
        Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: self.end,
        }
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
