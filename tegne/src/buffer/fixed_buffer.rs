use ash::version::DeviceV1_0;
use ash::vk::Buffer as VkBuffer;
use ash::vk::BufferUsageFlags;
use ash::vk::DeviceMemory;
use ash::vk::MemoryPropertyFlags;
use std::mem;
use std::rc::Rc;

use super::Buffer;
use super::BufferType;
use crate::memory::alloc;
use crate::memory::copy;
use crate::tegne::Device;

pub struct FixedBuffer {
    buffer: VkBuffer,
    memory: DeviceMemory,
    size: u32,
    device: Rc<Device>,
}

impl FixedBuffer {
    pub fn new<T: Copy>(device: &Rc<Device>, data: &[T], buffer_type: BufferType) -> Self {
        let size = mem::size_of::<T>() * data.len();

        let (staging_buffer, staging_memory) = alloc::buffer(
            device,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            size,
        );

        copy::data_to_buffer(device, data, staging_memory, size);

        let (buffer, memory) = alloc::buffer(
            device,
            BufferUsageFlags::TRANSFER_DST | buffer_type.into(),
            MemoryPropertyFlags::DEVICE_LOCAL,
            size,
        );

        copy::buffer_to_buffer(device, staging_buffer, buffer, size);

        unsafe {
            device.logical().destroy_buffer(staging_buffer, None);
            device.logical().free_memory(staging_memory, None);
        }

        Self {
            buffer,
            memory,
            size: size as u32,
            device: Rc::clone(device),
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

impl Buffer for FixedBuffer {
    fn buffer(&self) -> VkBuffer {
        self.buffer
    }
}

impl Drop for FixedBuffer {
    fn drop(&mut self) {
        unsafe {
            self.device.logical().destroy_buffer(self.buffer, None);
            self.device.logical().free_memory(self.memory, None);
        }
    }
}
