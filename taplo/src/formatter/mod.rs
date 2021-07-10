//! This module is used to format TOML.
//!
//! The formatting can be done on documents that might
//! contain invalid syntax. In that case the invalid part is skipped.

use crate::{
    dom::{Cast, KeyNode, NodeSyntax, RootNode},
    syntax::{SyntaxElement, SyntaxKind::*, SyntaxNode, SyntaxToken},
};
use rowan::{GreenNode, NodeOrToken, TextRange};
use std::{
    cmp,
    iter::{repeat, FromIterator},
    ops::Range,
    rc::Rc,
};

#[cfg(feature = "serde")]
use serde_crate::{Deserialize, Serialize};

#[cfg(feature = "schema")]
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

        /// Align consecutive comments after entries and items vertically.
        ///
        /// This applies to comments that are after entries or array items.
        pub align_comments: bool,

        /// Put trailing commas for multiline
        /// arrays.
        pub array_trailing_comma: bool,

        /// Automatically expand arrays to multiple lines
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
        /// This is best-effort and might not be accurate.
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

#[derive(Debug)]
pub enum OptionParseError {
    InvalidOption(String),
    InvalidValue {
        key: String,
        error: Box<dyn std::error::Error>,
    },
}

impl core::fmt::Display for OptionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid formatting option: {}",
            match self {
                OptionParseError::InvalidOption(k) => {
                    format!(r#"invalid option "{}""#, k)
                }
                OptionParseError::InvalidValue { key, error } => {
                    format!(r#"invalid value for option "{}": {}"#, key, error)
                }
            }
        )
    }
}

impl std::error::Error for OptionParseError {}

impl Default for Options {
    fn default() -> Self {
        Options {
            align_entries: false,
            align_comments: true,
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
    fn newline(&self) -> &'static str {
        if self.crlf {
            "\r\n".into()
        } else {
            "\n".into()
        }
    }

    fn newlines(&self, count: usize) -> impl Iterator<Item = &'static str> {
        repeat(self.newline()).take(usize::min(count, self.allowed_blank_lines + 1))
    }
}

#[derive(Debug, Clone, Default)]
struct Context {
    indent_level: usize,
    force_multiline: bool,
    scopes: Rc<ScopedOptions>,
}

impl Context {
    /// Update options based on the text range.
    fn update_options(&self, opts: &mut Options, range: TextRange) {
        for (r, s) in &self.scopes.0 {
            if r.contains_range(range) {
                opts.update(s.clone());
            }
        }
    }

    fn indent<'o>(&self, opts: &'o Options) -> impl Iterator<Item = &'o str> {
        repeat(opts.indent_string.as_ref()).take(self.indent_level)
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
        s += options.newline();
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
        s += options.newline();
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
        s += options.newline();
    }

    s
}

fn format_impl(node: SyntaxNode, options: Options, context: Context) -> String {
    assert!(node.kind() == ROOT);
    let mut formatted = format_root(node, &options, &context);

    if formatted.ends_with("\r\n") {
        formatted.truncate(formatted.len() - 2);
    } else if formatted.ends_with('\n') {
        formatted.truncate(formatted.len() - 1);
    }

    if options.trailing_newline {
        formatted += options.newline();
    }

    formatted
}

struct FormattedEntry {
    syntax: SyntaxElement,
    key: String,
    value: String,
    comment: Option<String>,
}

impl PartialEq for FormattedEntry {
    fn eq(&self, other: &Self) -> bool {
        self.key
            .replace('\'', "")
            .replace('"', "")
            .eq(&other.key.replace('\'', "").replace('"', ""))
    }
}

impl Eq for FormattedEntry {}

impl PartialOrd for FormattedEntry {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.key
            .replace('\'', "")
            .replace('"', "")
            .partial_cmp(&other.key.replace('\'', "").replace('"', ""))
    }
}

impl Ord for FormattedEntry {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.key
            .replace('\'', "")
            .replace('"', "")
            .cmp(&other.key.replace('\'', "").replace('"', ""))
    }
}

impl FormattedItem for FormattedEntry {
    fn write_to(&self, formatted: &mut String) {
        *formatted += &self.key;
        *formatted += " = ";
        *formatted += &self.value;
    }

    fn trailing_comment(&self) -> Option<String> {
        self.comment.clone()
    }

