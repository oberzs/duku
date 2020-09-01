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
    size: usize,
    marker: PhantomData<*const T>,
    device: Rc<Device>,
}

impl<T: Copy> DynamicBuffer<T> {
    pub(crate) fn new(device: &Rc<Device>, usage: BufferUsage, size: usize) -> Self {
        let real_size = mem::size_of::<T>() * size;

        // on CPU accessible memory, so we can copy to it
        let access = BufferAccess::Cpu;
        let memory = BufferMemory::new(device, &[usage], access, real_size);

        Self {
            device: Rc::clone(device),
            marker: PhantomData,
            memory,
            usage,
            access,
            size,
        }
    }

    pub(crate) fn update_data(&self, data: &[T]) {
        let real_size = mem::size_of::<T>() * data.len();
        debug_assert!(
            self.size >= data.len(),
            "dynamic buffer needs to be resized before"
        );
        self.memory.copy_from_data(data, real_size);
    }

    pub(crate) fn resize(&mut self, size: usize) {
        let real_size = mem::size_of::<T>() * size;
        self.memory = BufferMemory::new(&self.device, &[self.usage], self.access, real_size);
        self.size = size;
    }

    pub(crate) fn size(&self) -> usize {
        self.size
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
