# document_symbols.ensure_non_empty_key

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\document_symbols.rs` (lines 163-169)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn ensure_non_empty_key(s: String) -> String {
    if s.is_empty() {
        r"''".into()
    } else {
        s
    }
}
```

## Docstring

_No docstring provided._

## Source

```
fn ensure_non_empty_key(s: String) -> String {
    if s.is_empty() {
        r"''".into()
    } else {
        s
    }
}
```

## Git History

- Commits: 4
- Authors: 3
- Bug fixes: 0
- Features: 2

## Callers

None

## Callees

None
