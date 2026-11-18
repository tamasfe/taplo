# hover.is_primitive

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-lsp\src\handlers\hover.rs` (lines 279-295)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn is_primitive(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        BOOL | DATE
            | DATE_TIME_LOCAL
            | DATE_TIME_OFFSET
            | TIME
            | STRING
            | MULTI_LINE_STRING
            | STRING_LITERAL
            | MULTI_LINE_STRING_LITERAL
            | INTEGER
            | INTEGER_HEX
            | INTEGER_OCT
            | INTEGER_BIN
    )
}
```

## Docstring

_No docstring provided._

## Source

```
fn is_primitive(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        BOOL | DATE
            | DATE_TIME_LOCAL
            | DATE_TIME_OFFSET
            | TIME
            | STRING
            | MULTI_LINE_STRING
            | STRING_LITERAL
            | MULTI_LINE_STRING_LITERAL
            | INTEGER
            | INTEGER_HEX
            | INTEGER_OCT
            | INTEGER_BIN
    )
}
```

## Git History

- Commits: 7
- Authors: 4
- Bug fixes: 3
- Features: 3

## Callers

None

## Callees

None
