// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// DynamicBuffer - buffer struct thats memory will change

use std::mem;
use std::rc::Rc;

use super::BufferAccess;
use super::BufferMemory;
use super::BufferUsage;
use crate::device::Device;
use crate::vk;

pub(crate) struct DynamicBuffer {
    memory: BufferMemory,
    usage: BufferUsage,
    access: BufferAccess,
    size: usize,
    device: Rc<Device>,
}

impl DynamicBuffer {
    pub(crate) fn new<T: Copy>(device: &Rc<Device>, usage: BufferUsage, capacity: usize) -> Self {
        let size = mem::size_of::<T>() * capacity;

        // on CPU accessible memory, so we can copy to it
        let access = BufferAccess::Cpu;
        let memory = BufferMemory::new(device, &[usage], access, size);

        Self {
            device: Rc::clone(device),
            memory,
            usage,
            access,
            size,
        }
    }

    pub(crate) fn update_data<T: Copy>(&mut self, data: &[T]) {
        let size = mem::size_of::<T>() * data.len();

        if size <= self.size {
            self.memory.copy_from_data(data, size);
        } else {
            // reallocate memory if data is too big
            self.memory = BufferMemory::new(&self.device, &[self.usage], self.access, size);
            self.memory.copy_from_data(data, size);
            self.size = size;
        }
    }

    pub(crate) const fn size(&self) -> usize {
        self.size
    }

    pub(crate) const fn handle(&self) -> vk::Buffer {
        self.memory.handle()
    }
}

impl PartialEq for DynamicBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.memory == other.memory
    }
}
