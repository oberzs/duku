// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Buffer - struct that manages allocated buffer memory

mod properties;

use std::ffi::c_void;
use std::marker::PhantomData;
use std::mem;
use std::ptr;

use crate::device::Device;
use crate::vk;
pub(crate) use properties::BufferAccess;
pub(crate) use properties::BufferUsage;

pub(crate) struct Buffer<T: Copy> {
    handle: vk::Buffer,
    memory: vk::DeviceMemory,
    usage: BufferUsage,
    size: usize,
    marker: PhantomData<*const T>,
}

impl<T: Copy> Buffer<T> {
    pub(crate) fn dynamic(device: &Device, usage: BufferUsage, len: usize) -> Self {
        let size = mem::size_of::<T>() * len;

        // create buffer
        let info = vk::BufferCreateInfo {
            s_type: vk::STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            size: size as u64,
            usage: usage.flag(),
            sharing_mode: vk::SHARING_MODE_EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
        };

        let (handle, memory) = device.allocate_buffer(&info, BufferAccess::Cpu);

        Self {
            marker: PhantomData,
            handle,
            memory,
            size,
            usage,
        }
    }

    pub(crate) fn staging(device: &Device, data: &[T]) -> Self {
        let size = mem::size_of::<T>() * data.len();
        let usage = BufferUsage::TransferSrc;

        // create buffer
        let info = vk::BufferCreateInfo {
            s_type: vk::STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            size: size as u64,
            usage: usage.flag(),
            sharing_mode: vk::SHARING_MODE_EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
        };

        let (handle, memory) = device.allocate_buffer(&info, BufferAccess::Cpu);

        let buffer = Self {
            marker: PhantomData,
            handle,
            memory,
            size,
            usage,
        };
        buffer.copy_from_data(device, data);
        buffer
    }

    pub(crate) fn resize(&mut self, device: &Device, len: usize) {
        debug_assert!(
            self.usage != BufferUsage::TransferSrc,
            "cannot resize staging buffer"
        );

        let size = mem::size_of::<T>() * len;

        // create buffer
        let info = vk::BufferCreateInfo {
            s_type: vk::STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            size: size as u64,
            usage: self.usage.flag(),
            sharing_mode: vk::SHARING_MODE_EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
        };

        self.destroy(device);
        let (handle, memory) = device.allocate_buffer(&info, BufferAccess::Cpu);
        self.handle = handle;
        self.memory = memory;
        self.size = size;
    }

    pub(crate) fn copy_from_data(&self, device: &Device, data: &[T]) {
        let size = mem::size_of::<T>() * data.len();

        debug_assert!(self.size >= size, "dynamic buffer needs to be resized");

        device.map_memory(self.memory, size, |mem| unsafe {
            ptr::copy_nonoverlapping(data as *const [T] as *const c_void, mem, size);
        });
    }

    pub(crate) fn handle(&self) -> vk::Buffer {
        self.handle
    }

    pub(crate) fn size(&self) -> usize {
        self.size
    }

    pub(crate) fn len(&self) -> usize {
        self.size / mem::size_of::<T>()
    }

    pub(crate) fn destroy(&self, device: &Device) {
        device.free_buffer(self.handle, self.memory);
    }
}

impl<T: Copy> PartialEq for Buffer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
