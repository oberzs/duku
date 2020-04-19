use ash::vk::Buffer as VkBuffer;
use ash::vk::BufferUsageFlags;

pub enum BufferType {
    Vertex,
    Index,
    Uniform,
}

pub trait Buffer {
    fn buffer(&self) -> VkBuffer;
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