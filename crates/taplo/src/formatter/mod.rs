//! This module is used to format TOML.
//!
//! The formatting can be done on documents that might
//! contain invalid syntax. In that case the invalid part is skipped.

use crate::{
    dom::{self, node::DomNode, FromSyntax, Keys, Node},
    syntax::{SyntaxElement, SyntaxKind::*, SyntaxNode, SyntaxToken},
    util::overlaps,
};
use itertools::Itertools;
use once_cell::unsync::OnceCell;
use rowan::{GreenNode, NodeOrToken, TextRange};
use std::{
    cmp,
    collections::VecDeque,
    iter::{repeat, FromIterator},
    ops::Range,
    rc::Rc,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

#[macro_use]
mod macros;

#[derive(Debug, Clone, Default)]
/// Scoped formatter options based on text ranges.
pub struct ScopedOptions(Vec<(TextRange, OptionsIncomplete)>);

impl FromIterator<(TextRange, OptionsIncomplete)> for ScopedOptions {
    fn from_iter<T: IntoIterator<Item = (TextRange, OptionsIncomplete)>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

create_options!(
    /// All the formatting options.
    #[derive(Debug, Clone, Eq, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

        /// If `align_comments` is true, apply the alignment in cases where
        /// there's only one comment.
        pub align_single_comments: bool,

        /// Put trailing commas for multiline
        /// arrays.
        pub array_trailing_comma: bool,

        /// Automatically expand arrays to multiple lines once they
        /// exceed the configured `column_width`.
        pub array_auto_expand: bool,

        /// Expand values (e.g.) inside inline tables
        /// where possible.
        pub inline_table_expand: bool,

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

        /// Omit whitespace around `=`.
        pub compact_entries: bool,

        /// Target maximum column width after which
        /// arrays are expanded into new lines.
        ///
        /// This is best-effort and might not be accurate.
        pub column_width: usize,

        /// Indent subtables if they come in order.
        pub indent_tables: bool,

        /// Indent entries under tables.
        pub indent_entries: bool,

        /// Indentation to use, should be tabs or spaces
        /// but technically could be anything.
        pub indent_string: String,

        /// Add trailing newline to the source.
        pub trailing_newline: bool,

        /// Alphabetically reorder keys that are not separated by blank lines.
        pub reorder_keys: bool,

        /// Alphabetically reorder array values that are not separated by blank lines.
        pub reorder_arrays: bool,

        /// Alphabetically reorder inline table values.
        pub reorder_inline_tables: bool,

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
        error: Box<dyn std::error::Error + Send + Sync>,
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
            align_single_comments: true,
            array_trailing_comma: true,
            array_auto_expand: true,
            array_auto_collapse: true,
            compact_arrays: true,
            compact_inline_tables: false,
            compact_entries: false,
            column_width: 80,
            indent_tables: false,
            indent_entries: false,
            inline_table_expand: true,
            trailing_newline: true,
            allowed_blank_lines: 2,
            indent_string: "  ".into(),
            reorder_keys: false,
            reorder_arrays: false,
            reorder_inline_tables: false,
            crlf: false,
        }
    }
}

impl Options {
    fn newline(&self) -> &'static str {
        if self.crlf {
            "\r\n"
        } else {
            "\n"
        }
    }

    fn newlines(&self, count: usize) -> impl Iterator<Item = &'static str> {
        repeat(self.newline()).take(usize::min(count, self.allowed_blank_lines + 1))
    }

    fn should_align_comments(&self, comment_count: usize) -> bool {
        (comment_count != 1 || self.align_single_comments) && self.align_comments
    }
}

#[derive(Debug, Clone)]
struct Context {
    indent_level: usize,
    force_multiline: bool,
    errors: Rc<[TextRange]>,
    scopes: Rc<ScopedOptions>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            indent_level: Default::default(),
            force_multiline: Default::default(),
            errors: Rc::from([]),
            scopes: Default::default(),
        }
    }
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

    fn error_at(&self, range: TextRange) -> bool {
        for error_range in self.errors.iter().copied() {
            if overlaps(range, error_range) {
                return true;
            }
        }

        false
    }

    fn indent<'o>(&self, opts: &'o Options) -> impl Iterator<Item = &'o str> {
        repeat(opts.indent_string.as_ref()).take(self.indent_level)
    }
}

/// Formats a parsed TOML green tree.
pub fn format_green(green: GreenNode, options: Options) -> String {
    format_syntax(SyntaxNode::new_root(green), options)
}

/// Parses then formats a TOML document, skipping ranges that contain syntax errors.
pub fn format(src: &str, options: Options) -> String {
    let p = crate::parser::parse(src);

    let ctx = Context {
        errors: p.errors.iter().map(|err| err.range).collect(),
        ..Context::default()
    };

    format_impl(p.into_syntax(), options, ctx)
}

/// Formats a parsed TOML syntax tree.
pub fn format_syntax(node: SyntaxNode, options: Options) -> String {
    let mut s = format_impl(node, options.clone(), Context::default());

    s = s.trim_end().into();

    if options.trailing_newline {
        s += options.newline();
    }

    s
}

/// Formats a DOM root node with given scopes.
///
/// **This doesn't check errors of the DOM.**
pub fn format_with_scopes(
    dom: Node,
    options: Options,
    errors: &[TextRange],
    scopes: ScopedOptions,
) -> String {
    let c = Context {
        scopes: Rc::new(scopes),
        errors: errors.into(),
        ..Context::default()
    };

    let mut s = format_impl(
        dom.syntax().unwrap().clone().into_node().unwrap(),
        options.clone(),
        c,
    );

    s = s.trim_end().into();

    if options.trailing_newline {
        s += options.newline();
    }

    s
}

