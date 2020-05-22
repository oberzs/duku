use ash::vk::Buffer as VkBuffer;
use ash::vk::MemoryPropertyFlags;
use std::mem;
use std::sync::Arc;

use super::Buffer;
use super::BufferType;
use crate::error::Result;
use crate::instance::Device;

pub(crate) struct DynamicBuffer {
    buffer: Buffer,
    size: u32,
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
            buffer_type.into(),
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            size,
        )?;

        Ok(Self {
            buffer,
            size: size as u32,
        })
    }

    pub(crate) fn update_data<T: Copy>(&self, data: &[T]) -> Result<()> {
        let size = mem::size_of::<T>() * data.len();
        self.buffer.copy_from_data(data, size)?;
        Ok(())
    }

    pub(crate) fn size(&self) -> u32 {
        self.size
    }

    pub(crate) fn vk(&self) -> VkBuffer {
        self.buffer.vk()
    }
}
