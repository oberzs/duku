use ash::vk::Format;
use ash::vk::VertexInputAttributeDescription;
use ash::vk::VertexInputBindingDescription;
use ash::vk::VertexInputRate;
use std::mem;
use tegne_math::Vector2;
use tegne_math::Vector3;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    pub pos: Vector3,
    pub norm: Vector3,
    pub uv: Vector2,
}

impl Vertex {
    pub fn binding_description() -> VertexInputBindingDescription {
        VertexInputBindingDescription::builder()
            .binding(0)
            .stride(mem::size_of::<Self>() as u32)
            .input_rate(VertexInputRate::VERTEX)
            .build()
    }

    pub fn attribute_descriptions() -> [VertexInputAttributeDescription; 3] {
        let pos_size = mem::size_of::<Vector3>() as u32;
        let norm_size = mem::size_of::<Vector3>() as u32;

        let pos_desc = VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(Format::R32G32B32_SFLOAT)
            .offset(0)
            .build();

        let norm_desc = VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(Format::R32G32B32_SFLOAT)
            .offset(pos_size)
            .build();

        let uv_desc = VertexInputAttributeDescription::builder()
            .binding(0)
            .location(2)
            .format(Format::R32G32_SFLOAT)
            .offset(pos_size + norm_size)
            .build();

        [pos_desc, norm_desc, uv_desc]
    }
}
