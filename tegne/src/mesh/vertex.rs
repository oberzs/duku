// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Vertex - struct representing a vertex for a mesh

use ash::vk;
use std::mem;
use tegne_math::Vector2;
use tegne_math::Vector3;
use tegne_math::Vector4;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct Vertex {
    pub(crate) pos: Vector3,
    pub(crate) norm: Vector3,
    pub(crate) uv: Vector2,
    pub(crate) col: Vector4,
}

impl Vertex {
    pub(crate) fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(mem::size_of::<Self>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub(crate) fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 4] {
        let pos_size = mem::size_of::<Vector3>() as u32;
        let norm_size = mem::size_of::<Vector3>() as u32;
        let uv_size = mem::size_of::<Vector2>() as u32;

        let pos_desc = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(0)
            .build();

        let norm_desc = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(pos_size)
            .build();

        let uv_desc = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(2)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(pos_size + norm_size)
            .build();

        let col_desc = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(3)
            .format(vk::Format::R32G32B32A32_SFLOAT)
            .offset(pos_size + norm_size + uv_size)
            .build();

        [pos_desc, norm_desc, uv_desc, col_desc]
    }
}
