//! This module is used to format TOML.
//!
//! The formatting can be done on documents that might
//! contain invalid syntax. In that case the invalid part is skipped.
// TODO This is fine for now, but will need a refactor
// if more features are added either to the toml spec.

use crate::{
    dom::{Cast, EntryNode, KeyNode, NodeSyntax, RootNode},
    syntax::{SyntaxKind, SyntaxKind::*, SyntaxNode, SyntaxToken},
};
use rowan::{GreenNode, GreenNodeBuilder, NodeOrToken, SmolStr, TextRange};
use std::{iter::FromIterator, mem, rc::Rc};

#[cfg(feature = "serde")]
use serde_crate::{Deserialize, Serialize};

use schemars::JsonSchema;

#[macro_use]
mod macros;

#[derive(Debug, Clone, Default)]
/// Scoped formatter options based on text ranges.
pub struct ScopedOptions(Vec<(TextRange, OptionsIncomplete)>);

impl FromIterator<(TextRange, OptionsIncomplete)> for ScopedOptions {
    fn from_iter<T: IntoIterator<Item = (TextRange, OptionsIncomplete)>>(iter: T) -> Self {
        Self(Vec::from_iter(iter.into_iter()))
    }
}

create_options!(
    /// All the formatting options.
    #[derive(Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "serde", serde(crate = "serde_crate"))]
    pub struct Options {
        /// Align entries vertically.
        ///
        /// Entries that have table headers, comments,
        /// or blank lines between them are not aligned.
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

        /// Alphabetically reorder keys that are not separated by blank lines.
        pub reorder_keys: bool,

        /// The maximum amount of consecutive blank lines allowed.
        pub allowed_blank_lines: usize,

        /// Use CRLF line endings
        pub crlf: bool,
    }
);

impl Default for Options {
    fn default() -> Self {
        Options {
            align_entries: false,
            array_trailing_comma: true,
            array_auto_expand: true,
            array_auto_collapse: true,
            compact_arrays: true,
            compact_inline_tables: false,
            column_width: 80,
            indent_tables: false,
            trailing_newline: true,
            allowed_blank_lines: 2,
            indent_string: "  ".into(),
            reorder_keys: false,
            crlf: false,
        }
    }
}

impl Options {
    fn newline(&self) -> SmolStr {
        if self.crlf {
            "\r\n".into()
        } else {
            "\n".into()
        }
    }

    fn newlines_like(&self, s: &str) -> SmolStr {
        self.newline()
            .repeat(usize::min(s.newline_count(), self.allowed_blank_lines + 1))
            .into()
    }

    fn indent_chars(&self, level: usize) -> usize {
        self.indent_string.repeat(level).chars().count()
    }
}

#[derive(Debug, Clone, Default)]
struct Context {
    indent_level: usize,
    line_char_count: usize,
    scopes: Rc<ScopedOptions>,
}

impl Context {
    fn update_options(&self, opts: &mut Options, range: TextRange) {
        for (r, s) in &self.scopes.0 {
            if r.contains_range(range) {
                opts.update(s.clone());
            }
        }
    }
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
    let mut s = format_impl(node, options.clone(), Context::default()).to_string();

    s = s.trim_end().into();

    if options.trailing_newline {
        s += options.newline().as_str();
    }

    s
}

/// Formats a DOM root node with given scopes.
///
/// **This doesn't check errors of the DOM.**
pub fn format_with_scopes(dom: RootNode, options: Options, scopes: ScopedOptions) -> String {
    let mut c = Context::default();
    c.scopes = Rc::new(scopes);

    let mut s = format_impl(
        dom.syntax().into_node().unwrap(),
        options.clone(),
        Context::default(),
    )
    .to_string();

    s = s.trim_end().into();

    if options.trailing_newline {
        s += options.newline().as_str();
    }

    s
}

