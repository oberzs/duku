use ash::version::DeviceV1_0;
use ash::vk::Buffer;
use ash::vk::BufferCreateInfo;
use ash::vk::BufferUsageFlags;
use ash::vk::DeviceMemory;
use ash::vk::MemoryAllocateInfo;
use ash::vk::MemoryPropertyFlags;
use ash::vk::SharingMode;
use log::error;
use std::sync::Arc;

use crate::error::Result;
use crate::instance::Device;

pub(crate) fn buffer(
    device: &Arc<Device>,
    usage: BufferUsageFlags,
    properties: MemoryPropertyFlags,
    size: usize,
) -> Result<(Buffer, DeviceMemory)> {
    // create buffer
    let buffer_info = BufferCreateInfo::builder()
        .size((size as u32).into())
        .usage(usage)
        .sharing_mode(SharingMode::EXCLUSIVE);

    let buffer = unsafe { device.logical().create_buffer(&buffer_info, None)? };

    // alloc memory
    let mem_requirements = unsafe { device.logical().get_buffer_memory_requirements(buffer) };

    let mem_type = device
        .find_memory_type(mem_requirements.memory_type_bits, properties)
        .unwrap_or_else(|| {
            panic!(error!("device does not support buffer memory type"));
        });

    let alloc_info = MemoryAllocateInfo::builder()
        .allocation_size(mem_requirements.size)
        .memory_type_index(mem_type);

    let memory = unsafe { device.logical().allocate_memory(&alloc_info, None)? };

    // bind memory
    unsafe { device.logical().bind_buffer_memory(buffer, memory, 0)? };
    Ok((buffer, memory))
}
