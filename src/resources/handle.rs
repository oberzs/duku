// Oliver Berzs
// https://github.com/oberzs/duku

use std::fmt;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// A handle to a rendering resource.
///
/// Cloning this does not create a new resource
pub struct Handle<T> {
    value: Arc<RwLock<T>>,
    mutated: Arc<AtomicBool>,
}

/// RAII structure used to release the read access
pub type ReadGuard<'a, T> = RwLockReadGuard<'a, T>;

/// RAII structure used to release the write access
pub type WriteGuard<'a, T> = RwLockWriteGuard<'a, T>;

impl<T> Handle<T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            value: Arc::new(RwLock::new(value)),
            mutated: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Get guarded read access to the resource
    ///
    /// # Panics
    ///
    /// This function will panic if write access to
    /// the resource is being used.
    pub fn read(&self) -> ReadGuard<'_, T> {
        self.value.read().expect("poisoned handle")
    }

    /// Get guarded write access to the resource
    ///
    /// # Panics
    ///
    /// This function will panic if write access to
    /// the resource is being used.
    pub fn write(&self) -> WriteGuard<'_, T> {
        self.mutated.store(true, Ordering::Relaxed);
        self.value.write().expect("poisoned handle")
    }

    pub(crate) fn count(&self) -> usize {
        Arc::strong_count(&self.value)
    }

    pub(crate) fn mutated(&mut self) -> bool {
        if self.mutated.load(Ordering::Relaxed) {
            self.mutated.store(false, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    pub(crate) fn get_mut(&self) -> WriteGuard<'_, T> {
        self.value.write().expect("poisoned handle")
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            mutated: Arc::clone(&self.mutated),
        }
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handle({:p})", Arc::as_ptr(&self.value))
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::as_ptr(&self.value) == Arc::as_ptr(&other.value)
    }
}
