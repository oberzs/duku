// Oliver Berzs
// https://github.com/oberzs/duku

// Vertex - struct representing a vertex for a mesh

use std::mem;

use crate::math::Vector2;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::vk;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub(crate) struct Vertex {
    pub(crate) in_local_position: Vector3,
    pub(crate) in_normal: Vector3,
    pub(crate) in_tangent: Vector3,
    pub(crate) in_uv: Vector2,
    pub(crate) in_color: Vector4,
    pub(crate) in_texture: u32,
}

impl Vertex {
    pub(crate) const fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription {
            binding: 0,
            stride: mem::size_of::<Self>() as u32,
            input_rate: vk::VERTEX_INPUT_RATE_VERTEX,
        }
    }

    pub(crate) const fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 6] {
        let mut offsets = [0; 6];
        offsets[0] = 0;
        offsets[1] = offsets[0] + mem::size_of::<Vector3>() as u32;
        offsets[2] = offsets[1] + mem::size_of::<Vector3>() as u32;
        offsets[3] = offsets[2] + mem::size_of::<Vector3>() as u32;
        offsets[4] = offsets[3] + mem::size_of::<Vector2>() as u32;
        offsets[5] = offsets[4] + mem::size_of::<Vector4>() as u32;

        [
            // in_local_position
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::FORMAT_R32G32B32_SFLOAT,
                offset: offsets[0],
            },
            // in_normal
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::FORMAT_R32G32B32_SFLOAT,
                offset: offsets[1],
            },
            // in_tangent
            vk::VertexInputAttributeDescription {
                location: 2,
                binding: 0,
                format: vk::FORMAT_R32G32B32_SFLOAT,
                offset: offsets[2],
            },
            // in_uv
            vk::VertexInputAttributeDescription {
                location: 3,
                binding: 0,
                format: vk::FORMAT_R32G32_SFLOAT,
                offset: offsets[3],
            },
            // in_color
            vk::VertexInputAttributeDescription {
                location: 4,
                binding: 0,
                format: vk::FORMAT_R32G32B32A32_SFLOAT,
                offset: offsets[4],
            },
            // in_texture
            vk::VertexInputAttributeDescription {
                location: 5,
                binding: 0,
                format: vk::FORMAT_R32_UINT,
                offset: offsets[5],
            },
        ]
    }
}