/// Formats a DOM root node with given scopes.
///
/// All the scope keys must be valid glob patterns,
/// otherwise this function will panic!
///
/// **This doesn't check errors of the DOM.**
pub fn format_with_path_scopes<I: IntoIterator<Item = (String, OptionsIncomplete)>>(
    dom: RootNode,
    options: Options,
    scopes: I,
) -> String {
    let mut c = Context::default();

    let mut s = Vec::new();

    for (scope, opts) in scopes {
        let pat = glob::Pattern::new(&scope).unwrap();
        for (p2, node) in dom.iter() {
            if pat.matches(&p2.dotted()) {
                s.extend(node.text_ranges().into_iter().map(|r| (r, opts.clone())))
            }
        }
    }

    c.scopes = Rc::new(ScopedOptions::from_iter(s));

    let mut s = format_impl(dom.syntax().into_node().unwrap(), options.clone(), c).to_string();

    s = s.trim_end().into();

    if options.trailing_newline {
        s += options.newline().as_str();
    }

    s
}

// This is private because the layout of the formatted tree will
// not be compatible with the tree given as input.
fn format_impl(node: SyntaxNode, options: Options, mut context: Context) -> SyntaxNode {
    let kind: SyntaxKind = node.kind();

    let mut builder = GreenNodeBuilder::new();

    match kind {
        KEY => format_key(node, &mut builder, options, &mut context),
        VALUE => format_value(node, &mut builder, options, &mut context),
        TABLE_HEADER | TABLE_ARRAY_HEADER => {
            format_table_header(node, &mut builder, options, &mut context)
        }
        ENTRY => format_entry(node, &mut builder, options, &mut context),
        ARRAY => format_array(node, &mut builder, options, &mut context),
        INLINE_TABLE => format_inline_table(node, &mut builder, options, &mut context),
        ROOT => format_root(node, &mut builder, options, &mut context),
        _ => return node,
    };

    SyntaxNode::new_root(builder.finish())
}

