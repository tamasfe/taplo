# util.from_lsp

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\util.rs` (lines 233-238)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn from_lsp(val: lsp_types::Range) -> Self {
        Self {
            start: Position::from_lsp(val.start),
            end: Position::from_lsp(val.end),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn from_lsp(val: lsp_types::Range) -> Self {
        Self {
            start: Position::from_lsp(val.start),
            end: Position::from_lsp(val.end),
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
