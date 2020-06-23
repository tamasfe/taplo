//! This module is used to format TOML.
//!
//! The formatting can be done on documents that might
//! contain invalid syntax. In that case the invalid part is skipped.
// TODO This is fine for now, but will need a refactor
// of more features are added either to the formatter or the toml spec.

use crate::{
    dom::{Cast, EntryNode, KeyNode},
    syntax::{SyntaxKind, SyntaxKind::*, SyntaxNode},
};
use rowan::{GreenNode, GreenNodeBuilder, NodeOrToken, SmolStr};
use std::mem;

/// All the formatting options.
#[derive(Debug, Clone)]
pub struct Options {
    /// Align entries vertically.
    ///
    /// Entries that have table headers, comments,
    /// or empty lines between them are not aligned.
    pub align_entries: bool,

    /// Put trailing commas for multiline
    /// arrays
    pub array_trailing_comma: bool,

    /// Automatically expand arrays to multi lines
    /// if they're too long.
    pub array_auto_expand: bool,

    /// Automatically collapse arrays if they
    /// fit in one line.
    ///
    /// The array won't be collapsed if it
    /// contains a comment.
    pub array_auto_collapse: bool,

    /// Omit whitespace padding inside single-line arrays.
    pub compact_arrays: bool,

    /// Omit whitespace padding inside inline tables.
    pub compact_inline_tables: bool,

    /// Target maximum column width after which
    /// arrays are expanded into new lines.
    ///
    /// This is best-effort, and currently doesn't
    /// take whitespace into account.
    pub column_width: usize,

    /// Indent subtables if they come in order.
    pub indent_tables: bool,

    /// Indentation to use, should be tabs or spaces
    /// but technically could be anything.
    pub indent_string: String,

    /// Add trailing newline to the source.
    pub trailing_newline: bool,

    /// Alphabetically reorder keys that are not separated by empty lines.
    pub reorder_keys: bool,

    /// Use CRLF line endings
    pub crlf: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            align_entries: true,
            array_trailing_comma: true,
            array_auto_expand: true,
            array_auto_collapse: true,
            compact_arrays: true,
            compact_inline_tables: false,
            column_width: 80,
            indent_tables: true,
            trailing_newline: true,
            indent_string: "  ".into(),
            reorder_keys: true,
            crlf: false,
        }
    }
}

impl Options {
    fn newline(&self) -> &str {
        if self.crlf {
            "\r\n"
        } else {
            "\n"
        }
    }

