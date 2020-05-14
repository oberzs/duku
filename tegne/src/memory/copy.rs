use ash::version::DeviceV1_0;
use ash::vk::Buffer;
use ash::vk::DeviceMemory;
use ash::vk::MemoryMapFlags;
use std::ffi::c_void;
use std::ptr;
use std::sync::Arc;

use crate::error::Result;
use crate::instance::Commands;
use crate::instance::Device;
use crate::utils::OrError;

pub(crate) fn data_to_buffer<T: Copy>(
    device: &Arc<Device>,
    src: &[T],
    dst: DeviceMemory,
    size: usize,
) {
    unsafe {
        let memory = device
            .logical()
            .map_memory(dst, 0, (size as u32).into(), MemoryMapFlags::empty())
            .or_error("cannot map memory");

        let src_ptr: *const [T] = src;
        ptr::copy_nonoverlapping(src_ptr as *const c_void, memory, size);

        device.logical().unmap_memory(dst);
    }
}

pub(crate) fn buffer_to_buffer(
    device: &Arc<Device>,
    src: Buffer,
    dst: Buffer,
    size: usize,
) -> Result<()> {
    let cmd = Commands::new(device);
    cmd.begin_one_time()?;
    cmd.copy_buffer(src, dst, size)?;
    device.submit_buffer(cmd.end()?)?;
    Ok(())
}
