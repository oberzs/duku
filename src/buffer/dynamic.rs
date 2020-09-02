// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// DynamicBuffer - buffer struct thats memory will change

use std::marker::PhantomData;
use std::mem;
use std::rc::Rc;

use super::BufferAccess;
use super::BufferMemory;
use super::BufferUsage;
use crate::device::Device;
use crate::vk;

pub(crate) struct DynamicBuffer<T: Copy> {
    memory: BufferMemory,
    usage: BufferUsage,
    access: BufferAccess,
    marker: PhantomData<*const T>,
    len: usize,
    device: Rc<Device>,
}

impl<T: Copy> DynamicBuffer<T> {
    pub(crate) fn new(device: &Rc<Device>, usage: BufferUsage, len: usize) -> Self {
        let bytes = mem::size_of::<T>() * len;

        // on CPU accessible memory, so we can copy to it
        let access = BufferAccess::Cpu;
        let memory = BufferMemory::new(device, &[usage], access, bytes);

        Self {
            device: Rc::clone(device),
            marker: PhantomData,
            memory,
            usage,
            access,
            len,
        }
    }

    pub(crate) fn update_data(&self, data: &[T]) {
        let bytes = mem::size_of::<T>() * data.len();
        debug_assert!(
            self.len >= data.len(),
            "dynamic buffer needs to be resized before"
        );
        self.memory.copy_from_data(data, bytes);
    }

    pub(crate) fn resize(&mut self, len: usize) {
        let bytes = mem::size_of::<T>() * len;
        self.memory = BufferMemory::new(&self.device, &[self.usage], self.access, bytes);
        self.len = len;
    }

    pub(crate) fn bytes(&self) -> u64 {
        (self.len * mem::size_of::<T>()) as u64
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn handle(&self) -> vk::Buffer {
        self.memory.handle()
    }
}

impl<T: Copy> PartialEq for DynamicBuffer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.memory == other.memory
    }
}