// TODO(refactor)
#[allow(clippy::cognitive_complexity)]
fn format_root(
    node: SyntaxNode,
    builder: &mut GreenNodeBuilder,
    options: Options,
    context: &mut Context,
) {
    builder.start_node(ROOT.into());

    // Entries without a blank line between them
    let mut entry_group: Vec<SyntaxNode> = Vec::new();

    let mut indent_level: usize = 0;

    // Header keys with the indent level.
    // These are tracked for correct indentation.
    let mut indent_levels: Vec<(KeyNode, usize)> = Vec::new();

    // New line after each entry should be skipped,
    // because it is manually added if needed.
    let mut skip_newline = 0;

    for c in node.children_with_tokens() {
        let mut options = options.clone();
        context.update_options(&mut options, c.text_range());

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
                                &options.newline(),
                                if options.indent_tables {
                                    Some(&indent_str)
                                } else {
                                    None
                                },
                                if options.align_entries { None } else { Some(1) },
                            );
                            builder.token(NEWLINE.into(), options.newline());
                        }

                        if options.indent_tables {
                            // We surely have some...
                            if let Some(key_syntax) = n.first_child() {
                                if let Some(key) = KeyNode::cast(NodeOrToken::Node(key_syntax)) {
                                    // Search the previous tables for a common prefix
                                    // and indent based on that.
                                    indent_level = indent_levels
                                        .iter()
                                        .filter_map(|(k, level)| {
                                            if k.common_prefix_count(&key) > 0
                                                && k.key_count() <= key.key_count()
                                            {
                                                if k.key_count() == key.key_count() {
                                                    Some(*level)
                                                } else {
                                                    Some(k.common_prefix_count(&key))
                                                }
                                            } else {
                                                None
                                            }
                                        })
                                        .max()
                                        .unwrap_or(0);

                                    indent_levels.push((key.clone(), indent_level));
                                }
                            }
                            builder.token(
                                WHITESPACE.into(),
                                options.indent_string.repeat(indent_level).into(),
                            );
                        }
                        format_table_header(n, builder, options.clone(), context)
                    }
                    ENTRY => {
                        let mut entry_b = GreenNodeBuilder::new();
                        format_entry(
                            n,
                            &mut entry_b,
                            options.clone(),
                            &mut Context {
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
                    if t.text().as_str().newline_count() > 1 && options.allowed_blank_lines != 0 {
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
                                &options.newline(),
                                if options.indent_tables {
                                    Some(&indent_str)
                                } else {
                                    None
                                },
                                if options.align_entries { None } else { Some(1) },
                            );
                        }
                        builder.token(NEWLINE.into(), options.newlines_like(t.text().as_str()));
                    } else if skip_newline == 0 {
                        builder.token(NEWLINE.into(), options.newlines_like(t.text().as_str()));
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
                            &options.newline(),
                            if options.indent_tables {
                                Some(&indent_str)
                            } else {
                                None
                            },
                            if options.align_entries { None } else { Some(1) },
                        );
                        builder.token(NEWLINE.into(), options.newline());
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
        &options.newline(),
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
    options: Options,
    context: &mut Context,
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
                    format_entry(n, builder, options.clone(), context);
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
    options: Options,
    context: &mut Context,
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
        let mut prev_token: Option<SyntaxToken> = None;
        for (i, c) in node.children_with_tokens().enumerate() {
            match c {
                NodeOrToken::Node(n) => {
                    if node_index != 0 || was_comment {
                        if multiline {
                            builder.token(NEWLINE.into(), options.newline());
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
                        options.clone(),
                        &mut Context {
                            indent_level: context.indent_level + 1,
                            ..context.clone()
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
                NodeOrToken::Token(t) => {
                    match t.kind() {
                        BRACKET_START => {
                            builder.token(t.kind().into(), t.text().clone());

                            if multiline {
                                builder.token(NEWLINE.into(), options.newline());
                            } else if !options.compact_arrays {
                                builder.token(WHITESPACE.into(), " ".into());
                            }
                        }
                        BRACKET_END => {
                            if multiline {
                                builder.token(NEWLINE.into(), options.newline());
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
                            if i == all_token_count - 1 {
                                builder.token(WHITESPACE.into(), " ".into());
                                builder.token(t.kind().into(), t.text().clone());
                            // Comment after a value
                            } else {
                                if was_comment || was_value {
                                    if prev_token
                                        .take()
                                        .map(|t| t.kind() == NEWLINE)
                                        .unwrap_or(false)
                                    {
                                        builder.token(NEWLINE.into(), options.newline());
                                        builder.token(
                                            WHITESPACE.into(),
                                            options
                                                .indent_string
                                                .repeat(context.indent_level + 1)
                                                .into(),
                                        );
                                    } else {
                                        builder.token(WHITESPACE.into(), " ".into());
                                    }
                                // First line of the array
                                } else {
                                    builder.token(
                                        WHITESPACE.into(),
                                        options
                                            .indent_string
                                            .repeat(context.indent_level + 1)
                                            .into(),
                                    );
                                }

                                builder.token(t.kind().into(), t.text().clone());
                                was_comment = true;
                            }
                            was_value = false;
                        }
                        WHITESPACE | NEWLINE | COMMA => {}
                        _ => builder.token(t.kind().into(), t.text().clone()),
                    }
                    if t.kind() != WHITESPACE {
                        prev_token = Some(t.clone());
                    }
                }
            }
        }
    }

    builder.finish_node();
}

fn format_entry(
    node: SyntaxNode,
    builder: &mut GreenNodeBuilder,
    options: Options,
    context: &mut Context,
) {
    builder.start_node(ENTRY.into());

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                KEY => {
                    format_key(n, builder, options.clone(), context);
                    builder.token(WHITESPACE.into(), " ".into())
                }
                VALUE => format_value(n, builder, options.clone(), context),
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
    _options: Options,
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
    options: Options,
    context: &mut Context,
) {
    builder.start_node(VALUE.into());

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                ARRAY => format_array(n, builder, options.clone(), context),
                INLINE_TABLE => format_inline_table(n, builder, options.clone(), context),
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
    options: Options,
    context: &mut Context,
) {
    builder.start_node(node.kind().into());

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => {
                format_key(n, builder, options.clone(), context);
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
