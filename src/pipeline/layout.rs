// Oliver Berzs
// https://github.com/oberzs/draw-it

// ShaderLayout - struct that holds shader layout

use std::mem;
use std::ptr;

use super::ShaderConstants;
use super::ShaderMaterial;
use super::ShaderWorld;
use crate::buffer::Buffer;
use crate::device::Device;
use crate::image::ImageLayout;
use crate::vk;

pub(crate) struct ShaderLayout {
    handle: vk::PipelineLayout,
    world_layout: vk::DescriptorSetLayout,
    material_layout: vk::DescriptorSetLayout,
    image_layout: vk::DescriptorSetLayout,
    shadow_map_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Descriptor(pub(crate) u32, pub(crate) vk::DescriptorSet);

impl ShaderLayout {
    pub(crate) fn new(device: &Device) -> Self {
        // world uniform layout
        let world_binding = [vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::SHADER_STAGE_VERTEX_BIT | vk::SHADER_STAGE_FRAGMENT_BIT,
            p_immutable_samplers: ptr::null(),
        }];
        let world_layout = device.create_descriptor_set_layout(&world_binding);

        let max_world_count = 100;
        let world_pool_size = vk::DescriptorPoolSize {
            vk_type: vk::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            descriptor_count: max_world_count,
        };

        // material uniform layout
        let material_binding = [vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::SHADER_STAGE_VERTEX_BIT | vk::SHADER_STAGE_FRAGMENT_BIT,
            p_immutable_samplers: ptr::null(),
        }];
        let material_layout = device.create_descriptor_set_layout(&material_binding);

        let max_material_count = 100;
        let material_pool_size = vk::DescriptorPoolSize {
            vk_type: vk::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            descriptor_count: max_material_count,
        };

