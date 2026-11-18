# util.into_lsp

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\util.rs` (lines 226-231)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn into_lsp(self) -> lsp_types::Range {
        lsp_types::Range {
            start: self.start.into_lsp(),
            end: self.end.into_lsp(),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn into_lsp(self) -> lsp_types::Range {
        lsp_types::Range {
            start: self.start.into_lsp(),
            end: self.end.into_lsp(),
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
