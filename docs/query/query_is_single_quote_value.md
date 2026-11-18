# query.is_single_quote_value

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\query.rs` (lines 334-339)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn is_single_quote_value(&self) -> bool {
        self.entry_value().is_some_and(|v| {
            v.descendants_with_tokens()
                .any(|t| matches!(t.kind(), STRING_LITERAL | MULTI_LINE_STRING_LITERAL))
        })
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn is_single_quote_value(&self) -> bool {
        self.entry_value().is_some_and(|v| {
            v.descendants_with_tokens()
                .any(|t| matches!(t.kind(), STRING_LITERAL | MULTI_LINE_STRING_LITERAL))
        })
    }
```

## Git History

- Commits: 6
- Authors: 3
- Bug fixes: 1
- Features: 2

## Callers

None

## Callees

None
