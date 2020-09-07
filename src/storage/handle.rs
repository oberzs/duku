// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Handle - reference-counted handle to a resource in storage

use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct Handle<T> {
    id: u32,
    counter: Rc<()>,
    marker: PhantomData<*const T>,
}

impl<T> Handle<T> {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            counter: Rc::new(()),
            marker: PhantomData,
            id,
        }
    }

    pub(crate) fn count(&self) -> usize {
        Rc::strong_count(&self.counter)
    }

    pub const fn id(&self) -> u32 {
        self.id
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handle({})", self.id)
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            counter: Rc::clone(&self.counter),
            marker: PhantomData,
        }
    }
}

impl<T> Eq for Handle<T> {}
unsafe impl<T> Send for Handle<T> {}
