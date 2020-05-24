// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// ShaderLayout - struct that holds shader layout

use ash::version::DeviceV1_0;
use ash::vk;
use log::debug;
use std::mem;
use std::sync::Arc;

use super::PushConstants;
use crate::buffer::DynamicBuffer;
use crate::error::Result;
use crate::instance::Device;

pub(crate) struct ShaderLayout {
    handle: vk::PipelineLayout,
    world_layout: vk::DescriptorSetLayout,
    material_layout: vk::DescriptorSetLayout,
    image_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    device: Arc<Device>,
}

impl ShaderLayout {
    pub(crate) fn new(device: &Arc<Device>) -> Result<Self> {
        debug!("creating shader layout");

        // world uniform layout
        let world_binding = [vk::DescriptorSetLayoutBinding::builder()
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT)
            .descriptor_count(1)
            .binding(0)
            .build()];

        let world_layout_info =
            vk::DescriptorSetLayoutCreateInfo::builder().bindings(&world_binding);

        let world_layout = unsafe {
            device
                .logical()
                .create_descriptor_set_layout(&world_layout_info, None)?
        };

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

        let material_layout_info =
            vk::DescriptorSetLayoutCreateInfo::builder().bindings(&material_binding);

        let material_layout = unsafe {
            device
                .logical()
                .create_descriptor_set_layout(&material_layout_info, None)?
        };

        let max_material_count = 100;
        let material_pool_size = vk::DescriptorPoolSize::builder()
            .descriptor_count(max_material_count)
            .ty(vk::DescriptorType::UNIFORM_BUFFER)
            .build();

        // image uniform layout
        let max_image_count = 100;
        let sampler_count = 3;
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
        let image_layout_info =
            vk::DescriptorSetLayoutCreateInfo::builder().bindings(&image_bindings);
        let image_layout = unsafe {
            device
                .logical()
                .create_descriptor_set_layout(&image_layout_info, None)?
        };

        let image_pool_size = vk::DescriptorPoolSize::builder()
            .descriptor_count(1)
            .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .build();

        // descriptor pool
        let pool_sizes = [world_pool_size, image_pool_size, material_pool_size];
        let descriptor_pool_info = vk::DescriptorPoolCreateInfo::builder()
            .pool_sizes(&pool_sizes)
            .max_sets(1 + max_world_count + max_material_count);

        let descriptor_pool = unsafe {
            device
                .logical()
                .create_descriptor_pool(&descriptor_pool_info, None)?
        };

        // push constants
        let push_constant = vk::PushConstantRange::builder()
            .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT)
            .size(mem::size_of::<PushConstants>() as u32)
            .offset(0)
            .build();

        // pipeline layout
        let constant_ranges = [push_constant];
        let set_layouts = [world_layout, material_layout, image_layout];
        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder()
            .push_constant_ranges(&constant_ranges)
            .set_layouts(&set_layouts);
        let handle = unsafe {
            device
                .logical()
                .create_pipeline_layout(&pipeline_layout_info, None)?
        };

        Ok(Self {
            handle,
            world_layout,
            material_layout,
            image_layout,
            descriptor_pool,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn world_set(&self, buffer: &DynamicBuffer) -> Result<vk::DescriptorSet> {
        let set_layouts = [self.world_layout];
        let set_alloc_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(&set_layouts);

        let set = unsafe {
            self.device
                .logical()
                .allocate_descriptor_sets(&set_alloc_info)?[0]
        };

        let buffer_info = vk::DescriptorBufferInfo::builder()
            .buffer(buffer.handle())
            .offset(0)
            .range(buffer.size().into())
            .build();

        let buffer_infos = [buffer_info];
        let descriptor_write = vk::WriteDescriptorSet::builder()
            .dst_set(set)
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&buffer_infos)
            .build();
        let descriptor_writes = [descriptor_write];

        unsafe {
            self.device
                .logical()
                .update_descriptor_sets(&descriptor_writes, &[]);
        }

        Ok(set)
    }

    pub(crate) fn material_set(&self, buffer: &DynamicBuffer) -> Result<vk::DescriptorSet> {
        let set_layouts = [self.material_layout];
        let set_alloc_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(&set_layouts);

        let set = unsafe {
            self.device
                .logical()
                .allocate_descriptor_sets(&set_alloc_info)?[0]
        };

        let buffer_info = vk::DescriptorBufferInfo::builder()
            .buffer(buffer.handle())
            .offset(0)
            .range(buffer.size().into())
            .build();

        let buffer_infos = [buffer_info];
        let descriptor_write = vk::WriteDescriptorSet::builder()
            .dst_set(set)
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&buffer_infos)
            .build();
        let descriptor_writes = [descriptor_write];

        unsafe {
            self.device
                .logical()
                .update_descriptor_sets(&descriptor_writes, &[]);
        }

        Ok(set)
    }

    pub(crate) fn image_set(&self) -> Result<vk::DescriptorSet> {
        let set_layouts = [self.image_layout];
        let set_alloc_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(&set_layouts)
            .build();

        let set = unsafe {
            self.device
                .logical()
                .allocate_descriptor_sets(&set_alloc_info)?[0]
        };
        Ok(set)
    }

    pub(crate) fn handle(&self) -> vk::PipelineLayout {
        self.handle
    }
}

impl Drop for ShaderLayout {
    fn drop(&mut self) {
        unsafe {
            self.device
                .logical()
                .destroy_pipeline_layout(self.handle, None);
            self.device
                .logical()
                .destroy_descriptor_set_layout(self.world_layout, None);
            self.device
                .logical()
                .destroy_descriptor_set_layout(self.material_layout, None);
            self.device
                .logical()
                .destroy_descriptor_set_layout(self.image_layout, None);
            self.device
                .logical()
                .destroy_descriptor_pool(self.descriptor_pool, None);
        }
    }
}
