# util.new_utf16

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\util.rs` (lines 61-63)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn new_utf16(source: &str, one_based: bool) -> Self {
        Self::new_impl(source, true, if one_based { 1 } else { 0 })
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn new_utf16(source: &str, one_based: bool) -> Self {
        Self::new_impl(source, true, if one_based { 1 } else { 0 })
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
