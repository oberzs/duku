// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// BufferMemory - struct that manages allocated buffer memory

use ash::version::DeviceV1_0;
use ash::vk;
use log::error;
use std::ffi::c_void;
use std::ptr;
use std::sync::Arc;

use super::BufferAccess;
use super::BufferUsage;
use crate::device::Commands;
use crate::device::Device;
use crate::error::Result;

pub(crate) struct BufferMemory {
    handle: vk::Buffer,
    memory: vk::DeviceMemory,
    device: Arc<Device>,
}

impl BufferMemory {
    pub(crate) fn new(
        device: &Arc<Device>,
        usage: &[BufferUsage],
        access: BufferAccess,
        size: usize,
    ) -> Result<Self> {
        // create buffer
        let buffer_info = vk::BufferCreateInfo::builder()
            .size((size as u32).into())
            .usage(BufferUsage::combine(usage))
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let handle = unsafe { device.logical().create_buffer(&buffer_info, None)? };

        // alloc memory
        let mem_requirements = unsafe { device.logical().get_buffer_memory_requirements(handle) };

        let mem_type = device
            .find_memory_type(mem_requirements.memory_type_bits, access.flag())
            .unwrap_or_else(|| {
                panic!(error!("device does not support buffer memory type"));
            });

        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(mem_requirements.size)
            .memory_type_index(mem_type);

        let memory = unsafe { device.logical().allocate_memory(&alloc_info, None)? };

        // bind memory
        unsafe { device.logical().bind_buffer_memory(handle, memory, 0)? };

        Ok(Self {
            handle,
            memory,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn copy_from_data<T: Copy>(&self, data: &[T], size: usize) -> Result<()> {
        unsafe {
            let memory = self.device.logical().map_memory(
                self.memory,
                0,
                (size as u32).into(),
                vk::MemoryMapFlags::empty(),
            )?;
            ptr::copy_nonoverlapping(data as *const [T] as *const c_void, memory, size);
            self.device.logical().unmap_memory(self.memory);
        }
        Ok(())
    }

    pub(crate) fn copy_from_memory(&self, memory: &Self, size: usize) -> Result<()> {
        let cmd = Commands::new(&self.device)?;
        cmd.begin()?;
        cmd._copy_buffer(memory.handle(), self.handle, size);
        self.device.submit_and_wait(cmd.end()?)?;
        Ok(())
    }

    pub(crate) fn handle(&self) -> vk::Buffer {
        self.handle
    }
}

impl Drop for BufferMemory {
    fn drop(&mut self) {
        unsafe {
            self.device.wait_for_idle().unwrap();
            self.device.logical().destroy_buffer(self.handle, None);
            self.device.logical().free_memory(self.memory, None);
        }
    }
}
