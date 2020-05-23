use ash::vk::Buffer as VkBuffer;
use ash::vk::MemoryPropertyFlags;
use std::cell::Cell;
use std::cell::RefCell;
use std::mem;
use std::sync::Arc;

use super::Buffer;
use super::BufferType;
use crate::error::Result;
use crate::instance::Device;

pub(crate) struct DynamicBuffer {
    buffer: RefCell<Buffer>,
    buffer_type: BufferType,
    size: Cell<usize>,
    device: Arc<Device>,
}

impl DynamicBuffer {
    pub(crate) fn new<T: Copy>(
        device: &Arc<Device>,
        len: usize,
        buffer_type: BufferType,
    ) -> Result<Self> {
        let size = mem::size_of::<T>() * len;

        let buffer = Buffer::new(
            device,
            buffer_type.flag(),
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            size,
        )?;

        Ok(Self {
            buffer: RefCell::new(buffer),
            buffer_type,
            size: Cell::new(size),
            device: Arc::clone(device),
        })
    }

    pub(crate) fn update_data<T: Copy>(&self, data: &[T]) -> Result<()> {
        let size = mem::size_of::<T>() * data.len();
        self.buffer.borrow().copy_from_data(data, size)?;
        Ok(())
    }

    pub(crate) fn size(&self) -> u32 {
        self.size.get() as u32
    }

    pub(crate) fn vk(&self) -> VkBuffer {
        self.buffer.borrow().vk()
    }
}
