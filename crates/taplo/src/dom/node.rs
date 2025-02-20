use std::iter::empty;

use crate::{
    private::Sealed,
    syntax::{SyntaxElement, SyntaxKind},
    util::shared::Shared,
};

mod nodes;
use either::Either;
pub use nodes::*;
use rowan::TextRange;

use super::{
    error::{Error, QueryError},
    index::Index,
    Comment, FromSyntax, KeyOrIndex, Keys,
};

pub trait DomNode: Sized + Sealed {
    fn syntax(&self) -> Option<&SyntaxElement>;
    fn errors(&self) -> &Shared<Vec<Error>>;
    fn validate_node(&self) -> Result<(), &Shared<Vec<Error>>>;
    fn is_valid_node(&self) -> bool {
        self.validate_node().is_ok()
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Table(Table),
    Array(Array),
    Bool(Bool),
    Str(Str),
    Integer(Integer),
    Float(Float),
    Date(DateTime),
    Invalid(Invalid),
}

impl Sealed for Node {}
impl DomNode for Node {
    fn syntax(&self) -> Option<&SyntaxElement> {
        match self {
            Node::Table(n) => n.syntax(),
            Node::Array(n) => n.syntax(),
            Node::Bool(n) => n.syntax(),
            Node::Str(n) => n.syntax(),
            Node::Integer(n) => n.syntax(),
            Node::Float(n) => n.syntax(),
            Node::Date(n) => n.syntax(),
            Node::Invalid(n) => n.syntax(),
        }
    }

    fn errors(&self) -> &Shared<Vec<Error>> {
        match self {
            Node::Table(n) => n.errors(),
            Node::Array(n) => n.errors(),
            Node::Bool(n) => n.errors(),
            Node::Str(n) => n.errors(),
            Node::Integer(n) => n.errors(),
            Node::Float(n) => n.errors(),
            Node::Date(n) => n.errors(),
            Node::Invalid(n) => n.errors(),
        }
    }

    fn validate_node(&self) -> Result<(), &Shared<Vec<Error>>> {
        match self {
            Node::Table(n) => n.validate_node(),
            Node::Array(n) => n.validate_node(),
            Node::Bool(n) => n.validate_node(),
            Node::Str(n) => n.validate_node(),
            Node::Integer(n) => n.validate_node(),
            Node::Float(n) => n.validate_node(),
            Node::Date(n) => n.validate_node(),
            Node::Invalid(n) => n.validate_node(),
        }
    }
}

impl Node {
    pub fn path(&self, keys: &Keys) -> Option<Node> {
        let mut node = self.clone();
        for key in keys.iter() {
            node = node.get(key);
        }

        if node.is_invalid() {
            None
        } else {
            Some(node)
        }
    }

    pub fn get(&self, idx: impl Index) -> Node {
        idx.index_into(self).unwrap_or_else(|| {
            Node::from(
                InvalidInner {
                    errors: Shared::from(Vec::from([Error::Query(QueryError::NotFound)])),
                    syntax: None,
                }
                .wrap(),
            )
        })
    }

    pub fn try_get(&self, idx: impl Index) -> Result<Node, Error> {
        idx.index_into(self)
            .ok_or(Error::Query(QueryError::NotFound))
    }

    pub fn get_matches(
        &self,
        pattern: &str,
    ) -> Result<impl ExactSizeIterator<Item = (KeyOrIndex, Node)>, Error> {
        let glob = globset::Glob::new(pattern)
            .map_err(QueryError::from)?
            .compile_matcher();
        let mut matched = Vec::new();

        match self {
            Node::Table(t) => {
                let entries = t.entries().read();
                for (key, node) in entries.iter() {
                    if glob.is_match(pattern) {
                        matched.push((KeyOrIndex::from(key.clone()), node.clone()));
                    }
                }
            }
            Node::Array(arr) => {
                let items = arr.items().read();
                for (idx, node) in items.iter().enumerate() {
                    if glob.is_match(idx.to_string()) {
                        matched.push((KeyOrIndex::from(idx), node.clone()));
                    }
                }
            }
            _ => {}
        }

        Ok(matched.into_iter())
    }

