// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// uniform structs to manage shader accessible uniform data

use ash::vk;
use log::debug;
use log::info;
use std::cell::Cell;
use std::cell::RefCell;
use std::sync::Arc;

use super::MaterialData;
use super::Sampler;
use super::SamplerAddress;
use super::SamplerFilter;
use super::SamplerMipmaps;
use super::SamplerOptions;
use super::ShaderLayout;
use super::WorldData;
use crate::buffer::BufferUsage;
use crate::buffer::DynamicBuffer;
use crate::device::Device;
use crate::error::Result;
use crate::image::ImageLayout;
use crate::profile_scope;

pub(crate) struct WorldUniform {
    descriptor: Descriptor,
    buffer: DynamicBuffer,
}

pub(crate) struct MaterialUniform {
    descriptor: Descriptor,
    buffer: DynamicBuffer,
}

pub(crate) struct ImageUniform {
    descriptor: Descriptor,
    sampler_combinations: Vec<Sampler>,
    images: RefCell<Vec<vk::ImageView>>,
    should_update: Cell<bool>,
    device: Arc<Device>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Descriptor(pub u32, pub vk::DescriptorSet);

impl WorldUniform {
    pub(crate) fn new(device: &Arc<Device>, layout: &ShaderLayout) -> Result<Self> {
        let buffer = DynamicBuffer::new::<WorldData>(device, BufferUsage::Uniform, 1)?;

        let descriptor_set = layout.world_set(&buffer)?;
        let descriptor = Descriptor(0, descriptor_set);

        Ok(Self { buffer, descriptor })
    }

    pub(crate) fn update(&self, data: WorldData) -> Result<()> {
        self.buffer.update_data(&[data])
    }

    pub(crate) fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}

impl MaterialUniform {
    pub(crate) fn new(device: &Arc<Device>, layout: &ShaderLayout) -> Result<Self> {
        let buffer = DynamicBuffer::new::<MaterialData>(device, BufferUsage::Uniform, 1)?;

        let descriptor_set = layout.material_set(&buffer)?;
        let descriptor = Descriptor(1, descriptor_set);

        Ok(Self { buffer, descriptor })
    }

    pub(crate) fn update(&self, data: MaterialData) -> Result<()> {
        self.buffer.update_data(&[data])
    }

    pub(crate) fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}

impl ImageUniform {
    pub(crate) fn new(
        device: &Arc<Device>,
        layout: &ShaderLayout,
        anisotropy: f32,
    ) -> Result<Self> {
        profile_scope!("new");
        debug!("creating image uniforms");
        info!("using anisotropy level {}", anisotropy);

        let descriptor_set = layout.image_set()?;
        let descriptor = Descriptor(2, descriptor_set);

        let mut sampler_combinations = vec![];
        for filter in &[SamplerFilter::Linear, SamplerFilter::Nearest] {
            for address in &[SamplerAddress::Repeat, SamplerAddress::Clamp] {
                for mipmaps in &[SamplerMipmaps::Enabled, SamplerMipmaps::Disabled] {
                    sampler_combinations.push(Sampler::new(
                        device,
                        SamplerOptions {
                            anisotropy,
                            filter: *filter,
                            address: *address,
                            mipmaps: *mipmaps,
                        },
                    )?);
                }
            }
        }

        Ok(Self {
            descriptor,
            sampler_combinations,
            images: RefCell::new(vec![]),
            should_update: Cell::new(true),
            device: Arc::clone(device),
        })
    }

    pub(crate) fn add(&self, image: vk::ImageView) -> i32 {
        let mut images = self.images.borrow_mut();
        let index = images.len() as i32;
        images.push(image);
        self.should_update.set(true);
        index
    }

    pub(crate) fn remove(&self, index: i32) {
        self.images.borrow_mut().remove(index as usize);
        self.should_update.set(true);
    }

    pub(crate) fn update_if_needed(&self) {
        // update if image was added/removed
        if self.should_update.get() {
            // configure image writes to descriptor
            let image_infos = (0..100)
                .map(|i| {
                    let has_image = i < self.images.borrow().len();
                    let index = if has_image { i } else { 0 };
                    vk::DescriptorImageInfo::builder()
                        .image_layout(ImageLayout::Shader.flag())
                        .image_view(self.images.borrow()[index])
                        .build()
                })
                .collect::<Vec<_>>();
            let image_write = vk::WriteDescriptorSet::builder()
                .dst_set(self.descriptor.1)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                .image_info(&image_infos)
                .build();

            // configure sampler writes to descriptor
            let sampler_info = self
                .sampler_combinations
                .iter()
                .map(|s| {
                    vk::DescriptorImageInfo::builder()
                        .image_layout(ImageLayout::Shader.flag())
                        .sampler(s.handle())
                        .build()
                })
                .collect::<Vec<_>>();

            let sampler_write = vk::WriteDescriptorSet::builder()
                .dst_set(self.descriptor.1)
                .dst_binding(1)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::SAMPLER)
                .image_info(&sampler_info)
                .build();

            // write data to descriptor
            let writes = [image_write, sampler_write];
            self.device.update_descriptor_sets(&writes);
            self.should_update.set(false);
        }
    }

    pub(crate) fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}