    fn syntax(&self) -> SyntaxElement {
        self.syntax.clone()
    }
}

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
    let mut last_table_key = None;

    fn add_comments(
        comments: &mut Vec<String>,
        formatted: &mut String,
        context: &Context,
        options: &Options,
    ) -> bool {
        let were_comments = !comments.is_empty();

        for (idx, comment) in comments.drain(0..).enumerate() {
            if idx != 0 {
                *formatted += &options.newline();
            }
            formatted.extend(context.indent(&options));
            *formatted += &comment;
        }

        were_comments
    }

    /// Special handling of blank lines;
    /// if the newlines are followed by whitespace, then a
    /// new line again, we skip handling of those newlines, and instead
    /// add them to the last batch before a value.
    fn dangling_newlines(t: SyntaxToken) -> Option<usize> {
        let newline_count = t.text().newline_count();

        if let Some(nt) = t.next_sibling_or_token() {
            if let Some(nnt) = nt.next_sibling_or_token() {
                if nt.kind() == WHITESPACE && nnt.kind() == NEWLINE {
                    return Some(newline_count);
                }
            }
        }

        None
    }

    let mut dangling_newline_count = 0;

    for c in node.children_with_tokens() {
        let mut options = options.clone();
        context.update_options(&mut options, c.text_range());

        match c {
            NodeOrToken::Node(node) => match node.kind() {
                TABLE_ARRAY_HEADER | TABLE_HEADER => {
                    if let Some(key) = node.first_child().map(Into::into).and_then(KeyNode::cast) {
                        if let Some(last_key) = last_table_key {
                            context.indent_level =
                                table_indent_level(&last_key, &key, context.indent_level);
                        }

                        last_table_key = Some(key);
                    }

                    if add_comments(&mut comment_group, &mut formatted, &context, &options) {
                        formatted += &options.newline();
                        skip_newlines = 0;
                    }

                    let header = format_table_header(node, &options, &context);
                    let comment = header.trailing_comment();

                    if options.indent_tables {
                        formatted.extend(context.indent(&options));
                    }

                    header.write_to(&mut formatted);
                    if let Some(c) = comment {
                        formatted += " ";
                        formatted += &c;
                    }
                }
                ENTRY => {
                    if add_comments(&mut comment_group, &mut formatted, &context, &options) {
                        formatted += &options.newline();
                        skip_newlines = 0;
                    }

                    entry_group.push(format_entry(node, &options, &context));
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
                        add_comments(&mut comment_group, &mut formatted, &context, &options);
                        add_entries(&mut entry_group, &mut formatted, &options, &context);
                        skip_newlines = 0;
                    }

                    formatted.extend(options.newlines(newline_count.saturating_sub(skip_newlines)));
                }
                COMMENT => {
                    add_entries(&mut entry_group, &mut formatted, &options, &context);
                    comment_group.push(token.text().to_string());
                    skip_newlines += 1;
                }
                WHITESPACE => {}
                _ => unreachable!(),
            },
        }
    }

    add_comments(&mut comment_group, &mut formatted, &context, options);
    add_entries(&mut entry_group, &mut formatted, options, &context);

    formatted
}

/// Determine the indentation level based on 2 consecutive table keys.
fn table_indent_level(key1: &KeyNode, key2: &KeyNode, indent: usize) -> usize {
    if key1 == key2 {
        return indent;
    }

    if key1.common_prefix_count(key2) > 0 && key2.contains(key1) {
        return indent + 1;
    } else if key1.common_prefix_count(key2) > 0 && key2.key_count() >= key1.key_count() {
        return indent;
    }

    if !key1.is_part_of(key2) && !key1.contains(key2) {
        return 0;
    }

    indent.saturating_sub(1)
}

/// Add entries to the formatted string.
fn add_entries(
    entry_group: &mut Vec<FormattedEntry>,
    formatted: &mut String,
    options: &Options,
    context: &Context,
) {
    if options.reorder_keys {
        entry_group.sort();
    }

    let indent_chars_count = context.indent_level * options.indent_string.chars().count();

    // We check for too long lines, and try to expand them
    // if possible.
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

            let line_count = entry.value.split("\n").count();

            // check each line of the value
            // for the first line we include the actual indent, key, and the eq parts as well
            for (idx, line) in entry.value.split("\n").enumerate() {
                let mut chars_count = line.chars().count();
                if idx == 0 {
                    chars_count += indent_chars_count;
                    chars_count += entry.key.chars().count();
                    chars_count += 3; // " = "
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

                    entry.comment = value.trailing_comment();
                    value.write_to(&mut entry.value);
                    break;
                }
            }
        }
    }

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
            }

            row
        })
        .collect::<Vec<_>>();

    *formatted += &format_rows(
        if !options.align_entries && !options.align_comments {
            0..0
        } else if !options.align_entries && options.align_comments {
            3..usize::MAX
        } else if options.align_entries && !options.align_comments {
            0..3
        } else {
            0..usize::MAX
        },
        &rows,
        options.newline(),
        " ",
    );
}

