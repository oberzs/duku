use ash::version::DeviceV1_0;
use ash::vk::DescriptorBufferInfo;
use ash::vk::DescriptorPool;
use ash::vk::DescriptorPoolCreateInfo;
use ash::vk::DescriptorPoolSize;
use ash::vk::DescriptorSet;
use ash::vk::DescriptorSetAllocateInfo;
use ash::vk::DescriptorSetLayout;
use ash::vk::DescriptorSetLayoutBinding;
use ash::vk::DescriptorSetLayoutCreateInfo;
use ash::vk::DescriptorType;
use ash::vk::PipelineLayout;
use ash::vk::PipelineLayoutCreateInfo;
use ash::vk::PushConstantRange;
use ash::vk::ShaderStageFlags;
use ash::vk::WriteDescriptorSet;
use log::debug;
use std::mem;
use std::rc::Rc;
use std::rc::Weak;

use super::PushConstants;
use crate::buffer::Buffer;
use crate::buffer::DynamicBuffer;
use crate::instance::Device;
use crate::utils::OrError;

pub(crate) struct ShaderLayout {
    pipeline_layout: PipelineLayout,
    world_layout: DescriptorSetLayout,
    material_layout: DescriptorSetLayout,
    image_layout: DescriptorSetLayout,
    descriptor_pool: DescriptorPool,
    device: Weak<Device>,
}

impl ShaderLayout {
    pub(crate) fn new(device: &Rc<Device>) -> Self {
        debug!("creating shader layout");

        // world layout
        let world_binding = DescriptorSetLayoutBinding::builder()
            .descriptor_type(DescriptorType::UNIFORM_BUFFER)
            .stage_flags(ShaderStageFlags::VERTEX | ShaderStageFlags::FRAGMENT)
            .descriptor_count(1)
            .binding(0)
            .build();

        let world_bindings = [world_binding];
        let world_layout_info = DescriptorSetLayoutCreateInfo::builder().bindings(&world_bindings);

        let world_layout = unsafe {
            device
                .logical()
                .create_descriptor_set_layout(&world_layout_info, None)
                .or_error("cannot create world descriptor set layout")
        };

        let max_world_count = 100;
        let world_pool_size = DescriptorPoolSize::builder()
            .descriptor_count(max_world_count)
            .ty(DescriptorType::UNIFORM_BUFFER)
            .build();

        // material layout
        let material_binding = DescriptorSetLayoutBinding::builder()
            .descriptor_type(DescriptorType::UNIFORM_BUFFER)
            .stage_flags(ShaderStageFlags::VERTEX | ShaderStageFlags::FRAGMENT)
            .descriptor_count(1)
            .binding(0)
            .build();

        let material_bindings = [material_binding];
        let material_layout_info =
            DescriptorSetLayoutCreateInfo::builder().bindings(&material_bindings);

        let material_layout = unsafe {
            device
                .logical()
                .create_descriptor_set_layout(&material_layout_info, None)
                .or_error("cannot create material descriptor set layout")
        };

        let max_material_count = 100;
        let material_pool_size = DescriptorPoolSize::builder()
            .descriptor_count(max_material_count)
            .ty(DescriptorType::UNIFORM_BUFFER)
            .build();

        // image layout
        let max_image_count = 100;
        let sampler_count = 2;
        let image_binding = DescriptorSetLayoutBinding::builder()
            .descriptor_type(DescriptorType::SAMPLED_IMAGE)
            .stage_flags(ShaderStageFlags::FRAGMENT)
            .descriptor_count(max_image_count)
            .binding(0)
            .build();
        let sampler_binding = DescriptorSetLayoutBinding::builder()
            .descriptor_type(DescriptorType::SAMPLER)
            .stage_flags(ShaderStageFlags::FRAGMENT)
            .descriptor_count(sampler_count)
            .binding(1)
            .build();

        let image_bindings = [image_binding, sampler_binding];
        let image_layout_info = DescriptorSetLayoutCreateInfo::builder().bindings(&image_bindings);
        let image_layout = unsafe {
            device
                .logical()
                .create_descriptor_set_layout(&image_layout_info, None)
                .or_error("cannot create image descriptor set layout")
        };

        let image_pool_size = DescriptorPoolSize::builder()
            .descriptor_count(1)
            .ty(DescriptorType::COMBINED_IMAGE_SAMPLER)
            .build();

        // descriptor pool
        let pool_sizes = [world_pool_size, image_pool_size, material_pool_size];
        let descriptor_pool_info = DescriptorPoolCreateInfo::builder()
            .pool_sizes(&pool_sizes)
            .max_sets(1 + max_world_count + max_material_count);

        let descriptor_pool = unsafe {
            device
                .logical()
                .create_descriptor_pool(&descriptor_pool_info, None)
                .or_error("cannot create descriptor pool")
        };

        // push constants
        let push_constant = PushConstantRange::builder()
            .stage_flags(ShaderStageFlags::VERTEX | ShaderStageFlags::FRAGMENT)
            .size(mem::size_of::<PushConstants>() as u32)
            .offset(0)
            .build();

        // pipeline layout
        let constant_ranges = [push_constant];
        let set_layouts = [world_layout, material_layout, image_layout];
        let pipeline_layout_info = PipelineLayoutCreateInfo::builder()
            .push_constant_ranges(&constant_ranges)
            .set_layouts(&set_layouts);
        let pipeline_layout = unsafe {
            device
                .logical()
                .create_pipeline_layout(&pipeline_layout_info, None)
                .or_error("cannot create pipeline layout")
        };

        Self {
            pipeline_layout,
            world_layout,
            material_layout,
            image_layout,
            descriptor_pool,
            device: Rc::downgrade(device),
        }
    }

