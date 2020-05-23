use ash::version::DeviceV1_0;
use ash::vk::Buffer as VkBuffer;
use ash::vk::BufferCreateInfo;
use ash::vk::BufferUsageFlags;
use ash::vk::DeviceMemory;
use ash::vk::MemoryAllocateInfo;
use ash::vk::MemoryMapFlags;
use ash::vk::MemoryPropertyFlags;
use ash::vk::SharingMode;
use log::error;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::sync::Arc;
use std::thread;

use crate::error::Result;
use crate::instance::Commands;
use crate::instance::Device;

pub(crate) struct Buffer {
    vk: VkBuffer,
    memory: DeviceMemory,
    device: Arc<Device>,
}

pub(crate) enum BufferType {
    Vertex,
    Index,
    Uniform,
}

impl Buffer {
    pub(crate) fn new(
        device: &Arc<Device>,
        usage: BufferUsageFlags,
        properties: MemoryPropertyFlags,
        size: usize,
    ) -> Result<Self> {
        // create buffer
        let buffer_info = BufferCreateInfo::builder()
            .size((size as u32).into())
            .usage(usage)
            .sharing_mode(SharingMode::EXCLUSIVE);

        let vk = unsafe { device.logical().create_buffer(&buffer_info, None)? };

        // alloc memory
        let mem_requirements = unsafe { device.logical().get_buffer_memory_requirements(vk) };

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
        unsafe { device.logical().bind_buffer_memory(vk, memory, 0)? };

        Ok(Self {
            vk,
            memory,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn _device_local<T: Copy>(
        device: &Arc<Device>,
        data: &[T],
        buffer_type: BufferType,
    ) -> Result<Self> {
        let size = mem::size_of::<T>() * data.len();

        let staging_buffer = Self::new(
            device,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            size,
        )?;

        staging_buffer.copy_from_data(data, size)?;

        let buffer = Self::new(
            device,
            BufferUsageFlags::TRANSFER_DST | buffer_type.flag(),
            MemoryPropertyFlags::DEVICE_LOCAL,
            size,
        )?;

        buffer._copy_from_buffer(&staging_buffer, size)?;

        Ok(buffer)
    }

    pub(crate) fn copy_from_data<T: Copy>(&self, data: &[T], size: usize) -> Result<()> {
        unsafe {
            let memory = self.device.logical().map_memory(
                self.memory,
                0,
                (size as u32).into(),
                MemoryMapFlags::empty(),
            )?;
            ptr::copy_nonoverlapping(data as *const [T] as *const c_void, memory, size);
            self.device.logical().unmap_memory(self.memory);
        }
        Ok(())
    }

    pub(crate) fn _copy_from_buffer(&self, buffer: &Buffer, size: usize) -> Result<()> {
        let cmd = Commands::new(&self.device)?;
        cmd.begin()?;
        cmd._copy_buffer(buffer.vk(), self.vk, size);
        self.device.submit_and_wait(cmd.end()?)?;
        Ok(())
    }

    pub(crate) fn vk(&self) -> VkBuffer {
        self.vk
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.device.wait_for_idle().unwrap();
            self.device.logical().destroy_buffer(self.vk, None);
            self.device.logical().free_memory(self.memory, None);
        }
    }
}

impl BufferType {
    pub(crate) fn flag(&self) -> BufferUsageFlags {
        match *self {
            Self::Vertex => BufferUsageFlags::VERTEX_BUFFER,
            Self::Index => BufferUsageFlags::INDEX_BUFFER,
            Self::Uniform => BufferUsageFlags::UNIFORM_BUFFER,
        }
    }
}
