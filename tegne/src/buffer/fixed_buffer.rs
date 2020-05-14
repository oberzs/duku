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
use crate::instance::Device;
use crate::memory::alloc;
use crate::memory::copy;
use crate::utils::OrError;

pub(crate) struct FixedBuffer {
    vk: VkBuffer,
    memory: DeviceMemory,
    device: Weak<Device>,
}

impl FixedBuffer {
    pub(crate) fn new<T: Copy>(device: &Arc<Device>, data: &[T], buffer_type: BufferType) -> Self {
        let size = mem::size_of::<T>() * data.len();

        let (staging_buffer, staging_memory) = alloc::buffer(
            device,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            size,
        );

        copy::data_to_buffer(device, data, staging_memory, size);

        let (vk, memory) = alloc::buffer(
            device,
            BufferUsageFlags::TRANSFER_DST | buffer_type.into(),
            MemoryPropertyFlags::DEVICE_LOCAL,
            size,
        );

        copy::buffer_to_buffer(device, staging_buffer, vk, size);

        unsafe {
            device.logical().destroy_buffer(staging_buffer, None);
            device.logical().free_memory(staging_memory, None);
        }

        Self {
            vk,
            memory,
            device: Arc::downgrade(device),
        }
    }

    fn device(&self) -> Arc<Device> {
        self.device.upgrade().or_error("device has been dropped")
    }
}

impl Buffer for FixedBuffer {
    fn vk_buffer(&self) -> VkBuffer {
        self.vk
    }
}

impl Drop for FixedBuffer {
    fn drop(&mut self) {
        unsafe {
            self.device().wait_for_idle();
            self.device().logical().destroy_buffer(self.vk, None);
            self.device().logical().free_memory(self.memory, None);
        }
    }
}