    fn indent_chars(&self, level: usize) -> usize {
        self.indent_string.repeat(level).chars().count()
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct Context {
    indent_level: usize,
    line_char_count: usize,
}

/// Formats a parsed TOML green tree.
pub fn format_green(green: GreenNode, options: Options) -> String {
    format_syntax(SyntaxNode::new_root(green), options)
}

/// Parses then formats a TOML document, ignoring errors.
pub fn format(src: &str, options: Options) -> String {
    format_syntax(crate::parser::parse(src).into_syntax(), options)
}

/// Formats a parsed TOML syntax tree.
pub fn format_syntax(node: SyntaxNode, options: Options) -> String {
    let mut s = format_impl(node, options.clone()).to_string();

    s = s.trim_end().into();

    if options.trailing_newline {
        s += options.newline();
    }

    s
}

// This is private because the layout of the formatted tree will
// not be compatible with the tree given as input.
fn format_impl(node: SyntaxNode, options: Options) -> SyntaxNode {
    let kind: SyntaxKind = node.kind();

    let mut builder = GreenNodeBuilder::new();

    match kind {
        KEY => format_key(node, &mut builder, &options, &mut Context::default()),
        VALUE => format_value(node, &mut builder, &options, Context::default()),
        TABLE_HEADER | TABLE_ARRAY_HEADER => {
            format_table_header(node, &mut builder, &options, Context::default())
        }
        ENTRY => format_entry(node, &mut builder, &options, Context::default()),
        ARRAY => format_array(node, &mut builder, &options, Context::default()),
        INLINE_TABLE => format_inline_table(node, &mut builder, &options, Context::default()),
        ROOT => format_root(node, &mut builder, &options),
        _ => return node,
    };

    SyntaxNode::new_root(builder.finish())
}

// TODO(refactor)
#[allow(clippy::cognitive_complexity)]
fn format_root(node: SyntaxNode, builder: &mut GreenNodeBuilder, options: &Options) {
    builder.start_node(ROOT.into());

    // Entries without an empty line between them
    let mut entry_group: Vec<SyntaxNode> = Vec::new();

    // Needed for indentation to stop incorrectly indenting
    // tables that have no super table defined before them.
    let mut base_prefix_count = 0;

    let mut indent_level: usize = 0;
    let mut last_table_key: Option<KeyNode> = None;

    // New line after each entry should be skipped,
    // because it is manually added if needed.
    let mut skip_newline = 0;

    for c in node.children_with_tokens() {
        match c.clone() {
            NodeOrToken::Node(n) => {
                if n.descendants_with_tokens().any(|e| e.kind() == ERROR) {
                    add_all(n, builder);
                    continue;
                }

                match n.kind() {
                    TABLE_HEADER | TABLE_ARRAY_HEADER => {
                        let indent_str = options.indent_string.repeat(indent_level);

                        if options.reorder_keys {
                            entry_group.sort_by(|a, b| {
                                let ea = EntryNode::cast(NodeOrToken::Node(a.clone())).unwrap();
                                let eb = EntryNode::cast(NodeOrToken::Node(b.clone())).unwrap();

                                ea.key()
                                    .full_key_string()
                                    .partial_cmp(&eb.key().full_key_string())
                                    .unwrap()
                            });
                        }
                        if !entry_group.is_empty() {
                            add_aligned(
                                mem::take(&mut entry_group),
                                builder,
                                &newlines(options.newline(), Some(1)),
                                if options.indent_tables {
                                    Some(&indent_str)
                                } else {
                                    None
                                },
                                if options.align_entries { None } else { Some(1) },
                            );
                            builder.token(NEWLINE.into(), options.newline().into());
                        }

                        if options.indent_tables {
                            // We surely have some...
                            if let Some(key_syntax) = n.first_child() {
                                if let Some(key) = KeyNode::cast(NodeOrToken::Node(key_syntax)) {
                                    if let Some(last_key) = last_table_key.take() {
                                        if key.key_count() == 1
                                            || last_key.key_count() > key.key_count()
                                        {
                                            indent_level = 0
                                        } else if key != last_key {
                                            indent_level = last_key
                                                .common_prefix_count(&key)
                                                .checked_sub(base_prefix_count)
                                                .unwrap_or_default();
                                        }

                                        if indent_level == 0 {
                                            base_prefix_count = key.key_count() - 1;
                                        }
                                    }
                                    last_table_key = Some(key);
                                }
                            }
                            builder.token(
                                WHITESPACE.into(),
                                options.indent_string.repeat(indent_level).into(),
                            );
                        }
                        format_table_header(n, builder, options, Context::default())
                    }
                    ENTRY => {
                        let mut entry_b = GreenNodeBuilder::new();
                        format_entry(
                            n,
                            &mut entry_b,
                            options,
                            Context {
                                indent_level,
                                ..Default::default()
                            },
                        );
                        entry_group.push(extract_comment_from_entry(SyntaxNode::new_root(
                            entry_b.finish(),
                        )));
                        skip_newline += 1;
                    }
                    _ => {
                        if options.indent_tables {
                            builder.token(
                                WHITESPACE.into(),
                                options.indent_string.repeat(indent_level).into(),
                            );
                        }
                        add_all(n, builder);
                    }
                };
            }
            NodeOrToken::Token(t) => match t.kind() {
                NEWLINE => {
                    if t.text().as_str().newline_count() > 1 {
                        let indent_str = options.indent_string.repeat(indent_level);

                        if !entry_group.is_empty() {
                            if options.reorder_keys {
                                entry_group.sort_by(|a, b| {
                                    let ea = EntryNode::cast(NodeOrToken::Node(a.clone())).unwrap();
                                    let eb = EntryNode::cast(NodeOrToken::Node(b.clone())).unwrap();

                                    ea.key()
                                        .full_key_string()
                                        .partial_cmp(&eb.key().full_key_string())
                                        .unwrap()
                                });
                            }
                            add_aligned(
                                mem::take(&mut entry_group),
                                builder,
                                &newlines(options.newline(), Some(1)),
                                if options.indent_tables {
                                    Some(&indent_str)
                                } else {
                                    None
                                },
                                if options.align_entries { None } else { Some(1) },
                            );
                            builder.token(NEWLINE.into(), options.newline().into());
                        }
                        builder.token(NEWLINE.into(), options.newline().into());
                    }

                    if skip_newline == 0 {
                        builder.token(NEWLINE.into(), newlines(options.newline(), None));
                    }
                    skip_newline = i32::max(0, skip_newline - 1);
                }
                COMMENT => {
                    let indent_str = options.indent_string.repeat(indent_level);

                    if options.reorder_keys {
                        entry_group.sort_by(|a, b| {
                            let ea = EntryNode::cast(NodeOrToken::Node(a.clone())).unwrap();
                            let eb = EntryNode::cast(NodeOrToken::Node(b.clone())).unwrap();

                            ea.key()
                                .full_key_string()
                                .partial_cmp(&eb.key().full_key_string())
                                .unwrap()
                        });
                    }
                    if !entry_group.is_empty() {
                        add_aligned(
                            mem::take(&mut entry_group),
                            builder,
                            &newlines(options.newline(), Some(1)),
                            if options.indent_tables {
                                Some(&indent_str)
                            } else {
                                None
                            },
                            if options.align_entries { None } else { Some(1) },
                        );
                        builder.token(NEWLINE.into(), options.newline().into());
                    }

                    if options.indent_tables {
                        builder.token(
                            WHITESPACE.into(),
                            options.indent_string.repeat(indent_level).into(),
                        );
                    }
                    builder.token(t.kind().into(), t.text().clone())
                }
                WHITESPACE => {}
                _ => {
                    if options.indent_tables {
                        builder.token(
                            WHITESPACE.into(),
                            options.indent_string.repeat(indent_level).into(),
                        );
                    }
                    builder.token(t.kind().into(), t.text().clone())
                }
            },
        }
    }

    let indent_str = options.indent_string.repeat(indent_level);

    if options.reorder_keys {
        entry_group.sort_by(|a, b| {
            let ea = EntryNode::cast(NodeOrToken::Node(a.clone())).unwrap();
            let eb = EntryNode::cast(NodeOrToken::Node(b.clone())).unwrap();

            ea.key()
                .full_key_string()
                .partial_cmp(&eb.key().full_key_string())
                .unwrap()
        });
    }
    add_aligned(
        mem::take(&mut entry_group),
        builder,
        options.newline(),
        if options.indent_tables {
            Some(&indent_str)
        } else {
            None
        },
        if options.align_entries { None } else { Some(1) },
    );

    builder.finish_node();
}

fn format_inline_table(
    node: SyntaxNode,
    builder: &mut GreenNodeBuilder,
    options: &Options,
    context: Context,
) {
    builder.start_node(INLINE_TABLE.into());

    if node.children().count() == 0 {
        builder.token(BRACE_START.into(), "{".into());
        builder.token(BRACE_END.into(), "}".into());
    } else {
        let mut has_previous = false;
        for c in node.children_with_tokens() {
            match c {
                NodeOrToken::Node(n) => {
                    if has_previous {
                        builder.token(COMMA.into(), ",".into());
                        builder.token(WHITESPACE.into(), " ".into());
                    }
                    format_entry(n, builder, options, context);
                    has_previous = true;
                }
                NodeOrToken::Token(t) => match t.kind() {
                    BRACE_START => {
                        builder.token(t.kind().into(), t.text().clone());
                        if !options.compact_inline_tables {
                            builder.token(WHITESPACE.into(), " ".into());
                        }
                    }
                    BRACE_END => {
                        if !options.compact_inline_tables {
                            builder.token(WHITESPACE.into(), " ".into());
                        }
                        builder.token(t.kind().into(), t.text().clone());
                    }
                    WHITESPACE | NEWLINE | COMMA => {}
                    _ => builder.token(t.kind().into(), t.text().clone()),
                },
            }
        }
    }

    builder.finish_node();
}

fn format_array(
    node: SyntaxNode,
    builder: &mut GreenNodeBuilder,
    options: &Options,
    context: Context,
) {
    builder.start_node(ARRAY.into());

    let (token_count, char_count, has_newline) = node
        .descendants_with_tokens()
        .filter(|t| match t {
            NodeOrToken::Node(_) => false,
            NodeOrToken::Token(t) => t.kind() != WHITESPACE,
        })
        .fold((0, 0, false), |(mut count, mut len, mut has_newline), e| {
            len += u32::from(e.text_range().len());
            count += 1;

            if e.kind() == NEWLINE {
                count -= 1;
                has_newline = true;
            }

            (count, len, has_newline)
        });

    let child_count = node.children().count();
    let all_token_count = node.children_with_tokens().count();

    let has_comment_inside = node
        .children_with_tokens()
        .enumerate()
        .map(|(i, c)| match c {
            NodeOrToken::Node(n) => n.descendants_with_tokens().any(|d| d.kind() == COMMENT),
            NodeOrToken::Token(t) => i != all_token_count - 1 && t.kind() == COMMENT,
        })
        .any(|h| h);

    // Not counting newlines, and start and end brackets
    if token_count - 2 == 0 {
        builder.token(BRACKET_START.into(), "[".into());
        builder.token(BRACKET_END.into(), "]".into());
    } else {
        let too_long = char_count
            + context.line_char_count as u32
            + options.indent_chars(context.indent_level) as u32
            > options.column_width as u32;

        let multiline: bool =
            (has_comment_inside || has_newline || (options.array_auto_expand && too_long))
                && !(options.array_auto_collapse && !has_comment_inside && !too_long);

        let mut was_value = false;
        let mut was_comment = false;
        let mut node_index = 0;
        for (i, c) in node.children_with_tokens().enumerate() {
            match c {
                NodeOrToken::Node(n) => {
                    if node_index != 0 || was_comment {
                        if multiline {
                            builder.token(NEWLINE.into(), options.newline().into());
                        } else {
                            builder.token(WHITESPACE.into(), " ".into());
                        }
                    }

                    if multiline {
                        builder.token(
                            WHITESPACE.into(),
                            options
                                .indent_string
                                .repeat(context.indent_level + 1)
                                .into(),
                        );
                    }

                    let mut b = GreenNodeBuilder::new();

                    format_value(
                        n,
                        &mut b,
                        options,
                        Context {
                            indent_level: context.indent_level + 1,
                            ..context
                        },
                    );

                    let (val, comment) =
                        extract_comment_from_value(SyntaxNode::new_root(b.finish()));

                    add_all(val, builder);

                    if node_index != child_count - 1 || (multiline && options.array_trailing_comma)
                    {
                        builder.token(COMMA.into(), ",".into());
                    }

                    if let Some(comm) = comment {
                        builder.token(WHITESPACE.into(), " ".into());
                        builder.token(COMMENT.into(), comm.into());
                    } else {
                        was_value = true;
                    }

                    node_index += 1;
                }
                NodeOrToken::Token(t) => match t.kind() {
                    BRACKET_START => {
                        builder.token(t.kind().into(), t.text().clone());

                        if multiline {
                            builder.token(NEWLINE.into(), options.newline().into());
                        } else if !options.compact_arrays {
                            builder.token(WHITESPACE.into(), " ".into());
                        }
                    }
                    BRACKET_END => {
                        if multiline {
                            builder.token(NEWLINE.into(), options.newline().into());
                            builder.token(
                                WHITESPACE.into(),
                                options.indent_string.repeat(context.indent_level).into(),
                            );
                        } else if !options.compact_arrays {
                            builder.token(WHITESPACE.into(), " ".into());
                        }
                        builder.token(t.kind().into(), t.text().clone());
                    }
                    COMMENT => {
                        // Comment after the array
                        if i == all_token_count - 1 || was_value {
                            builder.token(WHITESPACE.into(), " ".into());
                            builder.token(t.kind().into(), t.text().clone());
                        // Comment after a value
                        } else {
                            if was_comment || was_value {
                                builder.token(NEWLINE.into(), options.newline().into());
                            }
                            builder.token(
                                WHITESPACE.into(),
                                options
                                    .indent_string
                                    .repeat(context.indent_level + 1)
                                    .into(),
                            );
                            builder.token(t.kind().into(), t.text().clone());
                            was_comment = true;
                        }
                        was_value = false;
                    }
                    WHITESPACE | NEWLINE | COMMA => {}
                    _ => builder.token(t.kind().into(), t.text().clone()),
                },
            }
        }
    }

    builder.finish_node();
}

fn format_entry(
    node: SyntaxNode,
    builder: &mut GreenNodeBuilder,
    options: &Options,
    mut context: Context,
) {
    builder.start_node(ENTRY.into());

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                KEY => {
                    format_key(n, builder, options, &mut context);
                    builder.token(WHITESPACE.into(), " ".into())
                }
                VALUE => format_value(n, builder, options, context),
                _ => add_all(n, builder),
            },
            NodeOrToken::Token(t) => match t.kind() {
                EQ => {
                    context.line_char_count += 1;
                    builder.token(EQ.into(), "=".into());
                    builder.token(WHITESPACE.into(), " ".into());
                }
                WHITESPACE | NEWLINE => {}
                _ => {
                    context.line_char_count += u32::from(t.text_range().len()) as usize;
                    builder.token(t.kind().into(), t.text().clone())
                }
            },
        }
    }
    builder.finish_node();
}