fn format_entry(node: SyntaxNode, options: &Options, context: &Context) -> FormattedEntry {
    let mut key = String::new();
    let mut value = String::new();
    let mut comment = None;

    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                KEY => {
                    format_key(n, &mut key, options, context);
                }
                VALUE => {
                    let val = format_value(n, options, context);
                    let c = val.trailing_comment();

                    if c.is_some() {
                        debug_assert!(comment.is_none());
                        comment = c;
                    }

                    val.write_to(&mut value);
                }
                _ => unreachable!(),
            },
            NodeOrToken::Token(t) => {
                if let COMMENT = t.kind() {
                    debug_assert!(comment.is_none());
                    comment = Some(t.text().into())
                }
            }
        }
    }

    FormattedEntry {
        syntax: node.into(),
        key,
        value,
        comment,
    }
}

fn format_key(node: SyntaxNode, formatted: &mut String, _options: &Options, _context: &Context) {
    // Idents and periods without whitespace
    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(_) => {}
            NodeOrToken::Token(t) => match t.kind() {
                WHITESPACE | NEWLINE => {}
                _ => {
                    *formatted += &t.text();
                }
            },
        }
    }
}

fn format_value(node: SyntaxNode, options: &Options, context: &Context) -> impl FormattedItem {
    let mut value = String::new();
    let mut comment = None;
    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                ARRAY => {
                    let formatted = format_array(n, options, context);

                    let c = formatted.trailing_comment();

                    if let Some(c) = c {
                        debug_assert!(comment.is_none());
                        comment = Some(c)
                    }

                    debug_assert!(value.is_empty());
                    formatted.write_to(&mut value);
                }
                INLINE_TABLE => {
                    let formatted = format_inline_table(n, options, context);

                    let c = formatted.trailing_comment();

                    if let Some(c) = c {
                        debug_assert!(comment.is_none());
                        comment = Some(c)
                    }

                    debug_assert!(value.is_empty());

                    formatted.write_to(&mut value);
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

fn format_inline_table(
    node: SyntaxNode,
    options: &Options,
    context: &Context,
) -> impl FormattedItem {
    let mut formatted = String::new();
    let mut comment = None;

    if node.children().count() == 0 {
        formatted = "{}".into();
    }

    let mut node_index = 0;
    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => {
                if node_index != 0 {
                    formatted += ", ";
                }

                let entry = format_entry(n, options, context);
                debug_assert!(entry.comment.is_none());
                entry.write_to(&mut formatted);

                node_index += 1;
            }
            NodeOrToken::Token(t) => match t.kind() {
                BRACE_START => {
                    formatted += "{";
                    if !options.compact_inline_tables {
                        formatted += " ";
                    }
                }
                BRACE_END => {
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
                _ => unreachable!(),
            },
        }
    }

    (node.into(), formatted, comment)
}

// Check whether the array spans multiple lines in its current form.
fn is_array_multiline(node: &SyntaxNode) -> bool {
    node.descendants_with_tokens().any(|n| n.kind() == NEWLINE)
}

fn can_collapse_array(node: &SyntaxNode) -> bool {
    !node.descendants_with_tokens().any(|n| n.kind() == COMMENT)
}

fn format_array(node: SyntaxNode, options: &Options, context: &Context) -> impl FormattedItem {
    let mut multiline = is_array_multiline(&node);

    let mut formatted = String::new();
    let mut trailing_comment = None;

    // We always try to collapse it if possible.
    if can_collapse_array(&node) && options.array_auto_collapse && !context.force_multiline {
        multiline = false;
    }

    // We use the same strategy as for entries, refer to [`format_root`].
    let mut skip_newlines = 0;

    // Formatted value, and optional trailing comment.
    // The value should also include the comma at the end if needed.
    let mut value_group: Vec<(String, Option<String>)> = Vec::new();

    let add_values = |value_group: &mut Vec<(String, Option<String>)>,
                      formatted: &mut String,
                      context: &Context|
     -> bool {
        let were_values = !value_group.is_empty();

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

        let rows = value_group
            .drain(0..)
            .map(|(value, comment)| {
                let mut row = Vec::with_capacity(5);

                row.push(context.indent(options).collect::<String>());
                row.push(value);
                if let Some(c) = comment {
                    row.push(c);
                }

                row
            })
            .collect::<Vec<_>>();

        *formatted += &format_rows(
            if options.align_comments {
                0..usize::MAX
            } else {
                0..0
            },
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

    let mut node_index = 0;
    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                VALUE => {
                    let val = format_value(n, options, &inner_context);
                    let mut val_string = String::new();

                    val.write_to(&mut val_string);

                    if node_index < node_count - 1 || (multiline && options.array_trailing_comma) {
                        val_string += ",";
                    }

                    value_group.push((val_string, val.trailing_comment()));
                    skip_newlines += 1;

                    node_index += 1;
                }
                _ => unreachable!(),
            },
            NodeOrToken::Token(t) => match t.kind() {
                BRACKET_START => {
                    formatted += "[";
                    if !options.compact_arrays && !multiline {
                        formatted += " ";
                    }
                }
                BRACKET_END => {
                    add_values(&mut value_group, &mut formatted, &inner_context);

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
                    let newline_count = t.text().newline_count();

                    if newline_count > 1 {
                        add_values(&mut value_group, &mut formatted, &inner_context);
                        skip_newlines = 0;
                    }

                    if !formatted.ends_with("]") {
                        formatted
                            .extend(options.newlines(newline_count.saturating_sub(skip_newlines)));
                    }
                }
                COMMENT => {
                    let newline_before = t
                        .siblings_with_tokens(rowan::Direction::Prev)
                        .skip(1) // skip is needed because the iterator includes the actual token
                        .filter(|s| s.kind() != WHITESPACE)
                        .next()
                        .map(|s| s.kind() == NEWLINE)
                        .unwrap_or(false);

                    if !newline_before && !value_group.is_empty() {
                        // It's actually trailing comment, so we add it to the last value.
                        value_group.last_mut().unwrap().1 = Some(t.text().to_string());
                        continue;
                    }

                    if add_values(&mut value_group, &mut formatted, &inner_context) {
                        formatted += options.newline();
                        skip_newlines = 0;
                    }

                    if formatted.ends_with(']') {
                        trailing_comment = Some(t.text().into());
                    } else {
                        formatted.extend(inner_context.indent(options));
                        formatted += t.text();
                    }
                }
                _ => {}
            },
        }
    }

    if node.children().count() == 0 {
        formatted = "[]".into();
    }

    (node.into(), formatted, trailing_comment)
}

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
                format_key(n, &mut &mut formatted, options, context);
            }
            NodeOrToken::Token(t) => match t.kind() {
                BRACKET_START | BRACKET_END => formatted += t.text(),
                WHITESPACE | NEWLINE => {}
                COMMENT => {
                    debug_assert!(comment.is_none());
                    comment = Some(t.text().to_string());
                }
                _ => unreachable!(),
            },
        }
    }

    (node.into(), formatted, comment)
}

