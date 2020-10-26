// Oliver Berzs
// https://github.com/oberzs/draw-it

// enums for possible buffer properties

use crate::vk;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum BufferUsage {
    Vertex,
    Index,
    Uniform,
    TransferSrc,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum MemoryAccess {
    Gpu,
    Cpu,
}

impl BufferUsage {
    pub(crate) const fn flag(&self) -> vk::BufferUsageFlags {
        match *self {
            Self::Vertex => vk::BUFFER_USAGE_VERTEX_BUFFER_BIT,
            Self::Index => vk::BUFFER_USAGE_INDEX_BUFFER_BIT,
            Self::Uniform => vk::BUFFER_USAGE_UNIFORM_BUFFER_BIT,
            Self::TransferSrc => vk::BUFFER_USAGE_TRANSFER_SRC_BIT,
        }
    }
}

impl MemoryAccess {
    pub(crate) const fn flag(&self) -> vk::MemoryPropertyFlags {
        match *self {
            Self::Gpu => vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
            Self::Cpu => {
                vk::MEMORY_PROPERTY_HOST_COHERENT_BIT | vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT
            }
        }
    }
}
