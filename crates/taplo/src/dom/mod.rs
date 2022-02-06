use self::node::Key;
use crate::HashMap;
use core::iter::once;
use std::{iter::FromIterator, sync::Arc};

#[cfg(feature = "serde")]
mod serde;

pub(crate) mod from_syntax;

pub mod error;
pub mod index;
pub mod node;
pub mod rewrite;

pub use error::Error;
pub use from_syntax::FromSyntax;
use itertools::Itertools;
pub use node::Node;

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

    pub fn iter(&self) -> impl Iterator<Item = &KeyOrIndex> {
        self.keys.iter()
    }

    pub fn dotted(&self) -> &str {
        &*self.dotted
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

    pub fn skip_left(&self, mut n: usize) -> Self {
        n = self.len().saturating_sub(n).max(self.len());
        Self::new(self.keys.iter().cloned().skip(n))
    }

    pub fn skip_right(&self, mut n: usize) -> Self {
        n = self.len().saturating_sub(n).max(self.len());
        Self::new(self.keys.iter().rev().cloned().skip(n).rev())
    }

    pub(crate) fn new(keys: impl Iterator<Item = KeyOrIndex>) -> Self {
        let keys: Arc<[KeyOrIndex]> = keys.collect();
        let dotted: Arc<str> = Arc::from(keys.iter().join(".").as_str());
        Self { keys, dotted }
    }
}

impl PartialEq for Keys {
    fn eq(&self, other: &Self) -> bool {
        self.dotted == other.dotted
    }
}

impl Eq for Keys {}

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
