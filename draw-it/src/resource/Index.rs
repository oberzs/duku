// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Index - reference-counted index into mesh storage

use std::fmt;
use std::rc::Rc;

#[derive(Clone, Hash, PartialEq, Eq)]
pub(crate) struct Index(Rc<u32>);

impl Index {
    pub(crate) fn new(pointer: u32) -> Self {
        Self(Rc::new(pointer))
    }

    pub(crate) fn count(&self) -> usize {
        Rc::strong_count(&self.0)
    }
}

impl fmt::Debug for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Index({})", *self.0)
    }
}
