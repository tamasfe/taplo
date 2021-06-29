//! This module is used to format TOML.
//!
//! The formatting can be done on documents that might
//! contain invalid syntax. In that case the invalid part is skipped.

use crate::{
    dom::{Cast, KeyNode, NodeSyntax, RootNode},
    syntax::{SyntaxElement, SyntaxKind::*, SyntaxNode},
};
use rowan::{GreenNode, NodeOrToken, TextRange};
use std::{
    cmp,
    iter::{repeat, FromIterator},
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
    let mut entry_group: Vec<FormattedEntry> = Vec::new();

    let mut formatted = String::new();

    // We defer printing the entries so that we can align them vertically.
    // Whenever an entry is added to the group, we skip its trailing newline,
    // otherwise it would be inserted before the entries.
    let mut skip_newlines = 0;

    let mut context = context.clone();

    // Table key for determining indents
    let mut last_table_key = None;

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
                    entry_group.push(format_entry(node, &options, &context));
                    skip_newlines += 1;
                }
                _ => unreachable!(),
            },
            NodeOrToken::Token(token) => match token.kind() {
                NEWLINE => {
                    let newline_count = token.text().newline_count();

                    if newline_count > 1 {
                        add_entries(&mut entry_group, &mut formatted, &options, &context);
                        skip_newlines = 0;
                    }

                    formatted.extend(options.newlines(newline_count.saturating_sub(skip_newlines)));
                }
                COMMENT => {
                    formatted += token.text();
                }
                WHITESPACE => {}
                _ => unreachable!(),
            },
        }
    }

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

    // Maximum characters entry/value for alignment.
    let max_widths = if options.align_entries {
        let mut w = (0, 0);

        for entry in entry_group.iter() {
            w.0 = cmp::max(entry.key.chars().count(), w.0);
            w.1 = cmp::max(entry.value.chars().count(), w.1);
        }

        w
    } else {
        (0, 0)
    };

    for (i, entry) in entry_group.drain(0..).enumerate() {
        if i != 0 {
            *formatted += options.newline();
        }
        formatted.extend(context.indent(options));
        *formatted += &entry.key;
        formatted.extend(repeat(' ').take(max_widths.0 - entry.key.chars().count()));

        *formatted += " = ";
        *formatted += &entry.value;

        if let Some(c) = entry.comment {
            formatted.extend(repeat(' ').take(max_widths.1 - entry.value.chars().count()));
            *formatted += " ";
            *formatted += &c;
        }
    }
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
                    format_array(n, options, context);
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
    node.descendants_with_tokens()
        .any(|n| n.kind() == NEWLINE || n.kind() == COMMENT)
}

fn format_array(node: SyntaxNode, options: &Options, context: &Context) -> impl FormattedItem {
    let mut multiline = is_array_multiline(&node);

    let mut formatted = String::new();
    let mut trailing_comment = None;

    if options.array_auto_collapse && !context.force_multiline {
        multiline = false;
    }

    if node.children().count() == 0 {
        formatted = "[]".into();
    }

    let mut node_index = 0;
    for c in node.children_with_tokens() {
        match c {
            NodeOrToken::Node(n) => match n.kind() {
                VALUE => {
                    // let val = 
                    if node_index != 0 {

                    }

                    node_index += 1;
                }
                _ => unreachable!(),
            },
            NodeOrToken::Token(t) => match t.kind() {
                BRACKET_START => {
                    formatted += "[";
                    if !options.compact_arrays {
                        formatted += " ";
                    }
                }
                BRACKET_END => {
                    if !options.compact_arrays {
                        formatted += " ";
                    }
                    formatted += "]";
                }
                NEWLINE => {
                    if !formatted.ends_with("]") {
                        formatted.extend(options.newlines(t.text().newline_count()));
                    }
                }
                COMMENT => {
                    if formatted.ends_with('[') {
                        formatted += " ";
                        formatted += t.text();
                    } else if formatted.ends_with(']') {
                        trailing_comment = Some(t.text().into());
                    } else {
                        formatted.extend(context.indent(options));
                        formatted += t.text();
                    }
                }
                _ => {}
            },
        }
    }

    formatted.extend(context.indent(options));

    if !options.compact_arrays {
        formatted += " ";
    }
    formatted += "]";

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
