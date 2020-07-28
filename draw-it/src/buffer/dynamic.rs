// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// DynamicBuffer - buffer struct thats memory will change

use ash::vk;
use std::mem;
use std::sync::Arc;

use super::BufferAccess;
use super::BufferMemory;
use super::BufferUsage;
use crate::device::Device;
use crate::error::Result;

pub(crate) struct DynamicBuffer {
    memory: BufferMemory,
    usage: BufferUsage,
    access: BufferAccess,
    size: usize,
    device: Arc<Device>,
}

impl DynamicBuffer {
    pub(crate) fn new<T: Copy>(
        device: &Arc<Device>,
        usage: BufferUsage,
        capacity: usize,
    ) -> Result<Self> {
        let size = mem::size_of::<T>() * capacity;

        // on CPU accessible memory, so we can copy to it
        let access = BufferAccess::Cpu;
        let memory = BufferMemory::new(device, &[usage], access, size)?;

        Ok(Self {
            device: Arc::clone(device),
            memory,
            usage,
            access,
            size,
        })
    }

    pub(crate) fn update_data<T: Copy>(&mut self, data: &[T]) -> Result<()> {
        let size = mem::size_of::<T>() * data.len();

        if size <= self.size {
            self.memory.copy_from_data(data, size)?;
        } else {
            // reallocate memory if data is too big
            self.memory = BufferMemory::new(&self.device, &[self.usage], self.access, size)?;
            self.memory.copy_from_data(data, size)?;
            self.size = size;
        }

        Ok(())
    }

    pub(crate) fn size(&self) -> u32 {
        self.size as u32
    }

    pub(crate) fn handle(&self) -> vk::Buffer {
        self.memory.handle()
    }
}

impl PartialEq for DynamicBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.memory == other.memory
    }
}
