// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// FixedBuffer - buffer struct thats memory will not change

#![allow(dead_code)] // TODO: remove when used

use std::mem;
use std::rc::Rc;

use super::BufferAccess;
use super::BufferMemory;
use super::BufferUsage;
use crate::device::Device;

pub(crate) struct FixedBuffer {
    memory: BufferMemory,
}

impl FixedBuffer {
    pub(crate) fn new<T: Copy>(device: &Rc<Device>, usage: BufferUsage, data: &[T]) -> Self {
        let size = mem::size_of::<T>() * data.len();

        // on CPU accessible memory, so we can copy to it
        let staging_memory =
            BufferMemory::new(device, &[BufferUsage::TransferSrc], BufferAccess::Cpu, size);
        staging_memory.copy_from_data(data, size);

        // on GPU accessible memory, so it reads fast
        let memory = BufferMemory::new(
            device,
            &[BufferUsage::TransferDst, usage],
            BufferAccess::Gpu,
            size,
        );
        memory.copy_from_memory(&staging_memory, size);

        Self { memory }
    }
}
