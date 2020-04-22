use ash::vk::Buffer;
use ash::vk::DescriptorSet;
use ash::vk::Pipeline;

use crate::shaders::PushConstants;

struct Order {
    pipeline: Pipeline,
    material_descriptor: (u32, DescriptorSet),
    push_consts: PushConstants,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}
