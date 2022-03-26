use std::{hash::Hash, sync::Arc};

use arc_swap::{ArcSwapOption, Guard};

#[derive(Debug)]
pub struct Shared<T>(ArcSwapOption<T>)
where
    T: Clone;

impl<T> Hash for Shared<T>
where
    T: Clone + Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.load().hash(state)
    }
}

impl<T> PartialEq for Shared<T>
where
    T: Clone + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.0.load() == *other.0.load()
    }
}

impl<T> Eq for Shared<T> where T: Clone + Eq {}

impl<T> Default for Shared<T>
where
    T: Clone + Default,
{
    fn default() -> Self {
        Self(ArcSwapOption::new(Some(Arc::new(T::default()))))
    }
}

impl<T> Shared<T>
where
    T: Clone,
{
    pub fn get(&self) -> Arc<T> {
        self.0.load_full().unwrap()
    }

    pub fn read(&self) -> SharedGuard<T> {
        SharedGuard(self.0.load())
    }

    pub(crate) fn new(value: T) -> Self {
        value.into()
    }

    pub(crate) fn update(&self, f: impl FnOnce(&mut T)) {
        let mut inner = self.0.load_full().take().unwrap();
        f(Arc::make_mut(&mut inner));
        self.0.store(Some(inner))
    }
}

impl<T: Clone> From<T> for Shared<T> {
    fn from(value: T) -> Self {
        Self(ArcSwapOption::new(Some(Arc::new(value))))
    }
}

pub struct SharedGuard<T: Clone>(Guard<Option<Arc<T>>>);

impl<T: Clone> std::ops::Deref for SharedGuard<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}