fn format_key(
    node: SyntaxNode,
    builder: &mut GreenNodeBuilder,
    _options: &Options,
    context: &mut Context,
) {
    builder.start_node(KEY.into());
    // Idents and periods without whitespace
    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(_) => {}
            NodeOrToken::Token(t) => match t.kind() {
                IDENT => {
                    context.line_char_count += u32::from(t.text_range().len()) as usize;
                    builder.token(IDENT.into(), t.text().clone())
                }
                PERIOD => {
                    context.line_char_count += u32::from(t.text_range().len()) as usize;
                    builder.token(PERIOD.into(), ".".into())
                }
                WHITESPACE | NEWLINE => {}
                _ => {
                    context.line_char_count += u32::from(t.text_range().len()) as usize;
                    builder.token(t.kind().into(), t.text().clone())
                }
            },
        }
    }

    builder.finish_node();
}

fn format_value(
    node: SyntaxNode,
    builder: &mut GreenNodeBuilder,
    options: &Options,
    context: Context,
) {
    builder.start_node(VALUE.into());

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                ARRAY => format_array(n, builder, options, context),
                INLINE_TABLE => format_inline_table(n, builder, options, context),
                _ => add_all(n, builder),
            },
            NodeOrToken::Token(t) => match t.kind() {
                NEWLINE | WHITESPACE => {}
                _ => builder.token(t.kind().into(), t.text().clone()),
            },
        }
    }

    builder.finish_node();
}

