// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// DynamicBuffer - buffer struct thats memory will change

use ash::vk;
use std::cell::Cell;
use std::cell::RefCell;
use std::mem;
use std::sync::Arc;

use super::BufferAccess;
use super::BufferMemory;
use super::BufferUsage;
use crate::device::Device;
use crate::error::Result;

pub(crate) struct DynamicBuffer {
    memory: RefCell<BufferMemory>,
    usage: BufferUsage,
    access: BufferAccess,
    size: Cell<usize>,
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
            memory: RefCell::new(memory),
            usage,
            access,
            size: Cell::new(size),
            device: Arc::clone(device),
        })
    }

    pub(crate) fn update_data<T: Copy>(&self, data: &[T]) -> Result<()> {
        let size = mem::size_of::<T>() * data.len();

        if size <= self.size.get() {
            self.memory.borrow().copy_from_data(data, size)?;
        } else {
            // reallocate memory if data is too big
            let mut memory = self.memory.borrow_mut();
            *memory = BufferMemory::new(&self.device, &[self.usage], self.access, size)?;
            memory.copy_from_data(data, size)?;
            self.size.set(size);
        }

        Ok(())
    }

    pub(crate) fn size(&self) -> u32 {
        self.size.get() as u32
    }

    pub(crate) fn handle(&self) -> vk::Buffer {
        self.memory.borrow().handle()
    }
}

impl PartialEq for DynamicBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.memory == other.memory
    }
}
