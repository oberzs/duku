use ash::version::DeviceV1_0;
use ash::vk::Buffer as VkBuffer;
use ash::vk::DeviceMemory;
use ash::vk::MemoryPropertyFlags;
use std::mem;
use std::rc::Rc;
use std::rc::Weak;

use super::Buffer;
use super::BufferType;
use crate::instance::Device;
use crate::memory::alloc;
use crate::memory::copy;
use crate::utils::OrError;

pub(crate) struct DynamicBuffer {
    vk: VkBuffer,
    memory: DeviceMemory,
    size: u32,
    device: Weak<Device>,
}

impl DynamicBuffer {
    pub(crate) fn new<T: Copy>(device: &Rc<Device>, len: usize, buffer_type: BufferType) -> Self {
        let size = mem::size_of::<T>() * len;

        let (vk, memory) = alloc::buffer(
            device,
            buffer_type.into(),
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            size,
        );

        Self {
            vk,
            memory,
            size: size as u32,
            device: Rc::downgrade(device),
        }
    }

    pub(crate) fn update_data<T: Copy>(&self, data: &[T]) {
        let size = mem::size_of::<T>() * data.len();
        copy::data_to_buffer(&self.device(), data, self.memory, size);
    }

    pub(crate) fn size(&self) -> u32 {
        self.size
    }

    fn device(&self) -> Rc<Device> {
        self.device.upgrade().or_error("device has been dropped")
    }
}

impl Buffer for DynamicBuffer {
    fn vk_buffer(&self) -> VkBuffer {
        self.vk
    }
}

impl Drop for DynamicBuffer {
    fn drop(&mut self) {
        unsafe {
            self.device().wait_for_idle();
            self.device().logical().destroy_buffer(self.vk, None);
            self.device().logical().free_memory(self.memory, None);
        }
    }
}