fn format_table_header(
    node: SyntaxNode,
    builder: &mut GreenNodeBuilder,
    options: &Options,
    mut context: Context,
) {
    builder.start_node(node.kind().into());

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => {
                format_key(n, builder, options, &mut context);
            }
            NodeOrToken::Token(t) => match t.kind() {
                BRACKET_START | BRACKET_END => builder.token(t.kind().into(), t.text().clone()),
                WHITESPACE | NEWLINE => {}
                COMMENT => {
                    builder.token(WHITESPACE.into(), " ".into());
                    builder.token(t.kind().into(), t.text().clone());
                }
                _ => builder.token(t.kind().into(), t.text().clone()),
            },
        }
    }

    builder.finish_node()
}

fn add_aligned(
    nodes: Vec<SyntaxNode>,
    builder: &mut GreenNodeBuilder,
    newline: &str,
    indent: Option<&str>,
    exact_tabs: Option<usize>,
) {
    let mut max_lengths: Vec<u32> = Vec::new();

    for node in &nodes {
        for (i, c) in node
            .children_with_tokens()
            .filter(|c| c.kind() != WHITESPACE)
            .enumerate()
        {
            let ts = c.text_range().len();

            if let Some(l) = max_lengths.get_mut(i) {
                *l = u32::max(*l, ts.into())
            } else {
                max_lengths.push(ts.into())
            }
        }
    }

    let node_count = nodes.len();

    for (i, node) in nodes.into_iter().enumerate() {
        builder.start_node(node.kind().into());

        if let Some(ind) = indent {
            builder.token(WHITESPACE.into(), ind.into());
        }

        for (i, c) in node
            .children_with_tokens()
            .filter(|c| c.kind() != WHITESPACE)
            .enumerate()
        {
            let ws_count = match &exact_tabs {
                Some(t) => *t,
                None => (max_lengths[i] - u32::from(c.text_range().len()) + 1) as usize,
            };

            match c {
                NodeOrToken::Node(n) => add_all(n, builder),
                NodeOrToken::Token(t) => {
                    builder.token(t.kind().into(), t.text().clone());
                }
            }

            if ws_count > 0 && i != max_lengths.len().checked_sub(1).unwrap_or_default() {
                builder.token(WHITESPACE.into(), " ".repeat(ws_count).into())
            }
        }

        builder.finish_node();

        if i != node_count - 1 {
            builder.token(NEWLINE.into(), newline.into());
        }
    }
}

