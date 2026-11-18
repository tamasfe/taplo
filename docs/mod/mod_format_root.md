# mod.format_root

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 427-605)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn format_root(node: SyntaxNode, options: &Options, context: &Context) -> String {
    assert!(node.kind() == ROOT);
    let mut formatted = String::new();

    let mut entry_group: Vec<FormattedEntry> = Vec::new();

    // We defer printing the entries so that we can align them vertically.
    // Whenever an entry is added to the group, we skip its trailing newline,
    // otherwise the inserted new line would end up before the actual entries.
    let mut skip_newlines = 0;

    // We defer printing comments as well because we need to know
    // what comes after them for correct indentation.
    let mut comment_group: Vec<String> = Vec::new();

    let mut context = context.clone();

    // Table key for determining indents
    let mut table_key_indent_history: Vec<(Keys, usize)> = Vec::new();

    fn add_comments(
        comments: &mut Vec<String>,
        formatted: &mut String,
        context: &Context,
        options: &Options,
    ) -> bool {
        let were_comments = !comments.is_empty();

        for (idx, comment) in comments.drain(0..).enumerate() {
            if idx != 0 {
                *formatted += options.newline();
            }
            formatted.extend(context.indent(options));
            *formatted += &comment;
        }

        were_comments
    }

    let mut dangling_newline_count = 0;
    let mut scoped_options = options.clone();

    for c in node.children_with_tokens() {
        if context.error_at(c.text_range()) {
            formatted += &c.to_string();
            continue;
        }

        let c_range = c.text_range();

        match c {
            NodeOrToken::Node(node) => match node.kind() {
                TABLE_ARRAY_HEADER | TABLE_HEADER => {
                    if add_entries(&mut entry_group, &mut formatted, &scoped_options, &context) {
                        formatted += scoped_options.newline();
                        skip_newlines = 0;
                    }

                    scoped_options = options.clone();
                    context.update_options(&mut scoped_options, c_range);

                    // We treat everything as indented other than table headers from now on.
                    if scoped_options.indent_entries && context.indent_level == 0 {
                        context.indent_level = 1;
                    }

                    if let Some(key) = node.first_child().map(Into::into).map(Keys::from_syntax) {
                        if scoped_options.indent_tables {
                            context.indent_level = table_indent_level(
                                &table_key_indent_history,
                                &key,
                                if scoped_options.indent_entries { 1 } else { 0 },
                            );
                        }
                        table_key_indent_history.push((key.clone(), context.indent_level));
                    }

                    let mut header_context = context.clone();

                    if scoped_options.indent_entries {
                        header_context.indent_level = header_context.indent_level.saturating_sub(1);
                    }

                    if add_comments(
                        &mut comment_group,
                        &mut formatted,
                        &header_context,
                        &scoped_options,
                    ) {
                        formatted += scoped_options.newline();
                        skip_newlines = 0;
                    }

                    let header = format_table_header(node, &scoped_options, &header_context);
                    let comment = header.trailing_comment();

                    if scoped_options.indent_tables {
                        formatted.extend(header_context.indent(&scoped_options));
                    }

                    header.write_to(&mut formatted, &scoped_options);
                    if let Some(c) = comment {
                        formatted += " ";
                        formatted += &c;
                    }
                }
                ENTRY => {
                    scoped_options = options.clone();
                    context.update_options(&mut scoped_options, c_range);

                    if add_comments(
                        &mut comment_group,
                        &mut formatted,
                        &context,
                        &scoped_options,
                    ) {
                        formatted += scoped_options.newline();
                        skip_newlines = 0;
                    }

                    entry_group.push(format_entry(node, &scoped_options, &context));
                    skip_newlines += 1;
                }
                _ => unreachable!(),
            },
            NodeOrToken::Token(token) => match token.kind() {
                NEWLINE => {
                    let mut newline_count = token.text().newline_count();

                    match dangling_newlines(token.clone()) {
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
                        add_comments(
                            &mut comment_group,
                            &mut formatted,
                            &context,
                            &scoped_options,
                        );
                        add_entries(&mut entry_group, &mut formatted, &scoped_options, &context);
                        skip_newlines = 0;
                    }

                    formatted.extend(
                        scoped_options.newlines(newline_count.saturating_sub(skip_newlines)),
                    );
                }
                COMMENT => {
                    if add_entries(&mut entry_group, &mut formatted, &scoped_options, &context) {
                        formatted += scoped_options.newline();
                        skip_newlines = 0;
                    }
                    comment_group.push(token.text().to_string());
                    skip_newlines += 1;
                }
                WHITESPACE => {}
                _ => formatted += token.text(),
            },
        }
    }

    add_comments(
        &mut comment_group,
        &mut formatted,
        &context,
        &scoped_options,
    );
    add_entries(&mut entry_group, &mut formatted, &scoped_options, &context);

    formatted
}
```

## Docstring

_No docstring provided._

## Source

```
fn format_root(node: SyntaxNode, options: &Options, context: &Context) -> String {
    assert!(node.kind() == ROOT);
    let mut formatted = String::new();

    let mut entry_group: Vec<FormattedEntry> = Vec::new();

    // We defer printing the entries so that we can align them vertically.
    // Whenever an entry is added to the group, we skip its trailing newline,
    // otherwise the inserted new line would end up before the actual entries.
    let mut skip_newlines = 0;

    // We defer printing comments as well because we need to know
    // what comes after them for correct indentation.
    let mut comment_group: Vec<String> = Vec::new();

    let mut context = context.clone();

    // Table key for determining indents
    let mut table_key_indent_history: Vec<(Keys, usize)> = Vec::new();

    fn add_comments(
        comments: &mut Vec<String>,
        formatted: &mut String,
        context: &Context,
        options: &Options,
    ) -> bool {
        let were_comments = !comments.is_empty();

        for (idx, comment) in comments.drain(0..).enumerate() {
            if idx != 0 {
                *formatted += options.newline();
            }
            formatted.extend(context.indent(options));
            *formatted += &comment;
        }

        were_comments
    }

    let mut dangling_newline_count = 0;
    let mut scoped_options = options.clone();

    for c in node.children_with_tokens() {
        if context.error_at(c.text_range()) {
            formatted += &c.to_string();
            continue;
        }

        let c_range = c.text_range();

        match c {
            NodeOrToken::Node(node) => match node.kind() {
                TABLE_ARRAY_HEADER | TABLE_HEADER => {
                    if add_entries(&mut entry_group, &mut formatted, &scoped_options, &context) {
                        formatted += scoped_options.newline();
                        skip_newlines = 0;
                    }

                    scoped_options = options.clone();
                    context.update_options(&mut scoped_options, c_range);

                    // We treat everything as indented other than table headers from now on.
                    if scoped_options.indent_entries && context.indent_level == 0 {
                        context.indent_level = 1;
                    }

                    if let Some(key) = node.first_child().map(Into::into).map(Keys::from_syntax) {
                        if scoped_options.indent_tables {
                            context.indent_level = table_indent_level(
                                &table_key_indent_history,
                                &key,
                                if scoped_options.indent_entries { 1 } else { 0 },
                            );
                        }
                        table_key_indent_history.push((key.clone(), context.indent_level));
                    }

                    let mut header_context = context.clone();

                    if scoped_options.indent_entries {
                        header_context.indent_level = header_context.indent_level.saturating_sub(1);
                    }

                    if add_comments(
                        &mut comment_group,
                        &mut formatted,
                        &header_context,
                        &scoped_options,
                    ) {
                        formatted += scoped_options.newline();
                        skip_newlines = 0;
                    }

                    let header = format_table_header(node, &scoped_options, &header_context);
                    let comment = header.trailing_comment();

                    if scoped_options.indent_tables {
                        formatted.extend(header_context.indent(&scoped_options));
                    }

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
