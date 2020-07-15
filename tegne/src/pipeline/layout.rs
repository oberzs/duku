// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// ShaderLayout - struct that holds shader layout

use ash::vk;
use std::mem;
use std::sync::Arc;

use super::PushConstants;
use crate::buffer::DynamicBuffer;
use crate::device::Device;
use crate::error::Result;
use crate::image::ImageLayout;

pub(crate) struct ShaderLayout {
    handle: vk::PipelineLayout,
    world_layout: vk::DescriptorSetLayout,
    material_layout: vk::DescriptorSetLayout,
    image_layout: vk::DescriptorSetLayout,
    shadow_map_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    device: Arc<Device>,
}

impl ShaderLayout {
    pub(crate) fn new(device: &Arc<Device>) -> Result<Self> {
        profile_scope!("new");

        // world uniform layout
        let world_binding = [vk::DescriptorSetLayoutBinding::builder()
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT)
            .descriptor_count(1)
            .binding(0)
            .build()];
        let world_layout = device.create_descriptor_set_layout(&world_binding)?;

        let max_world_count = 100;
        let world_pool_size = vk::DescriptorPoolSize::builder()
            .descriptor_count(max_world_count)
            .ty(vk::DescriptorType::UNIFORM_BUFFER)
            .build();

        // material uniform layout
        let material_binding = [vk::DescriptorSetLayoutBinding::builder()
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT)
            .descriptor_count(1)
            .binding(0)
            .build()];
        let material_layout = device.create_descriptor_set_layout(&material_binding)?;

        let max_material_count = 100;
        let material_pool_size = vk::DescriptorPoolSize::builder()
            .descriptor_count(max_material_count)
            .ty(vk::DescriptorType::UNIFORM_BUFFER)
            .build();

        // image uniform layout
        let max_image_count = 100;
        let sampler_count = 2 * 2 * 2;
        let image_binding = vk::DescriptorSetLayoutBinding::builder()
            .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            .descriptor_count(max_image_count)
            .binding(0)
            .build();
        let sampler_binding = vk::DescriptorSetLayoutBinding::builder()
            .descriptor_type(vk::DescriptorType::SAMPLER)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            .descriptor_count(sampler_count)
            .binding(1)
            .build();
        let image_bindings = [image_binding, sampler_binding];
        let image_layout = device.create_descriptor_set_layout(&image_bindings)?;

        let image_pool_size = vk::DescriptorPoolSize::builder()
            .descriptor_count(1)
            .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .build();

        // shadow map layout
        let shadow_map_binding = [vk::DescriptorSetLayoutBinding::builder()
            .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            .descriptor_count(3)
            .binding(0)
            .build()];
        let shadow_map_layout = device.create_descriptor_set_layout(&shadow_map_binding)?;
        let shadow_map_pool_size = vk::DescriptorPoolSize::builder()
            .descriptor_count(10)
            .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .build();

        // descriptor pool
        let pool_sizes = [
            world_pool_size,
            image_pool_size,
            material_pool_size,
            shadow_map_pool_size,
        ];
        let descriptor_pool =
            device.create_descriptor_pool(&pool_sizes, 1 + max_world_count + max_material_count)?;

        // push constants
        let push_constant = vk::PushConstantRange::builder()
            .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT)
            .size(mem::size_of::<PushConstants>() as u32)
            .offset(0)
            .build();

        // pipeline layout
        let constant_ranges = [push_constant];
        let set_layouts = [
            world_layout,
            material_layout,
            image_layout,
            shadow_map_layout,
        ];
        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder()
            .push_constant_ranges(&constant_ranges)
            .set_layouts(&set_layouts);
        let handle = device.create_pipeline_layout(&pipeline_layout_info)?;

        Ok(Self {
            handle,
            world_layout,
            material_layout,
            image_layout,
            shadow_map_layout,
            descriptor_pool,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn world_set(&self, buffer: &DynamicBuffer) -> Result<vk::DescriptorSet> {
        let set = self
            .device
            .allocate_descriptor_set(self.world_layout, self.descriptor_pool)?;

        let buffer_info = vk::DescriptorBufferInfo::builder()
            .buffer(buffer.handle())
            .offset(0)
            .range(buffer.size().into())
            .build();

        let buffer_infos = [buffer_info];
        let write = [vk::WriteDescriptorSet::builder()
            .dst_set(set)
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&buffer_infos)
            .build()];

        self.device.update_descriptor_sets(&write);

        Ok(set)
    }

    pub(crate) fn material_set(&self, buffer: &DynamicBuffer) -> Result<vk::DescriptorSet> {
        let set = self
            .device
            .allocate_descriptor_set(self.material_layout, self.descriptor_pool)?;

        let buffer_info = vk::DescriptorBufferInfo::builder()
            .buffer(buffer.handle())
            .offset(0)
            .range(buffer.size().into())
            .build();

        let buffer_infos = [buffer_info];
        let write = [vk::WriteDescriptorSet::builder()
            .dst_set(set)
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&buffer_infos)
            .build()];

        self.device.update_descriptor_sets(&write);

        Ok(set)
    }

    pub(crate) fn image_set(&self) -> Result<vk::DescriptorSet> {
        self.device
            .allocate_descriptor_set(self.image_layout, self.descriptor_pool)
    }

    pub(crate) fn shadow_map_set(&self, views: [vk::ImageView; 3]) -> Result<vk::DescriptorSet> {
        let set = self
            .device
            .allocate_descriptor_set(self.shadow_map_layout, self.descriptor_pool)?;

        let image_infos = views
            .iter()
            .map(|v| {
                vk::DescriptorImageInfo::builder()
                    .image_layout(ImageLayout::ShaderColor.flag())
                    .image_view(*v)
                    .build()
            })
            .collect::<Vec<_>>();
        let image_write = [vk::WriteDescriptorSet::builder()
            .dst_set(set)
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
            .image_info(&image_infos)
            .build()];

        self.device.update_descriptor_sets(&image_write);

        Ok(set)
    }

    pub(crate) fn handle(&self) -> vk::PipelineLayout {
        self.handle
    }
}

impl Drop for ShaderLayout {
    fn drop(&mut self) {
        self.device.destroy_pipeline_layout(self.handle);
        self.device.destroy_descriptor_set_layout(self.world_layout);
        self.device
            .destroy_descriptor_set_layout(self.material_layout);
        self.device.destroy_descriptor_set_layout(self.image_layout);
        self.device
            .destroy_descriptor_set_layout(self.shadow_map_layout);
        self.device.destroy_descriptor_pool(self.descriptor_pool);
    }
}
