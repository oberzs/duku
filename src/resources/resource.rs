// Oliver Berzs
// https://github.com/oberzs/duku

use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

pub(crate) struct Resource<T> {
    pub(crate) value: T,
    pub(crate) count: u32,
    pub(crate) mutated: bool,
}

/// A handle to a rendering resource.
///
/// Gives access to the underlying resource
/// through the [Deref](std::ops::Deref) trait.
///
/// Cloning this does not create a new resource
pub struct Handle<T> {
    _marker: PhantomData<*const T>,
    valid: bool,
    value: *mut T,
    count: *mut u32,
    mutated: *mut bool,
}

impl<T> Resource<T> {
    pub(crate) const fn new(value: T) -> Self {
        Self {
            count: 0,
            mutated: false,
            value,
        }
    }

    pub(crate) fn handle(&mut self) -> Handle<T> {
        self.count += 1;

        Handle {
            _marker: PhantomData::default(),
            valid: true,
            value: &mut self.value,
            count: &mut self.count,
            mutated: &mut self.mutated,
        }
    }
}

impl<T> Handle<T> {
    pub(crate) fn invalidate(&mut self) {
        self.valid = false;
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.value.as_ref().expect("bad pointer") }
    }
}

impl<T> DerefMut for Handle<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            let mutated = self.mutated.as_mut().expect("bad pointer");
            *mutated = true;
            self.value.as_mut().expect("bad pointer")
        }
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        // increase count
        let count = unsafe { self.count.as_mut().expect("bad pointer") };
        *count += 1;

        Self {
            _marker: PhantomData::default(),
            valid: true,
            value: self.value,
            count: self.count,
            mutated: self.mutated,
        }
    }
}

impl<T> Drop for Handle<T> {
    fn drop(&mut self) {
        // decrease count
        if self.valid {
            let count = unsafe { self.count.as_mut().expect("bad pointer") };
            *count -= 1;
        }
    }
}

unsafe impl<T> Send for Handle<T> {}
unsafe impl<T> Sync for Handle<T> {}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handle({:p})", self.value)
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
