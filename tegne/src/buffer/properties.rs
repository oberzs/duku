// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// enums for possible buffer properties

use ash::vk;

#[derive(Debug, Copy, Clone)]
pub(crate) enum BufferUsage {
    Vertex,
    Index,
    Uniform,
    TransferSrc,
    TransferDst,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum BufferAccess {
    Gpu,
    Cpu,
}

impl BufferUsage {
    pub(crate) fn combine(usages: &[Self]) -> vk::BufferUsageFlags {
        usages
            .iter()
            .fold(vk::BufferUsageFlags::empty(), |acc, usage| {
                acc | usage.flag()
            })
    }

    pub(crate) fn flag(&self) -> vk::BufferUsageFlags {
        match *self {
            Self::Vertex => vk::BufferUsageFlags::VERTEX_BUFFER,
            Self::Index => vk::BufferUsageFlags::INDEX_BUFFER,
            Self::Uniform => vk::BufferUsageFlags::UNIFORM_BUFFER,
            Self::TransferSrc => vk::BufferUsageFlags::TRANSFER_SRC,
            Self::TransferDst => vk::BufferUsageFlags::TRANSFER_DST,
        }
    }
}

impl BufferAccess {
    pub(crate) fn flag(&self) -> vk::MemoryPropertyFlags {
        match *self {
            Self::Gpu => vk::MemoryPropertyFlags::DEVICE_LOCAL,
            Self::Cpu => {
                vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE
            }
        }
    }
}
