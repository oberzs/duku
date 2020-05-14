use ash::version::DeviceV1_0;
use ash::vk::Buffer;
use ash::vk::BufferCreateInfo;
use ash::vk::BufferUsageFlags;
use ash::vk::DeviceMemory;
use ash::vk::MemoryAllocateInfo;
use ash::vk::MemoryPropertyFlags;
use ash::vk::SharingMode;
use std::sync::Arc;

use crate::instance::Device;
use crate::utils::OrError;

pub(crate) fn buffer(
    device: &Arc<Device>,
    usage: BufferUsageFlags,
    properties: MemoryPropertyFlags,
    size: usize,
) -> (Buffer, DeviceMemory) {
    // create buffer
    let buffer_info = BufferCreateInfo::builder()
        .size((size as u32).into())
        .usage(usage)
        .sharing_mode(SharingMode::EXCLUSIVE);

    let buffer = unsafe {
        device
            .logical()
            .create_buffer(&buffer_info, None)
            .or_error("annot create buffer")
    };

    // alloc memory
    let mem_requirements = unsafe { device.logical().get_buffer_memory_requirements(buffer) };

    let mem_type = device.pick_memory_type(mem_requirements.memory_type_bits, properties);

    let alloc_info = MemoryAllocateInfo::builder()
        .allocation_size(mem_requirements.size)
        .memory_type_index(mem_type);

    let memory = unsafe {
        device
            .logical()
            .allocate_memory(&alloc_info, None)
            .or_error("cannot allocate buffer memory")
    };

    // bind memory
    unsafe {
        device
            .logical()
            .bind_buffer_memory(buffer, memory, 0)
            .or_error("cannot bind buffer memory")
    };
    (buffer, memory)
}
