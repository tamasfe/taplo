# mod.add_entries

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 627-745)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn add_entries(
    entry_group: &mut Vec<FormattedEntry>,
    formatted: &mut String,
    options: &Options,
    context: &Context,
) -> bool {
    let were_entries = !entry_group.is_empty();

    if options.reorder_keys {
        entry_group.sort();
    }

    let indent_chars_count = context.indent_level * options.indent_string.chars().count();

    // We check for too long lines, and try to expand them if possible.
    // We don't take vertical alignment into account for simplicity.
    if options.array_auto_expand {
        for entry in entry_group.iter_mut() {
            let comment_chars_count = entry
                .comment
                .as_ref()
                .map(
                    |c| c.chars().count() + 1, // account for the separator ' ' as well
                )
                .unwrap_or(0);

            let line_count = entry.value.split('\n').count();

            // check each line of the value
            // for the first line we include the actual indent, key, and the eq parts as well
            for (idx, line) in entry.value.split('\n').enumerate() {
                let mut chars_count = line.chars().count();
                if idx == 0 {
                    chars_count += indent_chars_count;
                    chars_count += entry.key.chars().count();
                    chars_count += if options.compact_entries { 1 } else { 3 }; // " = "
                }

                // Include comment in the last line.
                if idx == line_count - 1 {
                    chars_count += comment_chars_count;
                }

                if chars_count > options.column_width {
                    let mut context = context.clone();
                    context.force_multiline = true;

                    // too long, reformat the value of the entry
                    let value = format_value(
                        entry
                            .syntax
                            .as_node()
                            .unwrap()
                            .children()
                            .find(|n| n.kind() == VALUE)
                            .unwrap(),
                        options,
                        &context,
                    );

                    entry.value.clear();

                    if let Some(c) = value.trailing_comment() {
                        debug_assert!(
                            entry.comment.is_none() || entry.comment.clone().unwrap() == c
                        );
                        entry.comment = Some(c);
                    }

                    value.write_to(&mut entry.value, options);
                    break;
                }
            }
        }
    }

    let mut comment_count = 0;
    // Transform the entries into generic rows that can be aligned.
    let rows = entry_group
        .drain(0..)
        .map(|e| {
            let mut row = Vec::with_capacity(5);

            row.push(context.indent(options).collect::<String>());
            row.push(e.key);
            row.push("=".to_string());
            row.push(e.value);
            if let Some(c) = e.comment {
                row.push(c);
                comment_count += 1;
            }

            row
        })
        .collect::<Vec<_>>();

    let align_comments = options.should_align_comments(comment_count);
    *formatted += &format_rows(
        if !options.align_entries && !align_comments {
            0..0
        } else if !options.align_entries && align_comments {
            3..usize::MAX
        } else if options.align_entries && !align_comments {
            0..3
        } else {
            0..usize::MAX
        },
        if options.compact_entries {
            3..usize::MAX
        } else {
            1..usize::MAX
        },
        &rows,
        options.newline(),
        " ",
    );

    were_entries
}
```

## Docstring

_No docstring provided._

## Source

```
fn add_entries(
    entry_group: &mut Vec<FormattedEntry>,
    formatted: &mut String,
    options: &Options,
    context: &Context,
) -> bool {
    let were_entries = !entry_group.is_empty();

    if options.reorder_keys {
        entry_group.sort();
    }

    let indent_chars_count = context.indent_level * options.indent_string.chars().count();

    // We check for too long lines, and try to expand them if possible.
    // We don't take vertical alignment into account for simplicity.
    if options.array_auto_expand {
        for entry in entry_group.iter_mut() {
            let comment_chars_count = entry
                .comment
                .as_ref()
                .map(
                    |c| c.chars().count() + 1, // account for the separator ' ' as well
                )
                .unwrap_or(0);

            let line_count = entry.value.split('\n').count();

            // check each line of the value
            // for the first line we include the actual indent, key, and the eq parts as well
            for (idx, line) in entry.value.split('\n').enumerate() {
                let mut chars_count = line.chars().count();
                if idx == 0 {
                    chars_count += indent_chars_count;
                    chars_count += entry.key.chars().count();
                    chars_count += if options.compact_entries { 1 } else { 3 }; // " = "
                }

                // Include comment in the last line.
                if idx == line_count - 1 {
                    chars_count += comment_chars_count;
                }

                if chars_count > options.column_width {
                    let mut context = context.clone();
                    context.force_multiline = true;

                    // too long, reformat the value of the entry
                    let value = format_value(
                        entry
                            .syntax
                            .as_node()
                            .unwrap()
                            .children()
                            .find(|n| n.kind() == VALUE)
                            .unwrap(),
                        options,
                        &context,
                    );

                    entry.value.clear();

                    if let Some(c) = value.trailing_comment() {
                        debug_assert!(
                            entry.comment.is_none() || entry.comment.clone().unwrap() == c
                        );
                        entry.comment = Some(c);
                    }

                    value.write_to(&mut entry.value, options);
                    break;
                }
            }
        }
    }

    let mut comment_count = 0;
    // Transform the entries into generic rows that can be aligned.
    let rows = entry_group
        .drain(0..)
        .map(|e| {
            let mut row = Vec::with_capacity(5);

            row.push(context.indent(options).collect::<String>());
            row.push(e.key);
            row.push("=".to_string());
            row.push(e.value);
            if let Some(c) = e.comment {
                row.push(c);
                comment_count += 1;
            }

            row
        })
        .collect::<Vec<_>>();

    let align_comments = options.should_align_comments(comment_count);
    *formatted += &format_rows(
        if !options.align_entries && !align_comments {
            0..0
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
