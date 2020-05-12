use ash::version::DeviceV1_0;
use ash::vk::Buffer;
use ash::vk::DeviceMemory;
use ash::vk::MemoryMapFlags;
use std::ffi::c_void;
use std::ptr;
use std::rc::Rc;

use crate::instance::CommandRecorder;
use crate::instance::Device;
use crate::utils::OrError;

pub(crate) fn data_to_buffer<T: Copy>(
    device: &Rc<Device>,
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

pub(crate) fn buffer_to_buffer(device: &Rc<Device>, src: Buffer, dst: Buffer, size: usize) {
    let recorder = CommandRecorder::new(device);
    recorder.begin_one_time();
    recorder.copy_buffer(src, dst, size);
    device.submit_buffer(recorder.end());
}
