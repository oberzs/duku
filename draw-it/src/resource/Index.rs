// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Index - reference-counted index into mesh storage

use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::rc::Rc;

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
