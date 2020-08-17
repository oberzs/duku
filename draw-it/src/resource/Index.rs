// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Index - reference-counted index into mesh storage

use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Eq)]
pub struct NewIndex(Rc<u32>);

impl NewIndex {
    pub(crate) fn new(pointer: u32) -> Self {
        Self(Rc::new(pointer))
    }

    pub(crate) fn count(&self) -> usize {
        Rc::strong_count(&self.0)
    }
}

impl fmt::Debug for NewIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Index({})", *self.0)
    }
}

impl PartialEq for NewIndex {
    fn eq(&self, other: &Self) -> bool {
        *self.0 == *other.0
    }
}

impl Hash for NewIndex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.deref().hash(state);
    }
}

// TODO: add pub(crate)
#[derive(Clone, Eq)]
pub struct Index {
    pointer: Rc<u32>,
    version: u32,
}

impl Index {
    pub(crate) fn new(pointer: u32) -> Self {
        Self {
            pointer: Rc::new(pointer),
            version: 0,
        }
    }

    pub(crate) fn count(&self) -> usize {
        Rc::strong_count(&self.pointer)
    }

    pub(crate) fn version(&self) -> u32 {
        self.version
    }

    pub(crate) fn bump(&mut self) {
        self.version = self.version.overflowing_add(1).0;
    }
}

impl fmt::Debug for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Index({})", *self.pointer)
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        *self.pointer == *other.pointer
    }
}

impl Hash for Index {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pointer.deref().hash(state);
    }
}
