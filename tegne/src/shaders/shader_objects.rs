use ash::version::DeviceV1_0;
use ash::vk::DescriptorImageInfo;
use ash::vk::DescriptorSet;
use ash::vk::DescriptorType;
use ash::vk::ImageView;
use ash::vk::WriteDescriptorSet;
use log::debug;
use log::info;
use std::cell::Cell;
use std::cell::RefCell;
use std::sync::Arc;
use tegne_math::Matrix4;
use tegne_math::Vector2;
use tegne_math::Vector3;
use tegne_math::Vector4;

use super::ShaderLayout;
use crate::buffers::BufferType;
use crate::buffers::DynamicBuffer;
use crate::error::Result;
use crate::images::ImageLayout;
use crate::images::Sampler;
use crate::images::SamplerAddress;
use crate::images::SamplerFilter;
use crate::images::SamplerOptions;
use crate::instance::Device;

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub(crate) struct Light {
    pub(crate) coords: Vector4,
    pub(crate) color: Vector4,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct WorldObject {
    pub(crate) cam_mat: Matrix4,
    pub(crate) light_mat: Matrix4,
    pub(crate) lights: [Light; 4],
    pub(crate) cam_pos: Vector3,
    pub(crate) time: f32,
    pub(crate) shadow_index: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct MaterialObject {
    pub(crate) albedo_tint: Vector3,
    pub(crate) font_width: f32,
    pub(crate) font_border_tint: Vector3,
    pub(crate) font_edge: f32,
    pub(crate) font_border_offset: Vector2,
    pub(crate) font_border_width: f32,
    pub(crate) font_border_edge: f32,
    pub(crate) arg_1: Vector4,
    pub(crate) arg_2: Vector4,
    pub(crate) arg_3: Vector4,
    pub(crate) arg_4: Vector4,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct PushConstants {
    pub(crate) model_mat: Matrix4,
    pub(crate) albedo_index: i32,
}

pub(crate) struct WorldUniforms {
    descriptor: Descriptor,
    buffer: DynamicBuffer,
}

pub(crate) struct MaterialUniforms {
    descriptor: Descriptor,
    buffer: DynamicBuffer,
}

pub(crate) struct ImageUniforms {
    descriptor: Descriptor,
    linear_repeat_sampler: Sampler,
    linear_clamp_sampler: Sampler,
    nearest_repeat_sampler: Sampler,
    images: RefCell<Vec<ImageView>>,
    should_update: Cell<bool>,
    device: Arc<Device>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Descriptor(pub u32, pub DescriptorSet);

impl WorldUniforms {
    pub(crate) fn new(device: &Arc<Device>, layout: &ShaderLayout) -> Result<Self> {
        let buffer = DynamicBuffer::new::<WorldObject>(device, 1, BufferType::Uniform)?;

        let descriptor_set = layout.world_set(&buffer)?;
        let descriptor = Descriptor(0, descriptor_set);

        Ok(Self { buffer, descriptor })
    }

    pub(crate) fn update(&self, data: WorldObject) -> Result<()> {
        self.buffer.update_data(&[data])
    }

    pub(crate) fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}

impl MaterialUniforms {
    pub(crate) fn new(device: &Arc<Device>, layout: &ShaderLayout) -> Result<Self> {
        let buffer = DynamicBuffer::new::<MaterialObject>(device, 1, BufferType::Uniform)?;

        let descriptor_set = layout.material_set(&buffer)?;
        let descriptor = Descriptor(1, descriptor_set);

        Ok(Self { buffer, descriptor })
    }

    pub(crate) fn update(&self, data: MaterialObject) -> Result<()> {
        self.buffer.update_data(&[data])
    }

    pub(crate) fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}

impl ImageUniforms {
    pub(crate) fn new(
        device: &Arc<Device>,
        layout: &ShaderLayout,
        anisotropy: f32,
    ) -> Result<Self> {
        debug!("creating image uniforms");
        info!("using anisotropy level {}", anisotropy);

        let descriptor_set = layout.image_set()?;
        let descriptor = Descriptor(2, descriptor_set);
        let linear_repeat_sampler = Sampler::new(
            device,
            SamplerOptions {
                anisotropy,
                ..Default::default()
            },
        )?;
        let linear_clamp_sampler = Sampler::new(
            device,
            SamplerOptions {
                anisotropy,
                address: SamplerAddress::Clamp,
                ..Default::default()
            },
        )?;
        let nearest_repeat_sampler = Sampler::new(
            device,
            SamplerOptions {
                anisotropy,
                filter: SamplerFilter::Nearest,
                ..Default::default()
            },
        )?;

        Ok(Self {
            descriptor,
            linear_repeat_sampler,
            linear_clamp_sampler,
            nearest_repeat_sampler,
            images: RefCell::new(vec![]),
            should_update: Cell::new(true),
            device: Arc::clone(device),
        })
    }

    pub(crate) fn image_count(&self) -> u32 {
        self.images.borrow().len() as u32
    }

    pub(crate) fn add(&self, image: ImageView) {
        self.images.borrow_mut().push(image);
        self.should_update.set(true);
    }

    pub(crate) fn update_if_needed(&self) {
        if self.should_update.get() {
            let image_infos = (0..100)
                .map(|i| {
                    let has_image = i < self.images.borrow().len();
                    let index = if has_image { i } else { 0 };
                    DescriptorImageInfo::builder()
                        .image_layout(ImageLayout::Shader.flag())
                        .image_view(self.images.borrow()[index])
                        .build()
                })
                .collect::<Vec<_>>();
            let sampler_info = [
                DescriptorImageInfo::builder()
                    .image_layout(ImageLayout::Shader.flag())
                    .sampler(self.linear_repeat_sampler.vk())
                    .build(),
                DescriptorImageInfo::builder()
                    .image_layout(ImageLayout::Shader.flag())
                    .sampler(self.linear_clamp_sampler.vk())
                    .build(),
                DescriptorImageInfo::builder()
                    .image_layout(ImageLayout::Shader.flag())
                    .sampler(self.nearest_repeat_sampler.vk())
                    .build(),
            ];

            let image_write = WriteDescriptorSet::builder()
                .dst_set(self.descriptor.1)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(DescriptorType::SAMPLED_IMAGE)
                .image_info(&image_infos)
                .build();
            let sampler_write = WriteDescriptorSet::builder()
                .dst_set(self.descriptor.1)
                .dst_binding(1)
                .dst_array_element(0)
                .descriptor_type(DescriptorType::SAMPLER)
                .image_info(&sampler_info)
                .build();

            let writes = [image_write, sampler_write];
            unsafe {
                self.device.logical().update_descriptor_sets(&writes, &[]);
            }

            self.should_update.set(false);
        }
    }

    pub(crate) fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}
