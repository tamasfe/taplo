# mod.format_array

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 956-1170)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn format_array(node: SyntaxNode, options: &Options, context: &Context) -> impl FormattedItem {
    let mut multiline = is_array_multiline(&node) || context.force_multiline;

    let mut formatted = String::new();

    // We always try to collapse it if possible.
    if can_collapse_array(&node) && options.array_auto_collapse && !context.force_multiline {
        multiline = false;
    }

    // We use the same strategy as for entries, refer to [`format_root`].
    let mut skip_newlines = 0;

    // Formatted value, optional trailing comment
    // The value must not include the comma at the end.
    let mut value_group: Vec<(String, Option<String>)> = Vec::new();
    let mut commas_group: Vec<bool> = Vec::new();

    let add_values = |value_group: &mut Vec<(String, Option<String>)>,
                      commas_group: &mut Vec<bool>,
                      formatted: &mut String,
                      context: &Context|
     -> bool {
        let were_values = !value_group.is_empty();

        if options.reorder_arrays {
            value_group.sort_unstable_by(|x, y| x.0.cmp(&y.0));
        }

        for (has_comma, p) in commas_group.drain(0..).zip(value_group.iter_mut()) {
            if has_comma {
                p.0 += ","
            };
        }

        if !multiline {
            for (idx, (val, comment)) in value_group.drain(0..).enumerate() {
                debug_assert!(comment.is_none());
                if idx != 0 {
                    *formatted += " "
                }

                *formatted += &val;
            }

            return were_values;
        }

        let mut comment_count = 0;
        let rows = value_group
            .drain(0..)
            .map(|(value, comment)| {
                let mut row = Vec::with_capacity(5);

                row.push(context.indent(options).collect::<String>());
                row.push(value);
                if let Some(c) = comment {
                    row.push(c);
                    comment_count += 1;
                }

                row
            })
            .collect::<Vec<_>>();

        let align_comments = options.should_align_comments(comment_count);
        *formatted += &format_rows(
            if align_comments { 0..usize::MAX } else { 0..0 },
            1..usize::MAX,
            &rows,
            options.newline(),
            " ",
        );

        were_values
    };

    let node_count = node.children().count();

    let mut inner_context = context.clone();

    if multiline {
        inner_context.indent_level += 1;
    }

    let mut dangling_newline_count = 0;

    let mut node_index = 0;
    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                VALUE => {
                    if multiline && formatted.ends_with('[') {
                        formatted += options.newline();
                    }

                    let val = format_value(n, options, &inner_context);
                    let mut val_string = String::new();

                    val.write_to(&mut val_string, options);

                    let has_comma =
                        node_index < node_count - 1 || (multiline && options.array_trailing_comma);
                    commas_group.push(has_comma);

                    value_group.push((val_string, val.trailing_comment()));
                    skip_newlines += 1;

                    node_index += 1;
                }
                _ => {
                    if cfg!(debug_assertions) {
                        unreachable!()
                    }
                }
            },
            NodeOrToken::Token(t) => match t.kind() {
                BRACKET_START => {
                    formatted += "[";
                    if !options.compact_arrays && !multiline {
                        formatted += " ";
                    }
                }
                BRACKET_END => {
                    add_values(
                        &mut value_group,
                        &mut commas_group,
                        &mut formatted,
                        &inner_context,
                    );

                    if multiline {
                        if !formatted.ends_with('\n') {
                            formatted += options.newline();
                        }

                        formatted.extend(context.indent(options));
                    } else if !options.compact_arrays {
                        formatted += " ";
                    }
                    formatted += "]";
                }
                NEWLINE => {
                    if !multiline {
                        continue;
                    }

                    let mut newline_count = t.text().newline_count();

                    match dangling_newlines(t.clone()) {
                        Some(dnl) => {
                            dangling_newline_count += dnl;
                            continue;
                        }
                        None => {
                            newline_count += dangling_newline_count;
                            dangling_newline_count = 0;
                        }
                    }

                    if newline_count > 1 {
                        add_values(
                            &mut value_group,
                            &mut commas_group,
                            &mut formatted,
                            &inner_context,
                        );
                        skip_newlines = 0;
                    }

                    formatted.extend(options.newlines(newline_count.saturating_sub(skip_newlines)));
                }
                COMMENT => {
                    let newline_before = t
                        .siblings_with_tokens(rowan::Direction::Prev)
                        .skip(1)
                        .find(|s| s.kind() != WHITESPACE)
                        .map(|s| s.kind() == NEWLINE)
                        .unwrap_or(false);

                    if !newline_before && !value_group.is_empty() {
                        // It's actually trailing comment, so we add it to the last value.
                        value_group.last_mut().unwrap().1 = Some(t.text().to_string());
                        continue;
                    }

                    if add_values(
                        &mut value_group,
                        &mut commas_group,
                        &mut formatted,
                        &inner_context,
                    ) {
                        formatted += options.newline();
                        skip_newlines = 0;
                    }

                    if formatted.ends_with('[') {
                        formatted += " ";
                        formatted += t.text();
                    } else {
                        formatted.extend(inner_context.indent(options));
                        formatted += t.text();
                    }
                }
                _ => {}
            },
        }
    }

    if formatted.is_empty() {
        formatted = "[]".into();
    }

    (node.into(), formatted, None)
}
```

## Docstring

_No docstring provided._

## Source

```
fn format_array(node: SyntaxNode, options: &Options, context: &Context) -> impl FormattedItem {
    let mut multiline = is_array_multiline(&node) || context.force_multiline;

    let mut formatted = String::new();

    // We always try to collapse it if possible.
    if can_collapse_array(&node) && options.array_auto_collapse && !context.force_multiline {
        multiline = false;
    }

    // We use the same strategy as for entries, refer to [`format_root`].
    let mut skip_newlines = 0;

    // Formatted value, optional trailing comment
    // The value must not include the comma at the end.
    let mut value_group: Vec<(String, Option<String>)> = Vec::new();
    let mut commas_group: Vec<bool> = Vec::new();

    let add_values = |value_group: &mut Vec<(String, Option<String>)>,
                      commas_group: &mut Vec<bool>,
                      formatted: &mut String,
                      context: &Context|
     -> bool {
        let were_values = !value_group.is_empty();

        if options.reorder_arrays {
            value_group.sort_unstable_by(|x, y| x.0.cmp(&y.0));
        }

        for (has_comma, p) in commas_group.drain(0..).zip(value_group.iter_mut()) {
            if has_comma {
                p.0 += ","
            };
        }

        if !multiline {
            for (idx, (val, comment)) in value_group.drain(0..).enumerate() {
                debug_assert!(comment.is_none());
                if idx != 0 {
                    *formatted += " "
                }

                *formatted += &val;
            }

            return were_values;
        }

        let mut comment_count = 0;
        let rows = value_group
            .drain(0..)
            .map(|(value, comment)| {
                let mut row = Vec::with_capacity(5);

                row.push(context.indent(options).collect::<String>());
                row.push(value);
                if let Some(c) = comment {
                    row.push(c);
                    comment_count += 1;
                }

                row
            })
            .collect::<Vec<_>>();

        let align_comments = options.should_align_comments(comment_count);
        *formatted += &format_rows(
            if align_comments { 0..usize::MAX } else { 0..0 },
            1..usize::MAX,
            &rows,
            options.newline(),
            " ",
        );

        were_values
    };

    let node_count = node.children().count();

    let mut inner_context = context.clone();

    if multiline {
        inner_context.indent_level += 1;
    }

    let mut dangling_newline_count = 0;

    let mut node_index = 0;
    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                VALUE => {
                    if multiline && formatted.ends_with('[') {
                        formatted += options.newline();
                    }

                    let val = format_value(n, options, &inner_context);
                    let mut val_string = String::new();

                    val.write_to(&mut val_string, options);
... (truncated)
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
