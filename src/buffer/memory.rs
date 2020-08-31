// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// BufferMemory - struct that manages allocated buffer memory

use std::ffi::c_void;
use std::ptr;
use std::rc::Rc;

use super::BufferAccess;
use super::BufferUsage;
use crate::device::Device;
use crate::vk;

pub(crate) struct BufferMemory {
    handle: vk::Buffer,
    memory: vk::DeviceMemory,
    device: Rc<Device>,
}

impl BufferMemory {
    pub(crate) fn new(
        device: &Rc<Device>,
        usage: &[BufferUsage],
        access: BufferAccess,
        size: usize,
    ) -> Self {
        // create buffer
        let info = vk::BufferCreateInfo {
            s_type: vk::STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            size: size as u64,
            usage: BufferUsage::combine(usage),
            sharing_mode: vk::SHARING_MODE_EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
        };

        let (handle, memory) = device.allocate_buffer(&info, access);

        Self {
            handle,
            memory,
            device: Rc::clone(device),
        }
    }

    pub(crate) fn copy_from_data<T: Copy>(&self, data: &[T], size: usize) {
        self.device.map_memory(self.memory, size, |mem| unsafe {
            ptr::copy_nonoverlapping(data as *const [T] as *const c_void, mem, size);
        });
    }

    pub(crate) fn copy_from_memory(&self, memory: &Self, size: usize) {
        self.device
            .do_commands(|cmd| cmd.copy_buffer(memory.handle(), self.handle, size));
    }

    pub(crate) const fn handle(&self) -> vk::Buffer {
        self.handle
    }
}

impl Drop for BufferMemory {
    fn drop(&mut self) {
        self.device.free_buffer(self.handle, self.memory);
    }
}

impl PartialEq for BufferMemory {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
