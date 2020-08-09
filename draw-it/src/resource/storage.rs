// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Ref - resource reference

use std::any;
use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Weak;

pub struct Storage<T> {
    value: Arc<Mutex<T>>,
}

pub struct Ref<T> {
    value: Weak<Mutex<T>>,
}

impl<T> Storage<T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            value: Arc::new(Mutex::new(value)),
        }
    }

    pub(crate) fn as_ref(&self) -> Ref<T> {
        Ref {
            value: Arc::downgrade(&self.value),
        }
    }

    pub(crate) fn count(&self) -> usize {
        Arc::weak_count(&self.value)
    }

    pub(crate) fn with<R>(&self, func: impl FnOnce(&mut T) -> R) -> R {
        func(&mut self.value.lock().expect("storage poisoned"))
    }
}

impl<T> Ref<T> {
    pub fn with<R>(&self, func: impl FnOnce(&mut T) -> R) -> R {
        let arc = self.value.upgrade().expect("did device die?");
        let mut inner = arc.lock().expect("ref poisoned");
        func(&mut inner)
    }
}

impl<T> fmt::Debug for Ref<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_name = any::type_name::<T>().split("::").last().expect("bad type");
        write!(f, "Ref<{}> {:p}", type_name, self.value.as_ptr())
    }
}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Self {
            value: Weak::clone(&self.value),
        }
    }
}

impl<T: PartialEq> PartialEq for Ref<T> {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.value, &other.value) || {
            let arc = self.value.upgrade().expect("did device die?");
            let inner = arc.lock().expect("ref poisoned");
            let other_arc = other.value.upgrade().expect("did device die?");
            let other_inner = other_arc.lock().expect("ref poisoned");
            inner.eq(&other_inner)
        }
    }
}