    /// Validate the node and then all children recursively.
    pub fn validate(&self) -> Result<(), impl Iterator<Item = Error> + core::fmt::Debug> {
        let mut errors = Vec::new();
        self.validate_all_impl(&mut errors);
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.into_iter())
        }
    }

    pub fn flat_iter(&self) -> impl DoubleEndedIterator<Item = (Keys, Node)> {
        let mut all = Vec::new();

        match self {
            Node::Table(t) => {
                let entries = t.inner.entries.read();
                for (key, entry) in &entries.all {
                    entry.collect_flat(Keys::from(key.clone()), &mut all);
                }
            }
            Node::Array(arr) => {
                let items = arr.inner.items.read();
                for (idx, item) in items.iter().enumerate() {
                    item.collect_flat(Keys::from(idx), &mut all);
                }
            }
            _ => {}
        }

        all.into_iter()
    }

    pub fn find_all_matches(
        &self,
        keys: Keys,
        include_children: bool,
    ) -> Result<impl ExactSizeIterator<Item = (Keys, Node)>, Error> {
        let mut all = self.flat_iter_impl();

        let mut err: Option<Error> = None;

        all.retain(|(k, _)| {
            if k.len() < keys.len() {
                return false;
            }

            let search_keys = keys.clone();
            let keys = k.clone();

            for (search_key, key) in search_keys.iter().zip(keys.iter()) {
                match search_key {
                    KeyOrIndex::Key(search_key) => {
                        let glob = match globset::Glob::new(search_key.value()) {
                            Ok(g) => g.compile_matcher(),
                            Err(glob_err) => {
                                err = Some(QueryError::from(glob_err).into());
                                return true;
                            }
                        };

                        match key {
                            KeyOrIndex::Key(key) => {
                                if !glob.is_match(key.value()) {
                                    return false;
                                }
                            }
                            KeyOrIndex::Index(idx) => {
                                if !glob.is_match(idx.to_string()) {
                                    return false;
                                }
                            }
                        }
                    }
                    KeyOrIndex::Index(search_idx) => match key {
                        KeyOrIndex::Key(_) => {
                            return false;
                        }
                        KeyOrIndex::Index(idx) => {
                            if idx != search_idx {
                                return false;
                            }
                        }
                    },
                }
            }

            true
        });

        if !include_children {
            all.retain(|(k, _)| k.len() == keys.len());
        }

        if let Some(err) = err {
            return Err(err);
        }

        Ok(all.into_iter())
    }

    pub fn text_ranges(&self, include_children: bool) -> impl ExactSizeIterator<Item = TextRange> {
        let mut ranges = Vec::with_capacity(1);

        match self {
            Node::Table(v) => {
                if include_children {
                    let entries = v.entries().read();

                    for (k, entry) in entries.iter() {
                        ranges.extend(k.text_ranges());
                        ranges.extend(entry.text_ranges(true));
                    }
                }

                if let Some(mut r) = v.syntax().map(|s| s.text_range()) {
                    for range in &ranges {
                        r = r.cover(*range);
                    }

                    ranges.insert(0, r);
                }
            }
            Node::Array(v) => {
                if include_children {
                    let items = v.items().read();
                    for item in items.iter() {
                        ranges.extend(item.text_ranges(true));
                    }
                }

                if let Some(mut r) = v.syntax().map(|s| s.text_range()) {
                    for range in &ranges {
                        r = r.cover(*range);
                    }

                    ranges.insert(0, r);
                }
            }
            Node::Bool(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Str(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Integer(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Float(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Date(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
            Node::Invalid(v) => ranges.push(v.syntax().map(|s| s.text_range()).unwrap_or_default()),
        }

        ranges.into_iter()
    }

    /// All the comments in the tree, including header comments returned from [`Self::header_comments`].
    pub fn comments(&self) -> impl Iterator<Item = Comment> {
        if let Some(syntax) = self.syntax().cloned().and_then(|s| s.into_node()) {
            Either::Left(
                syntax
                    .descendants_with_tokens()
                    .filter(|t| t.kind() == SyntaxKind::COMMENT)
                    .map(Comment::from_syntax),
            )
        } else {
            Either::Right(empty())
        }
    }

    /// Comments before the first item in the file.
    ///
    /// These are always counted from the root and the same
    /// values are returned from every node in the same tree.
    pub fn header_comments(&self) -> impl Iterator<Item = Comment> {
        let first_item = self
            .syntax()
            .and_then(|syntax| syntax.ancestors().last())
            .and_then(|root| root.descendants().nth(1));

        match first_item {
            Some(it) => Either::Left(self.comments().take_while(move |c| {
                c.syntax.as_ref().unwrap().text_range().end() <= it.text_range().start()
            })),
            None => Either::Right(self.comments()),
        }
    }

    fn flat_iter_impl(&self) -> Vec<(Keys, Node)> {
        let mut all = Vec::new();

        match self {
            Node::Table(t) => {
                let entries = t.inner.entries.read();
                for (key, entry) in &entries.all {
                    entry.collect_flat(Keys::from(key.clone()), &mut all);
                }
            }
            Node::Array(arr) => {
                let items = arr.inner.items.read();
                for (idx, item) in items.iter().enumerate() {
                    item.collect_flat(Keys::from(idx), &mut all);
                }
            }
            _ => {}
        }

        all
    }

    fn collect_flat(&self, parent: Keys, all: &mut Vec<(Keys, Node)>) {
        match self {
            Node::Table(t) => {
                all.push((parent.clone(), self.clone()));
                let entries = t.inner.entries.read();
                for (key, entry) in &entries.all {
                    entry.collect_flat(parent.join(key.clone()), all);
                }
            }
            Node::Array(arr) => {
                all.push((parent.clone(), self.clone()));
                let items = arr.inner.items.read();
                for (idx, item) in items.iter().enumerate() {
                    item.collect_flat(parent.join(idx), all);
                }
            }
            _ => {
                all.push((parent, self.clone()));
            }
        }
    }

    fn validate_all_impl(&self, errors: &mut Vec<Error>) {
        match self {
            Node::Table(v) => {
                if let Err(errs) = v.validate_node() {
                    errors.extend(errs.read().as_ref().iter().cloned())
                }

                let items = v.inner.entries.read();
                for (k, entry) in items.as_ref().all.iter() {
                    if let Err(errs) = k.validate_node() {
                        errors.extend(errs.read().as_ref().iter().cloned())
                    }
                    entry.validate_all_impl(errors);
                }
            }
            Node::Array(v) => {
                if let Err(errs) = v.validate_node() {
                    errors.extend(errs.read().as_ref().iter().cloned())
                }

                let items = v.inner.items.read();
                for item in &**items.as_ref() {
                    if let Err(errs) = item.validate_node() {
                        errors.extend(errs.read().as_ref().iter().cloned())
                    }
                }
            }
            Node::Bool(v) => {
                if let Err(errs) = v.validate_node() {
                    errors.extend(errs.read().as_ref().iter().cloned())
                }
            }
            Node::Str(v) => {
                if let Err(errs) = v.validate_node() {
                    errors.extend(errs.read().as_ref().iter().cloned())
                }
            }
            Node::Integer(v) => {
                if let Err(errs) = v.validate_node() {
                    errors.extend(errs.read().as_ref().iter().cloned())
                }
            }
            Node::Float(v) => {
                if let Err(errs) = v.validate_node() {
                    errors.extend(errs.read().as_ref().iter().cloned())
                }
            }
            Node::Date(v) => {
                if let Err(errs) = v.validate_node() {
                    errors.extend(errs.read().as_ref().iter().cloned())
                }
            }
            Node::Invalid(v) => {
                if let Err(errs) = v.validate_node() {
                    errors.extend(errs.read().as_ref().iter().cloned())
                }
            }
        }
    }
}

impl Node {
    /// Returns `true` if the node is [`Table`].
    ///
    /// [`Table`]: Node::Table
    pub fn is_table(&self) -> bool {
        matches!(self, Self::Table(..))
    }

    /// Returns `true` if the node is [`Array`].
    ///
    /// [`Array`]: Node::Array
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(..))
    }

    /// Returns `true` if the node is [`Bool`].
    ///
    /// [`Bool`]: Node::Bool
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(..))
    }

    /// Returns `true` if the node is [`Str`].
    ///
    /// [`Str`]: Node::Str
    pub fn is_str(&self) -> bool {
        matches!(self, Self::Str(..))
    }

    /// Returns `true` if the node is [`Integer`].
    ///
    /// [`Integer`]: Node::Integer
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(..))
    }

    /// Returns `true` if the node is [`Float`].
    ///
    /// [`Float`]: Node::Float
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(..))
    }

    /// Returns `true` if the node is [`Date`].
    ///
    /// [`Date`]: Node::Date
    pub fn is_date(&self) -> bool {
        matches!(self, Self::Date(..))
    }

    /// Returns `true` if the node is [`Invalid`].
    ///
    /// [`Invalid`]: Node::Invalid
    pub fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(..))
    }

    pub fn as_table(&self) -> Option<&Table> {
        if let Self::Table(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&Array> {
        if let Self::Array(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<&Bool> {
        if let Self::Bool(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_str(&self) -> Option<&Str> {
        if let Self::Str(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_integer(&self) -> Option<&Integer> {
        if let Self::Integer(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<&Float> {
        if let Self::Float(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_date(&self) -> Option<&DateTime> {
        if let Self::Date(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_invalid(&self) -> Option<&Invalid> {
        if let Self::Invalid(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_table(self) -> Result<Table, Self> {
        if let Self::Table(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_array(self) -> Result<Array, Self> {
        if let Self::Array(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_bool(self) -> Result<Bool, Self> {
        if let Self::Bool(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_str(self) -> Result<Str, Self> {
        if let Self::Str(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_integer(self) -> Result<Integer, Self> {
        if let Self::Integer(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_float(self) -> Result<Float, Self> {
        if let Self::Float(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_date(self) -> Result<DateTime, Self> {
        if let Self::Date(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn try_into_invalid(self) -> Result<Invalid, Self> {
        if let Self::Invalid(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

impl From<DateTime> for Node {
    fn from(v: DateTime) -> Self {
        Self::Date(v)
    }
}

impl From<Float> for Node {
    fn from(v: Float) -> Self {
        Self::Float(v)
    }
}

impl From<Integer> for Node {
    fn from(v: Integer) -> Self {
        Self::Integer(v)
    }
}

impl From<Str> for Node {
    fn from(v: Str) -> Self {
        Self::Str(v)
    }
}

impl From<Bool> for Node {
    fn from(v: Bool) -> Self {
        Self::Bool(v)
    }
}

impl From<Array> for Node {
    fn from(v: Array) -> Self {
        Self::Array(v)
    }
}

impl From<Table> for Node {
    fn from(v: Table) -> Self {
        Self::Table(v)
    }
}

impl From<Invalid> for Node {
    fn from(v: Invalid) -> Self {
        Self::Invalid(v)
    }
}
