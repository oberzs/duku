use ash::version::DeviceV1_0;
use ash::vk::Buffer as VkBuffer;
use ash::vk::DeviceMemory;
use ash::vk::MemoryPropertyFlags;
use std::mem;
use std::sync::Arc;

use super::Buffer;
use super::BufferType;
use crate::error::Result;
use crate::instance::Device;
use crate::memory::alloc;
use crate::memory::copy;

pub(crate) struct DynamicBuffer {
    vk: VkBuffer,
    memory: DeviceMemory,
    size: u32,
    device: Arc<Device>,
}

impl DynamicBuffer {
    pub(crate) fn new<T: Copy>(
        device: &Arc<Device>,
        len: usize,
        buffer_type: BufferType,
    ) -> Result<Self> {
        let size = mem::size_of::<T>() * len;

        let (vk, memory) = alloc::buffer(
            device,
            buffer_type.into(),
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            size,
        )?;

        Ok(Self {
            vk,
            memory,
            size: size as u32,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn update_data<T: Copy>(&self, data: &[T]) -> Result<()> {
        let size = mem::size_of::<T>() * data.len();
        copy::data_to_buffer(&self.device, data, self.memory, size)?;
        Ok(())
    }

    pub(crate) fn size(&self) -> u32 {
        self.size
    }
}

impl Drop for DynamicBuffer {
    fn drop(&mut self) {
        unsafe {
            self.device.wait_for_idle().unwrap();
            self.device.logical().destroy_buffer(self.vk, None);
            self.device.logical().free_memory(self.memory, None);
        }
    }
}

impl Buffer for DynamicBuffer {
    fn vk_buffer(&self) -> VkBuffer {
        self.vk
    }
}
