# mod.format_table_header

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 1172-1198)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn format_table_header(
    node: SyntaxNode,
    options: &Options,
    context: &Context,
) -> impl FormattedItem {
    let mut formatted = String::new();
    let mut comment = None;

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => {
                format_key(n, &mut formatted, options, context);
            }
            NodeOrToken::Token(t) => match t.kind() {
                BRACKET_START | BRACKET_END => formatted += t.text(),
                WHITESPACE | NEWLINE => {}
                COMMENT => {
                    debug_assert!(comment.is_none());
                    comment = Some(t.text().to_string());
                }
                _ => formatted += t.text(),
            },
        }
    }

    (node.into(), formatted, comment)
}
```

## Docstring

_No docstring provided._

## Source

```
fn format_table_header(
    node: SyntaxNode,
    options: &Options,
    context: &Context,
) -> impl FormattedItem {
    let mut formatted = String::new();
    let mut comment = None;

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => {
                format_key(n, &mut formatted, options, context);
            }
            NodeOrToken::Token(t) => match t.kind() {
                BRACKET_START | BRACKET_END => formatted += t.text(),
                WHITESPACE | NEWLINE => {}
                COMMENT => {
                    debug_assert!(comment.is_none());
                    comment = Some(t.text().to_string());
                }
                _ => formatted += t.text(),
            },
        }
    }

    (node.into(), formatted, comment)
}
```

## Git History

- Commits: 17
- Authors: 14
- Bug fixes: 5
- Features: 4

## Callers

None

## Callees

None
