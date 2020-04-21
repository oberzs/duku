use ash::version::DeviceV1_0;
use ash::vk::DescriptorImageInfo;
use ash::vk::DescriptorSet;
use ash::vk::DescriptorType;
use ash::vk::ImageLayout;
use ash::vk::ImageView;
use ash::vk::WriteDescriptorSet;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use tegne_math::Matrix4;
use tegne_math::Vector3;
use tegne_math::Vector4;

use super::ShaderLayout;
use crate::buffer::BufferType;
use crate::buffer::DynamicBuffer;
use crate::images::Anisotropy;
use crate::images::Sampler;
use crate::tegne::Device;

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct Light {
    pub position: Vector4,
    pub color: Vector3,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WorldObject {
    pub view: Matrix4,
    pub proj: Matrix4,
    pub light_matrix: Matrix4,
    pub lights: [Light; 4],
    pub view_pos: Vector3,
    pub time: f32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MaterialObject {
    pub albedo_tint: Vector4,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PushConstants {
    pub model: Matrix4,
    pub albedo_index: i32,
}

pub struct WorldUniforms {
    descriptor: DescriptorSet,
    buffer: DynamicBuffer,
}

pub struct MaterialUniforms {
    descriptor: DescriptorSet,
    buffer: DynamicBuffer,
}

pub struct ImageUniforms {
    descriptor: DescriptorSet,
    sampler: Sampler,
    images: RefCell<Vec<ImageView>>,
    should_update: Cell<bool>,
    device: Rc<Device>,
}

impl WorldUniforms {
    pub fn new(device: &Rc<Device>, layout: &ShaderLayout) -> Self {
        let buffer = DynamicBuffer::new::<WorldObject>(device, 1, BufferType::Uniform);

        let descriptor = layout.world_set(&buffer);

        Self { buffer, descriptor }
    }

    pub fn update(&self, data: WorldObject) {
        self.buffer.update_data(&[data]);
    }

    pub fn descriptor(&self) -> (u32, DescriptorSet) {
        (0, self.descriptor)
    }
}

impl MaterialUniforms {
    pub fn new(device: &Rc<Device>, layout: &ShaderLayout) -> Self {
        let buffer = DynamicBuffer::new::<MaterialObject>(device, 1, BufferType::Uniform);

        let descriptor = layout.material_set(&buffer);

        Self { buffer, descriptor }
    }

    pub fn update(&self, data: MaterialObject) {
        self.buffer.update_data(&[data]);
    }

    pub fn descriptor(&self) -> (u32, DescriptorSet) {
        (1, self.descriptor)
    }
}

impl ImageUniforms {
    pub fn new(device: &Rc<Device>, layout: &ShaderLayout, anisotropy: Anisotropy) -> Self {
        let descriptor = layout.image_set();
        let sampler = Sampler::new(device, anisotropy);

        Self {
            descriptor,
            sampler,
            images: RefCell::new(vec![]),
            should_update: Cell::new(true),
            device: Rc::clone(device),
        }
    }

    pub fn image_count(&self) -> u32 {
        self.images.borrow().len() as u32
    }

    pub fn add(&self, image: ImageView) {
        self.images.borrow_mut().push(image);
        self.should_update.set(true);
    }

    pub fn update_if_needed(&self) {
        if self.should_update.get() {
            let image_infos = (0..100)
                .map(|i| {
                    let has_image = i < self.images.borrow().len();
                    let index = if has_image { i } else { 0 };
                    DescriptorImageInfo::builder()
                        .image_layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                        .image_view(self.images.borrow()[index])
                        .build()
                })
                .collect::<Vec<_>>();
            let sampler_info = [DescriptorImageInfo::builder()
                .image_layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .sampler(self.sampler.vk())
                .build()];

            let image_write = WriteDescriptorSet::builder()
                .dst_set(self.descriptor)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(DescriptorType::SAMPLED_IMAGE)
                .image_info(&image_infos)
                .build();
            let sampler_write = WriteDescriptorSet::builder()
                .dst_set(self.descriptor)
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

    pub fn descriptor(&self) -> (u32, DescriptorSet) {
        (2, self.descriptor)
    }
}
