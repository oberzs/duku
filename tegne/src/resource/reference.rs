// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Ref - resource reference

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

#[derive(Clone)]
pub struct Ref<T> {
    value: Arc<Mutex<T>>,
}

impl<T> Ref<T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            value: Arc::new(Mutex::new(value)),
        }
    }

    pub fn with<R>(&self, func: impl FnOnce(MutexGuard<'_, T>) -> R) -> R {
        let guard = self.value.lock().expect("bad lock");
        func(guard)
    }

    pub(crate) fn count(&self) -> usize {
        Arc::strong_count(&self.value)
    }
}
