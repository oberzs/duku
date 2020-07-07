// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// uniform structs to manage shader accessible uniform data

use ash::vk;
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
    images: RefCell<Vec<Option<vk::ImageView>>>,
    should_update: Cell<bool>,
    device: Arc<Device>,
}

pub(crate) struct ShadowMapUniform {
    descriptor: Descriptor,
}

pub(crate) struct FramebufferUniform {
    descriptor: Descriptor,
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

impl PartialEq for MaterialUniform {
    fn eq(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}

impl ImageUniform {
    pub(crate) fn new(
        device: &Arc<Device>,
        layout: &ShaderLayout,
        anisotropy: f32,
    ) -> Result<Self> {
        profile_scope!("new");
        info!("using anisotropy level {}", anisotropy);

        let descriptor_set = layout.image_set()?;
        let descriptor = Descriptor(2, descriptor_set);

        // create sampler combinations
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

        // find free index
        let index = images
            .iter()
            .position(|img| img.is_none())
            .unwrap_or_else(|| images.len());

        // add new or replace image
        if index == images.len() {
            images.push(Some(image));
        } else {
            images[index] = Some(image);
        }

        self.should_update.set(true);
        index as i32
    }

    pub(crate) fn remove(&self, index: i32) {
        let mut images = self.images.borrow_mut();

        // mark image as removed
        images[index as usize] = None;

        self.should_update.set(true);
    }

    pub(crate) fn update_if_needed(&self) {
        let images = self.images.borrow();

        // update if image was added/removed
        if self.should_update.get() {
            // configure image writes to descriptor
            let image_infos = (0..100)
                .map(|i| {
                    // get image or default image
                    let image = match images.get(i) {
                        Some(Some(img)) => *img,
                        _ => images[0].expect("bad code"),
                    };

                    vk::DescriptorImageInfo::builder()
                        .image_layout(ImageLayout::ShaderColor.flag())
                        .image_view(image)
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
                        .image_layout(ImageLayout::ShaderColor.flag())
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

impl ShadowMapUniform {
    pub(crate) fn new(layout: &ShaderLayout, views: [vk::ImageView; 3]) -> Result<Self> {
        let descriptor_set = layout.shadow_map_set(views)?;
        let descriptor = Descriptor(3, descriptor_set);

        Ok(Self { descriptor })
    }

    pub(crate) fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}

impl FramebufferUniform {
    pub(crate) fn new(layout: &ShaderLayout, view: vk::ImageView) -> Result<Self> {
        let descriptor_set = layout.framebuffer_set(view)?;
        let descriptor = Descriptor(4, descriptor_set);

        Ok(Self { descriptor })
    }

    pub(crate) fn descriptor(&self) -> Descriptor {
        self.descriptor
    }
}