    pub(crate) fn world_set(&self, buffer: &DynamicBuffer) -> DescriptorSet {
        let set_layouts = [self.world_layout];
        let set_alloc_info = DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(&set_layouts);

        let set = unsafe {
            self.device()
                .logical()
                .allocate_descriptor_sets(&set_alloc_info)
                .or_error("cannot allocate world descriptor set")[0]
        };

        let buffer_info = DescriptorBufferInfo::builder()
            .buffer(buffer.vk_buffer())
            .offset(0)
            .range(buffer.size().into())
            .build();

        let buffer_infos = [buffer_info];
        let descriptor_write = WriteDescriptorSet::builder()
            .dst_set(set)
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_type(DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&buffer_infos)
            .build();
        let descriptor_writes = [descriptor_write];

        unsafe {
            self.device()
                .logical()
                .update_descriptor_sets(&descriptor_writes, &[]);
        }

        set
    }

    pub(crate) fn material_set(&self, buffer: &DynamicBuffer) -> DescriptorSet {
        let set_layouts = [self.material_layout];
        let set_alloc_info = DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(&set_layouts);

        let set = unsafe {
            self.device()
                .logical()
                .allocate_descriptor_sets(&set_alloc_info)
                .or_error("cannot allocate material descriptor set")[0]
        };

        let buffer_info = DescriptorBufferInfo::builder()
            .buffer(buffer.vk_buffer())
            .offset(0)
            .range(buffer.size().into())
            .build();

        let buffer_infos = [buffer_info];
        let descriptor_write = WriteDescriptorSet::builder()
            .dst_set(set)
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_type(DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&buffer_infos)
            .build();
        let descriptor_writes = [descriptor_write];

        unsafe {
            self.device()
                .logical()
                .update_descriptor_sets(&descriptor_writes, &[]);
        }

        set
    }

    pub(crate) fn image_set(&self) -> DescriptorSet {
        let set_layouts = [self.image_layout];
        let set_alloc_info = DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(&set_layouts)
            .build();

        unsafe {
            self.device()
                .logical()
                .allocate_descriptor_sets(&set_alloc_info)
                .or_error("cannot allocate image descriptor set")[0]
        }
    }

    pub(crate) fn pipeline(&self) -> PipelineLayout {
        self.pipeline_layout
    }

    fn device(&self) -> Rc<Device> {
        self.device.upgrade().or_error("device has been dropped")
    }
}

impl Drop for ShaderLayout {
    fn drop(&mut self) {
        unsafe {
            self.device()
                .logical()
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.device()
                .logical()
                .destroy_descriptor_set_layout(self.world_layout, None);
            self.device()
                .logical()
                .destroy_descriptor_set_layout(self.material_layout, None);
            self.device()
                .logical()
                .destroy_descriptor_set_layout(self.image_layout, None);
            self.device()
                .logical()
                .destroy_descriptor_pool(self.descriptor_pool, None);
        }
    }
}
