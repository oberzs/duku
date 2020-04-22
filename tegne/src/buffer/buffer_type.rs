use ash::vk::Buffer as VkBuffer;
use ash::vk::BufferUsageFlags;

pub(crate) enum BufferType {
    Vertex,
    Index,
    Uniform,
}

pub(crate) trait Buffer {
    fn vk_buffer(&self) -> VkBuffer;
}

impl Into<BufferUsageFlags> for BufferType {
    fn into(self) -> BufferUsageFlags {
        match self {
            Self::Vertex => BufferUsageFlags::VERTEX_BUFFER,
            Self::Index => BufferUsageFlags::INDEX_BUFFER,
            Self::Uniform => BufferUsageFlags::UNIFORM_BUFFER,
        }
    }
}