/// Formats a DOM root node with given scopes.
///
/// **This doesn't check errors of the DOM.**
pub fn format_with_path_scopes<I, S>(
    dom: Node,
    options: Options,
    errors: &[TextRange],
    scopes: I,
) -> Result<String, dom::Error>
where
    I: IntoIterator<Item = (S, OptionsIncomplete)>,
    S: AsRef<str>,
{
    let mut c = Context {
        errors: errors.into(),
        ..Context::default()
    };

    let mut s = Vec::new();

    for (scope, opts) in scopes {
        let keys: Keys = scope.as_ref().parse()?;
        let matched = dom.find_all_matches(keys, false)?;

        for (_, node) in matched {
            s.extend(node.text_ranges(true).map(|r| (r, opts.clone())));
        }
    }

    c.scopes = Rc::new(ScopedOptions::from_iter(s));

    let mut s = format_impl(
        dom.syntax().unwrap().clone().into_node().unwrap(),
        options.clone(),
        c,
    );

    s = s.trim_end().into();

    if options.trailing_newline {
        s += options.newline();
    }

    Ok(s)
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
    /// This field is used to cache the "cleaned" version of the key and should only
    /// be accessed through the `cleaned_key` helpers method.
    cleaned_key: OnceCell<Vec<String>>,
    value: String,
    comment: Option<String>,
}

impl FormattedEntry {
    fn cleaned_key(&self) -> &Vec<String> {
        self.cleaned_key.get_or_init(|| {
            self.key
                .replace(['\'', '"'], "")
                .split('.')
                .map(ToOwned::to_owned)
                .collect()
        })
    }
}

impl PartialEq for FormattedEntry {
    fn eq(&self, other: &Self) -> bool {
        self.cleaned_key().eq(other.cleaned_key())
    }
}

impl Eq for FormattedEntry {}

impl PartialOrd for FormattedEntry {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FormattedEntry {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.cleaned_key().cmp(other.cleaned_key())
    }
}

impl FormattedItem for FormattedEntry {
    fn write_to(&self, formatted: &mut String, options: &Options) {
        *formatted += &self.key;
        if options.compact_entries {
            *formatted += "=";
        } else {
            *formatted += " = ";
        }
        *formatted += &self.value;
    }

    fn trailing_comment(&self) -> Option<String> {
        self.comment.clone()
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

/// Determine the indentation level using the indentation history.
///
/// The latest key that is a strict prefix is used and indented. If none is found, the default
/// indentation is used.
fn table_indent_level(
    history: &[(Keys, usize)],
    current_key: &Keys,
    default_indent: usize,
) -> usize {
    history
        .iter()
        .rev()
        .find_map(|(previous_key, indent)| {
            (current_key.contains(previous_key) && current_key != previous_key)
                .then_some(*indent + 1)
        })
        .unwrap_or(default_indent)
}

/// Add entries to the formatted string.
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

                    val.write_to(&mut value, options);
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
        cleaned_key: OnceCell::new(),
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
                    *formatted += t.text();
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
                    formatted.write_to(&mut value, options);
                }
                INLINE_TABLE => {
                    let formatted = format_inline_table(n, options, context);

                    let c = formatted.trailing_comment();

                    if let Some(c) = c {
                        debug_assert!(comment.is_none());
                        comment = Some(c)
                    }

                    debug_assert!(value.is_empty());

                    formatted.write_to(&mut value, options);
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
// Check whether the array spans multiple lines in its current form.
fn is_array_multiline(node: &SyntaxNode) -> bool {
    node.descendants_with_tokens().any(|n| n.kind() == NEWLINE)
}

fn can_collapse_array(node: &SyntaxNode) -> bool {
    !node.descendants_with_tokens().any(|n| n.kind() == COMMENT)
}

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

// Simply a tuple of the formatted item and an optional trailing comment.
impl<T: AsRef<str>> FormattedItem for (SyntaxElement, T, Option<T>) {
    fn write_to(&self, formatted: &mut String, _options: &Options) {
        *formatted += self.1.as_ref()
    }

    fn trailing_comment(&self) -> Option<String> {
        self.2.as_ref().map(|s| s.as_ref().to_string())
    }
}

trait FormattedItem {
    #[allow(clippy::ptr_arg)]
    fn write_to(&self, formatted: &mut String, options: &Options);
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
fn format_rows<R, S>(
    align_range: Range<usize>,
    separator_range: Range<usize>,
    rows: &[R],
    newline: &str,
    separator: &str,
) -> String
where
    R: AsRef<[S]>,
    S: AsRef<str>,
{
    let mut out = String::new();

    // We currently don't support vertical alignment of complex data.
    let can_align = rows
        .iter()
        .flat_map(|r| r.as_ref().iter())
        .all(|s| !s.as_ref().contains('\n'));

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
            if item_idx > separator_range.start
                && item_idx <= separator_range.end.saturating_add(1)
                && item_idx < row.as_ref().len()
            {
                out += separator;
            }

            out += item.as_ref();

            if can_align
                && align_range.start <= item_idx
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

/// Special handling of blank lines.
///
/// A design decision was made in the parser that newline (LF) characters
/// and whitespace (" ", and \t) are part of separate tokens.
///
/// Generally we count the amount of blank lines by counting LF characters in a token,
/// however if any of the consecutive blank lines contain empty characters,
/// this way of counting becomes unreliable.
///
/// So we check if the newlines are followed by whitespace,
/// then newlines again, and return the count here,
/// and we can add these values up.
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
