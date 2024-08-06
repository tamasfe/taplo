use self::{error::QueryError, from_syntax::keys_from_syntax, node::Key};
use crate::{parser::Parser, syntax::SyntaxElement, util::join_ranges, HashMap};
use core::iter::once;
use std::{
    iter::{empty, FromIterator},
    str::FromStr,
    sync::Arc,
};

#[cfg(feature = "serde")]
mod serde;

pub(crate) mod from_syntax;

pub mod error;
pub mod index;
pub mod node;
pub mod rewrite;
mod to_toml;

pub use error::Error;
pub use from_syntax::FromSyntax;
use itertools::Itertools;
pub use node::Node;
use once_cell::unsync::OnceCell;
use rowan::TextRange;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyOrIndex {
    Key(Key),
    Index(usize),
}

impl<N> From<N> for KeyOrIndex
where
    N: Into<usize>,
{
    fn from(v: N) -> Self {
        Self::Index(v.into())
    }
}

impl From<Key> for KeyOrIndex {
    fn from(v: Key) -> Self {
        Self::Key(v)
    }
}

impl PartialEq<str> for KeyOrIndex {
    fn eq(&self, other: &str) -> bool {
        match self {
            KeyOrIndex::Key(k) => k.value() == other,
            KeyOrIndex::Index(_) => false,
        }
    }
}

impl core::fmt::Display for KeyOrIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyOrIndex::Key(v) => v.fmt(f),
            KeyOrIndex::Index(v) => v.fmt(f),
        }
    }
}

impl KeyOrIndex {
    /// Returns `true` if the key or index is [`Key`].
    ///
    /// [`Key`]: KeyOrIndex::Key
    pub fn is_key(&self) -> bool {
        matches!(self, Self::Key(..))
    }

    /// Returns `true` if the key or index is [`Index`].
    ///
    /// [`Index`]: KeyOrIndex::Index
    pub fn is_index(&self) -> bool {
        matches!(self, Self::Index(..))
    }