fn add_all(node: SyntaxNode, builder: &mut GreenNodeBuilder) {
    builder.start_node(node.kind().into());

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => add_all(n, builder),
            NodeOrToken::Token(t) => builder.token(t.kind().into(), t.text().clone()),
        }
    }

    builder.finish_node()
}

fn newlines(s: &str, count: Option<usize>) -> SmolStr {
    if s.contains('\r') {
        if count.is_none() {
            if s.newline_count() > 2 {
                "\r\n\r\n".into()
            } else {
                "\r\n".into()
            }
        } else {
            "\r\n".repeat(count.unwrap_or(1)).into()
        }
    } else if count.is_none() {
        if s.newline_count() > 2 {
            "\n\n".into()
        } else {
            "\n".into()
        }
    } else {
        "\n".repeat(count.unwrap_or(1)).into()
    }
}

// Moves the comment from the value into the entry
fn extract_comment_from_entry(node: SyntaxNode) -> SyntaxNode {
    let mut b = GreenNodeBuilder::new();
    b.start_node(node.kind().into());

    let mut comment = None;

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(child_n) => match child_n.kind() {
                VALUE => {
                    b.start_node(VALUE.into());

                    for val_child in child_n.children_with_tokens() {
                        match val_child {
                            NodeOrToken::Node(n) => {
                                add_all(n, &mut b);
                            }
                            NodeOrToken::Token(t) => match t.kind() {
                                COMMENT => {
                                    comment = t.text().clone().into();
                                }
                                _ => {
                                    b.token(t.kind().into(), t.text().clone());
                                }
                            },
                        }
                    }

                    b.finish_node();
                }
                _ => {
                    add_all(child_n, &mut b);
                }
            },
            NodeOrToken::Token(child_t) => b.token(child_t.kind().into(), child_t.text().clone()),
        }
    }

    if let Some(c) = comment {
        b.token(COMMENT.into(), c)
    }

    b.finish_node();

    SyntaxNode::new_root(b.finish())
}