        // image uniform layout
        let max_image_count = 100;
        let max_cubemap_count = 100;
        let sampler_count = 2 * 3 * 2;
        let image_binding = vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DESCRIPTOR_TYPE_SAMPLED_IMAGE,
            descriptor_count: max_image_count,
            stage_flags: vk::SHADER_STAGE_FRAGMENT_BIT,
            p_immutable_samplers: ptr::null(),
        };
        let sampler_binding = vk::DescriptorSetLayoutBinding {
            binding: 1,
            descriptor_type: vk::DESCRIPTOR_TYPE_SAMPLER,
            descriptor_count: sampler_count,
            stage_flags: vk::SHADER_STAGE_FRAGMENT_BIT,
            p_immutable_samplers: ptr::null(),
        };
        let cubemap_binding = vk::DescriptorSetLayoutBinding {
            binding: 2,
            descriptor_type: vk::DESCRIPTOR_TYPE_SAMPLED_IMAGE,
            descriptor_count: max_cubemap_count,
            stage_flags: vk::SHADER_STAGE_FRAGMENT_BIT,
            p_immutable_samplers: ptr::null(),
        };
        let image_bindings = [image_binding, sampler_binding, cubemap_binding];
        let image_layout = device.create_descriptor_set_layout(&image_bindings);

        let image_pool_size = vk::DescriptorPoolSize {
            vk_type: vk::DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
            descriptor_count: 1,
        };

        // shadow map layout
        let shadow_map_binding = [vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DESCRIPTOR_TYPE_SAMPLED_IMAGE,
            descriptor_count: 4,
            stage_flags: vk::SHADER_STAGE_FRAGMENT_BIT,
            p_immutable_samplers: ptr::null(),
        }];
        let shadow_map_layout = device.create_descriptor_set_layout(&shadow_map_binding);
        let shadow_map_pool_size = vk::DescriptorPoolSize {
            vk_type: vk::DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
            descriptor_count: 10,
        };

        // descriptor pool
        let pool_sizes = [
            world_pool_size,
            image_pool_size,
            material_pool_size,
            shadow_map_pool_size,
        ];
        let descriptor_pool =
            device.create_descriptor_pool(&pool_sizes, 1 + max_world_count + max_material_count);

        // push constants
        let push_constant = vk::PushConstantRange {
            stage_flags: vk::SHADER_STAGE_VERTEX_BIT | vk::SHADER_STAGE_FRAGMENT_BIT,
            offset: 0,
            size: mem::size_of::<ShaderConstants>() as u32,
        };

        // pipeline layout
        let constant_ranges = [push_constant];
        let set_layouts = [
            world_layout,
            material_layout,
            image_layout,
            shadow_map_layout,
        ];
        let pipeline_layout_info = vk::PipelineLayoutCreateInfo {
            s_type: vk::STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            set_layout_count: set_layouts.len() as u32,
            p_set_layouts: set_layouts.as_ptr(),
            push_constant_range_count: 1,
            p_push_constant_ranges: constant_ranges.as_ptr(),
        };
        let handle = device.create_pipeline_layout(&pipeline_layout_info);

        Self {
            handle,
            world_layout,
            material_layout,
            image_layout,
            shadow_map_layout,
            descriptor_pool,
        }
    }

    pub(crate) fn world_set(&self, device: &Device, buffer: &Buffer<ShaderWorld>) -> Descriptor {
        let set = device.allocate_descriptor_set(self.world_layout, self.descriptor_pool);

        let buffer_info = [vk::DescriptorBufferInfo {
            buffer: buffer.handle(),
            offset: 0,
            range: buffer.size() as u64,
        }];
        let write = [vk::WriteDescriptorSet {
            s_type: vk::STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            p_next: ptr::null(),
            dst_set: set,
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_count: 1,
            descriptor_type: vk::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            p_image_info: ptr::null(),
            p_buffer_info: buffer_info.as_ptr(),
            p_texel_buffer_view: ptr::null(),
        }];

        device.update_descriptor_sets(&write);

        Descriptor(0, set)
    }

    pub(crate) fn material_set(
        &self,
        device: &Device,
        buffer: &Buffer<ShaderMaterial>,
    ) -> Descriptor {
        let set = device.allocate_descriptor_set(self.material_layout, self.descriptor_pool);

        let buffer_info = [vk::DescriptorBufferInfo {
            buffer: buffer.handle(),
            offset: 0,
            range: buffer.size() as u64,
        }];
        let write = [vk::WriteDescriptorSet {
            s_type: vk::STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            p_next: ptr::null(),
            dst_set: set,
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_count: 1,
            descriptor_type: vk::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            p_image_info: ptr::null(),
            p_buffer_info: buffer_info.as_ptr(),
            p_texel_buffer_view: ptr::null(),
        }];

        device.update_descriptor_sets(&write);

        Descriptor(1, set)
    }

    pub(crate) fn image_set(&self, device: &Device) -> Descriptor {
        let set = device.allocate_descriptor_set(self.image_layout, self.descriptor_pool);
        Descriptor(2, set)
    }

    pub(crate) fn shadow_map_set(&self, device: &Device, views: [vk::ImageView; 4]) -> Descriptor {
        let set = device.allocate_descriptor_set(self.shadow_map_layout, self.descriptor_pool);

        let image_infos: Vec<_> = views
            .iter()
            .map(|v| vk::DescriptorImageInfo {
                sampler: 0,
                image_view: *v,
                image_layout: ImageLayout::ShaderDepth.flag(),
            })
            .collect();
        let image_write = [vk::WriteDescriptorSet {
            s_type: vk::STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            p_next: ptr::null(),
            dst_set: set,
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_count: image_infos.len() as u32,
            descriptor_type: vk::DESCRIPTOR_TYPE_SAMPLED_IMAGE,
            p_image_info: image_infos.as_ptr(),
            p_buffer_info: ptr::null(),
            p_texel_buffer_view: ptr::null(),
        }];

        device.update_descriptor_sets(&image_write);

        Descriptor(3, set)
    }

    pub(crate) fn destroy(&self, device: &Device) {
        device.destroy_pipeline_layout(self.handle);
        device.destroy_descriptor_set_layout(self.world_layout);
        device.destroy_descriptor_set_layout(self.material_layout);
        device.destroy_descriptor_set_layout(self.image_layout);
        device.destroy_descriptor_set_layout(self.shadow_map_layout);
        device.destroy_descriptor_pool(self.descriptor_pool);
    }

    pub(crate) const fn handle(&self) -> vk::PipelineLayout {
        self.handle
    }
}
