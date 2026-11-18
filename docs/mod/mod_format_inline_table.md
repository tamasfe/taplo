# mod.format_inline_table

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 859-946)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn format_inline_table(
    node: SyntaxNode,
    options: &Options,
    context: &Context,
) -> impl FormattedItem {
    let mut formatted = String::new();
    let mut comment = None;

    let mut context = context.clone();
    if context.force_multiline {
        context.force_multiline = options.inline_table_expand;
    }
    let context = &context;

    let child_count = node.children().count();

    if node.children().count() == 0 {
        formatted = "{}".into();
    }

    let mut sorted_children = if options.reorder_inline_tables {
        Some(
            node.children()
                .sorted_unstable_by(|x, y| x.to_string().cmp(&y.to_string()))
                .collect::<VecDeque<_>>(),
        )
    } else {
        None
    };

    let mut node_index = 0;
    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => {
                if node_index != 0 {
                    formatted += ", ";
                }

                let child = if options.reorder_inline_tables {
                    sorted_children
                        .as_mut()
                        .and_then(|children| children.pop_front())
                        .unwrap_or(n)
                } else {
                    n
                };

                let entry = format_entry(child, options, context);
                debug_assert!(entry.comment.is_none());
                entry.write_to(&mut formatted, options);

                node_index += 1;
            }
            NodeOrToken::Token(t) => match t.kind() {
                BRACE_START => {
                    if child_count == 0 {
                        // We're only interested in trailing comments.
                        continue;
                    }

                    formatted += "{";
                    if !options.compact_inline_tables {
                        formatted += " ";
                    }
                }
                BRACE_END => {
                    if child_count == 0 {
                        // We're only interested in trailing comments.
                        continue;
                    }

                    if !options.compact_inline_tables {
                        formatted += " ";
                    }
                    formatted += "}";
                }
                WHITESPACE | COMMA => {}
                COMMENT => {
                    debug_assert!(comment.is_none());
                    comment = Some(t.text().into());
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
fn format_inline_table(
    node: SyntaxNode,
    options: &Options,
    context: &Context,
) -> impl FormattedItem {
    let mut formatted = String::new();
    let mut comment = None;

    let mut context = context.clone();
    if context.force_multiline {
        context.force_multiline = options.inline_table_expand;
    }
    let context = &context;

    let child_count = node.children().count();

    if node.children().count() == 0 {
        formatted = "{}".into();
    }

    let mut sorted_children = if options.reorder_inline_tables {
        Some(
            node.children()
                .sorted_unstable_by(|x, y| x.to_string().cmp(&y.to_string()))
                .collect::<VecDeque<_>>(),
        )
    } else {
        None
    };

    let mut node_index = 0;
    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => {
                if node_index != 0 {
                    formatted += ", ";
                }

                let child = if options.reorder_inline_tables {
                    sorted_children
                        .as_mut()
                        .and_then(|children| children.pop_front())
                        .unwrap_or(n)
                } else {
                    n
                };

                let entry = format_entry(child, options, context);
                debug_assert!(entry.comment.is_none());
                entry.write_to(&mut formatted, options);

                node_index += 1;
            }
            NodeOrToken::Token(t) => match t.kind() {
                BRACE_START => {
                    if child_count == 0 {
                        // We're only interested in trailing comments.
                        continue;
                    }

                    formatted += "{";
                    if !options.compact_inline_tables {
                        formatted += " ";
                    }
                }
                BRACE_END => {
                    if child_count == 0 {
                        // We're only interested in trailing comments.
                        continue;
                    }

                    if !options.compact_inline_tables {
                        formatted += " ";
                    }
                    formatted += "}";
                }
                WHITESPACE | COMMA => {}
                COMMENT => {
                    debug_assert!(comment.is_none());
                    comment = Some(t.text().into());
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