// Simply a tuple of the formatted item and an optional trailing comment.
impl<T: AsRef<str>> FormattedItem for (SyntaxElement, T, Option<T>) {
    fn write_to(&self, formatted: &mut String) {
        *formatted += self.1.as_ref()
    }

    fn trailing_comment(&self) -> Option<String> {
        self.2.as_ref().map(|s| s.as_ref().to_string())
    }

    fn syntax(&self) -> SyntaxElement {
        self.0.clone()
    }
}

trait FormattedItem {
    fn syntax(&self) -> SyntaxElement;
    fn write_to(&self, formatted: &mut String);
    fn trailing_comment(&self) -> Option<String>;
}

trait NewlineCount {
    fn newline_count(&self) -> usize;
}

impl NewlineCount for &str {
    fn newline_count(&self) -> usize {
        self.chars().filter(|c| c == &'\n').count()
    }
}

// FIXME(docs)
fn format_rows<'r, R, S>(
    align_range: Range<usize>,
    rows: &[R],
    newline: &str,
    separator: &str,
) -> String
where
    R: AsRef<[S]>,
    S: AsRef<str>,
{
    let mut out = String::new();

    let diff_widths = |range: Range<usize>, row: &R| -> usize {
        let mut max_width = 0_usize;

        for row in rows {
            let row_len = row.as_ref().len();

            let range =
                cmp::min(range.start, row_len.saturating_sub(1))..cmp::min(range.end, row_len);

            max_width = cmp::max(
                max_width,
                row.as_ref()[range]
                    .iter()
                    .map(|s| s.as_ref().chars().count())
                    .sum(),
            );
        }

        let row_width = row.as_ref()[range]
            .iter()
            .map(|s| s.as_ref().chars().count())
            .sum::<usize>();

        max_width - row_width
    };

    for (row_idx, row) in rows.iter().enumerate() {
        if row_idx != 0 {
            out += newline;
        }

        let mut last_align_idx = 0_usize;

        for (item_idx, item) in row.as_ref().iter().enumerate() {
            // The first item is always indentation.
            if item_idx > 1 {
                out += separator;
            }

            out += item.as_ref();

            if align_range.start <= item_idx
                && align_range.end > item_idx
                && item_idx < row.as_ref().len() - 1
            {
                let diff = diff_widths(last_align_idx..item_idx + 1, row);
                out.extend(repeat(" ").take(diff));
                last_align_idx = item_idx + 1;
            }
        }
    }

    out
}
