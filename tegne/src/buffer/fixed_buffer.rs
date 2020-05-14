use ash::version::DeviceV1_0;
use ash::vk::Buffer as VkBuffer;
use ash::vk::BufferUsageFlags;
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

pub(crate) struct FixedBuffer {
    vk: VkBuffer,
    memory: DeviceMemory,
    device: Weak<Device>,
}

impl FixedBuffer {
    pub(crate) fn new<T: Copy>(
        device: &Arc<Device>,
        data: &[T],
        buffer_type: BufferType,
    ) -> Result<Self> {
        let size = mem::size_of::<T>() * data.len();

        let (staging_buffer, staging_memory) = alloc::buffer(
            device,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            size,
        )?;

        copy::data_to_buffer(device, data, staging_memory, size)?;

        let (vk, memory) = alloc::buffer(
            device,
            BufferUsageFlags::TRANSFER_DST | buffer_type.into(),
            MemoryPropertyFlags::DEVICE_LOCAL,
            size,
        )?;

        copy::buffer_to_buffer(device, staging_buffer, vk, size)?;

        unsafe {
            device.logical().destroy_buffer(staging_buffer, None);
            device.logical().free_memory(staging_memory, None);
        }

        Ok(Self {
            vk,
            memory,
            device: Arc::downgrade(device),
        })
    }
}

impl Buffer for FixedBuffer {
    fn vk_buffer(&self) -> VkBuffer {
        self.vk
    }
}

impl Drop for FixedBuffer {
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
