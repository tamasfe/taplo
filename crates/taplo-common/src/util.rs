use globset::{Glob, GlobSetBuilder};
use serde_json::Value;
use std::{
    hash::{Hash, Hasher},
    path::Path,
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct GlobRule {
    include: globset::GlobSet,
    exclude: globset::GlobSet,
}

impl GlobRule {
    pub fn new(
        include: impl IntoIterator<Item = impl AsRef<str>>,
        exclude: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Self, anyhow::Error> {
        let mut inc = GlobSetBuilder::new();
        for glob in include {
            inc.add(Glob::new(glob.as_ref())?);
        }

        let mut exc = GlobSetBuilder::new();
        for glob in exclude {
            exc.add(Glob::new(glob.as_ref())?);
        }

        Ok(Self {
            include: inc.build()?,
            exclude: exc.build()?,
        })
    }

    pub fn is_match(&self, text: impl AsRef<Path>) -> bool {
        if !self.include.is_match(text.as_ref()) {
            return false;
        }

        !self.exclude.is_match(text.as_ref())
    }
}

#[derive(Eq)]
pub struct ArcHashValue(pub Arc<Value>);

impl Hash for ArcHashValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        HashValue(&*self.0).hash(state);
    }
}

impl PartialEq for ArcHashValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Eq)]
pub struct HashValue<'v>(pub &'v Value);

impl<'v> PartialEq for HashValue<'v> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<'v> Hash for HashValue<'v> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.0 {
            Value::Null => 0.hash(state),
            Value::Bool(v) => v.hash(state),
            Value::Number(v) => v.hash(state),
            Value::String(v) => v.hash(state),
            Value::Array(v) => {
                for v in v {
                    HashValue(v).hash(state);
                }
            }
            Value::Object(v) => {
                for (k, v) in v {
                    k.hash(state);
                    HashValue(v).hash(state);
                }
            }
        }
    }
}
