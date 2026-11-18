# util.text_range

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\util.rs` (lines 77-80)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn text_range(&self, range: Range) -> Option<TextRange> {
        self.offset(range.start)
            .and_then(|start| self.offset(range.end).map(|end| TextRange::new(start, end)))
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn text_range(&self, range: Range) -> Option<TextRange> {
        self.offset(range.start)
            .and_then(|start| self.offset(range.end).map(|end| TextRange::new(start, end)))
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