fn extract_comment_from_value(node: SyntaxNode) -> (SyntaxNode, Option<String>) {
    let mut b = GreenNodeBuilder::new();
    b.start_node(node.kind().into());

    let mut comment = None;

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                ARRAY | INLINE_TABLE => {
                    let has_comment = n
                        .children_with_tokens()
                        .last()
                        .map(|t| t.kind() == COMMENT)
                        .unwrap_or(false);

                    b.start_node(n.kind().into());
                    let child_count = n.children_with_tokens().count();
                    for (i, c2) in n.children_with_tokens().enumerate() {
                        match c2 {
                            NodeOrToken::Node(n2) => {
                                add_all(n2, &mut b);
                            }
                            NodeOrToken::Token(t2) => match t2.kind() {
                                COMMENT => comment = Some(t2.text().to_string()),
                                WHITESPACE => {
                                    if !has_comment || i < child_count - 2 {
                                        b.token(t2.kind().into(), t2.text().clone())
                                    }
                                }
                                _ => b.token(t2.kind().into(), t2.text().clone()),
                            },
                        }
                    }
                    b.finish_node();
                }
                _ => {
                    add_all(n, &mut b);
                }
            },
            NodeOrToken::Token(c) => match c.kind() {
                COMMENT => comment = Some(c.text().to_string()),
                _ => b.token(c.kind().into(), c.text().clone()),
            },
        }
    }

    b.finish_node();

    (SyntaxNode::new_root(b.finish()), comment)
}

trait NewlineCount {
    fn newline_count(&self) -> usize;
}

impl NewlineCount for &str {
    fn newline_count(&self) -> usize {
        self.chars().filter(|c| c == &'\n').count()
    }
}
