use ash::version::DeviceV1_0;
use ash::vk::Buffer as VkBuffer;
use ash::vk::DeviceMemory;
use ash::vk::MemoryPropertyFlags;
use std::mem;
use std::sync::Arc;
use std::sync::Weak;

use super::Buffer;
use super::BufferType;
use crate::error::ErrorKind;
use crate::error::Result;
use crate::instance::Device;
use crate::memory::alloc;
use crate::memory::copy;

pub(crate) struct DynamicBuffer {
    vk: VkBuffer,
    memory: DeviceMemory,
    size: u32,
    device: Weak<Device>,
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
            device: Arc::downgrade(device),
        })
    }

    pub(crate) fn update_data<T: Copy>(&self, data: &[T]) -> Result<()> {
        let size = mem::size_of::<T>() * data.len();
        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        copy::data_to_buffer(&device, data, self.memory, size)?;
        Ok(())
    }

    pub(crate) fn size(&self) -> u32 {
        self.size
    }
}

impl Drop for DynamicBuffer {
    fn drop(&mut self) {
        let device = self
            .device
            .upgrade()
            .ok_or(ErrorKind::DeviceDropped)
            .unwrap();
        unsafe {
            device.wait_for_idle().unwrap();
            device.logical().destroy_buffer(self.vk, None);
            device.logical().free_memory(self.memory, None);
        }
    }
}

impl Buffer for DynamicBuffer {
    fn vk_buffer(&self) -> VkBuffer {
        self.vk
    }
}
