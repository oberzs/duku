// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Index - reference-counted index into storage

use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Eq)]
pub(crate) struct Index(Rc<u32>);

impl Index {
    pub(crate) fn new(pointer: u32) -> Self {
        Self(Rc::new(pointer))
    }

    pub(crate) fn count(&self) -> usize {
        Rc::strong_count(&self.0)
    }

    pub(crate) fn pointer(&self) -> u32 {
        *self.0
    }
}

impl fmt::Debug for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Index({})", *self.0)
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        *self.0 == *other.0
    }
}

impl Hash for Index {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.deref().hash(state);
    }
}
