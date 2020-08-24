// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Vertex - struct representing a vertex for a mesh

use std::mem;

use crate::math::Vector2;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::vk;

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
        vk::VertexInputBindingDescription {
            binding: 0,
            stride: mem::size_of::<Self>() as u32,
            input_rate: vk::VERTEX_INPUT_RATE_VERTEX,
        }
    }

    pub(crate) fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 4] {
        let size2 = mem::size_of::<Vector2>() as u32;
        let size3 = mem::size_of::<Vector3>() as u32;

        [
            // position
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::FORMAT_R32G32B32_SFLOAT,
                offset: 0,
            },
            // normal
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::FORMAT_R32G32B32_SFLOAT,
                offset: size3,
            },
            // uv
            vk::VertexInputAttributeDescription {
                location: 2,
                binding: 0,
                format: vk::FORMAT_R32G32_SFLOAT,
                offset: size3 * 2,
            },
            // color
            vk::VertexInputAttributeDescription {
                location: 3,
                binding: 0,
                format: vk::FORMAT_R32G32B32A32_SFLOAT,
                offset: size3 * 2 + size2,
            },
        ]
    }
}
