// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Ref - resource reference

use std::sync::Arc;
use std::sync::Mutex;

pub struct Ref<T> {
    value: Arc<Mutex<T>>,
}

impl<T> Ref<T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            value: Arc::new(Mutex::new(value)),
        }
    }

    pub fn with<R>(&self, func: impl FnOnce(&mut T) -> R) -> R {
        let mut guard = self.value.lock().unwrap();
        func(&mut guard)
    }

    pub(crate) fn count(&self) -> usize {
        Arc::strong_count(&self.value)
    }
}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
        }
    }
}

impl<T: PartialEq> PartialEq for Ref<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.value, &other.value)
            || self.value.lock().unwrap().eq(&other.value.lock().unwrap())
    }
}
