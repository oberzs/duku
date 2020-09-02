// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Buffer - struct that manages allocated buffer memory

mod properties;

use std::cell::Cell;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::rc::Rc;

use crate::device::Device;
use crate::vk;
pub(crate) use properties::BufferAccess;
pub(crate) use properties::BufferUsage;

pub(crate) struct Buffer<T: Copy> {
    handle: Cell<vk::Buffer>,
    memory: Cell<vk::DeviceMemory>,
    usage: BufferUsage,
    size: Cell<usize>,
    marker: PhantomData<T>,
    device: Rc<Device>,
}

impl<T: Copy> Buffer<T> {
    pub(crate) fn dynamic(device: &Rc<Device>, usage: BufferUsage, len: usize) -> Self {
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
            device: Rc::clone(device),
            handle: Cell::new(handle),
            memory: Cell::new(memory),
            size: Cell::new(size),
            marker: PhantomData,
            usage,
        }
    }

    pub(crate) fn staging(device: &Rc<Device>, data: &[T]) -> Self {
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
            device: Rc::clone(device),
            handle: Cell::new(handle),
            memory: Cell::new(memory),
            size: Cell::new(size),
            marker: PhantomData,
            usage,
        };
        buffer.copy_from_data(data);
        buffer
    }

    pub(crate) fn resize(&self, len: usize) {
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

        self.free();
        let (handle, memory) = self.device.allocate_buffer(&info, BufferAccess::Cpu);
        self.handle.set(handle);
        self.memory.set(memory);
        self.size.set(size);
    }

    pub(crate) fn copy_from_data(&self, data: &[T]) {
        let size = mem::size_of::<T>() * data.len();

        debug_assert!(
            self.size.get() >= size,
            "dynamic buffer needs to be resized"
        );

        self.device
            .map_memory(self.memory.get(), size, |mem| unsafe {
                ptr::copy_nonoverlapping(data as *const [T] as *const c_void, mem, size);
            });
    }

    pub(crate) fn handle(&self) -> vk::Buffer {
        self.handle.get()
    }

    pub(crate) fn size(&self) -> usize {
        self.size.get()
    }

    pub(crate) fn len(&self) -> usize {
        self.size.get() / mem::size_of::<T>()
    }

    fn free(&self) {
        self.device
            .free_buffer(self.handle.get(), self.memory.get());
    }
}

impl<T: Copy> Drop for Buffer<T> {
    fn drop(&mut self) {
        self.free();
    }
}

impl<T: Copy> PartialEq for Buffer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