    pub fn as_key(&self) -> Option<&Key> {
        if let Self::Key(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_index(&self) -> Option<&usize> {
        if let Self::Index(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Keys {
    dotted: Arc<str>,
    keys: Arc<[KeyOrIndex]>,
}

impl Keys {
    #[inline]
    pub fn empty() -> Self {
        Self::new(empty())
    }

    pub fn single(key: impl Into<KeyOrIndex>) -> Self {
        Self::new(once(key.into()))
    }

    pub fn new(keys: impl Iterator<Item = KeyOrIndex>) -> Self {
        let keys: Arc<[KeyOrIndex]> = keys.collect();
        let dotted: Arc<str> = Arc::from(keys.iter().join(".").as_str());
        Self { keys, dotted }
    }

    pub fn join(&self, key: impl Into<KeyOrIndex>) -> Self {
        self.extend(once(key.into()))
    }

    pub fn extend<I, K>(&self, keys: I) -> Self
    where
        I: IntoIterator<Item = K>,
        K: Into<KeyOrIndex>,
    {
        Self::new(
            self.keys
                .iter()
                .cloned()
                .chain(keys.into_iter().map(Into::into)),
        )
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &KeyOrIndex> + DoubleEndedIterator {
        self.keys.iter()
    }

    pub fn dotted(&self) -> &str {
        &self.dotted
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.len() == 0
    }

    pub fn common_prefix_count(&self, other: &Self) -> usize {
        self.iter()
            .zip(other.iter())
            .take_while(|(a, b)| a == b)
            .count()
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.len() >= other.len() && self.common_prefix_count(other) == other.len()
    }

    pub fn part_of(&self, other: &Self) -> bool {
        other.contains(self)
    }

    pub fn skip_left(&self, n: usize) -> Self {
        Self::new(self.keys.iter().skip(n).cloned())
    }

    pub fn skip_right(&self, n: usize) -> Self {
        Self::new(self.keys.iter().rev().skip(n).cloned().rev())
    }

    pub fn all_text_range(&self) -> TextRange {
        join_ranges(
            self.keys
                .iter()
                .filter_map(KeyOrIndex::as_key)
                .flat_map(|k| k.text_ranges()),
        )
    }
}

impl IntoIterator for Keys {
    type Item = KeyOrIndex;

    type IntoIter = std::vec::IntoIter<KeyOrIndex>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(&*self.keys).into_iter()
    }
}

impl core::fmt::Display for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.dotted().fmt(f)
    }
}

impl FromStr for Keys {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = Parser::new(s).parse_key_only();
        if let Some(err) = p.errors.pop() {
            return Err(QueryError::InvalidKey(err).into());
        }
        Ok(Keys::new(
            keys_from_syntax(&p.into_syntax().into()).map(Into::into),
        ))
    }
}

impl PartialEq for Keys {
    fn eq(&self, other: &Self) -> bool {
        self.dotted == other.dotted
    }
}

impl Eq for Keys {}

impl std::hash::Hash for Keys {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.dotted.hash(state);
    }
}

impl From<Key> for Keys {
    fn from(key: Key) -> Self {
        Keys::new(once(key.into()))
    }
}

impl<N> From<N> for Keys
where
    N: Into<usize>,
{
    fn from(v: N) -> Self {
        Keys::new(once(v.into().into()))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Entries {
    pub(crate) lookup: HashMap<Key, Node>,
    pub(crate) all: Vec<(Key, Node)>,
}

impl Entries {
    pub fn len(&self) -> usize {
        self.all.len()
    }

    pub fn is_empty(&self) -> bool {
        self.all.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &(Key, Node)> {
        self.all.iter()
    }

    pub(crate) fn add(&mut self, key: Key, node: Node) {
        self.lookup.insert(key.clone(), node.clone());
        self.all.push((key, node));
    }
}

impl FromIterator<(Key, Node)> for Entries {
    fn from_iter<T: IntoIterator<Item = (Key, Node)>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let size = iter.size_hint().0;

        let mut lookup = HashMap::with_capacity(size);
        let mut all = Vec::with_capacity(size);

        for (k, n) in iter {
            lookup.insert(k.clone(), n.clone());
            all.push((k, n));
        }

        Self { lookup, all }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Comment {
    syntax: Option<SyntaxElement>,
    value: OnceCell<CommentValue>,
}

impl Comment {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            syntax: None,
            value: OnceCell::from(CommentValue::Comment(value.into())),
        }
    }

    pub fn new_directive(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            syntax: None,
            value: OnceCell::from(CommentValue::Directive {
                name: name.into(),
                value: value.into(),
            }),
        }
    }

    pub fn is_directive(&self) -> bool {
        self.value_internal().is_directive()
    }

    pub fn directive(&self) -> Option<&str> {
        if let CommentValue::Directive { name, .. } = self.value_internal() {
            Some(name)
        } else {
            None
        }
    }

    pub fn value(&self) -> &str {
        match self.value_internal() {
            CommentValue::Comment(s) => s,
            CommentValue::Directive { value, .. } => value,
        }
    }

    fn value_internal(&self) -> &CommentValue {
        self.value
            .get_or_init(|| match self.syntax.as_ref().and_then(|s| s.as_token()) {
                Some(t) => {
                    let text = t.text();

                    if let Some(directive_content) = text.strip_prefix("#:") {
                        let mut directive_content = directive_content.split_whitespace();
                        let directive_name = directive_content.next().unwrap_or("");
                        let directive_value = directive_content.next().unwrap_or("");
                        return CommentValue::Directive {
                            name: directive_name.into(),
                            value: directive_value.into(),
                        };
                    }

                    if let Some(comment_content) = text.strip_prefix('#') {
                        return CommentValue::Comment(comment_content.into());
                    }

                    Default::default()
                }
                None => Default::default(),
            })
    }
}

impl core::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = &self.syntax {
            s.fmt(f)
        } else {
            match self.value_internal() {
                CommentValue::Comment(c) => {
                    f.write_str("#")?;
                    c.fmt(f)
                }
                CommentValue::Directive { name, value } => {
                    f.write_str("#:")?;
                    name.fmt(f)?;
                    f.write_str(" ")?;
                    value.fmt(f)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CommentValue {
    Comment(String),
    Directive { name: String, value: String },
}

impl CommentValue {
    /// Returns `true` if the comment value is [`Directive`].
    ///
    /// [`Directive`]: CommentValue::Directive
    fn is_directive(&self) -> bool {
        matches!(self, Self::Directive { .. })
    }
}

impl Default for CommentValue {
    fn default() -> Self {
        Self::Comment(String::new())
    }
}
