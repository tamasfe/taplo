# mod.format_value

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 804-857)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn format_value(node: SyntaxNode, options: &Options, context: &Context) -> impl FormattedItem {
    let mut value = String::new();
    let mut comment = None;

    let mut scoped_options = options.clone();
    context.update_options(&mut scoped_options, node.text_range());

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                ARRAY => {
                    let formatted = format_array(n, &scoped_options, context);

                    let c = formatted.trailing_comment();

                    if let Some(c) = c {
                        debug_assert!(comment.is_none());
                        comment = Some(c)
                    }

                    debug_assert!(value.is_empty());
                    formatted.write_to(&mut value, &scoped_options);
                }
                INLINE_TABLE => {
                    let formatted = format_inline_table(n, &scoped_options, context);

                    let c = formatted.trailing_comment();

                    if let Some(c) = c {
                        debug_assert!(comment.is_none());
                        comment = Some(c)
                    }

                    debug_assert!(value.is_empty());

                    formatted.write_to(&mut value, &scoped_options);
                }
                _ => unreachable!(),
            },
            NodeOrToken::Token(t) => match t.kind() {
                NEWLINE | WHITESPACE => {}
                COMMENT => {
                    debug_assert!(comment.is_none());
                    comment = Some(t.text().into());
                }
                _ => {
                    value = t.text().into();
                }
            },
        }
    }

    (node.into(), value, comment)
}
```

## Docstring

_No docstring provided._

## Source

```
fn format_value(node: SyntaxNode, options: &Options, context: &Context) -> impl FormattedItem {
    let mut value = String::new();
    let mut comment = None;

    let mut scoped_options = options.clone();
    context.update_options(&mut scoped_options, node.text_range());

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                ARRAY => {
                    let formatted = format_array(n, &scoped_options, context);

                    let c = formatted.trailing_comment();

                    if let Some(c) = c {
                        debug_assert!(comment.is_none());
                        comment = Some(c)
                    }

                    debug_assert!(value.is_empty());
                    formatted.write_to(&mut value, &scoped_options);
                }
                INLINE_TABLE => {
                    let formatted = format_inline_table(n, &scoped_options, context);

                    let c = formatted.trailing_comment();

                    if let Some(c) = c {
                        debug_assert!(comment.is_none());
                        comment = Some(c)
                    }

                    debug_assert!(value.is_empty());

                    formatted.write_to(&mut value, &scoped_options);
                }
                _ => unreachable!(),
            },
            NodeOrToken::Token(t) => match t.kind() {
                NEWLINE | WHITESPACE => {}
                COMMENT => {
                    debug_assert!(comment.is_none());
                    comment = Some(t.text().into());
                }
                _ => {
                    value = t.text().into();
                }
            },
        }
    }

    (node.into(), value, comment)
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
