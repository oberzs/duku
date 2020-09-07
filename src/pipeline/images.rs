// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// uniform structs to manage shader accessible uniform data

use std::ptr;

use super::Descriptor;
use super::Sampler;
use super::ShaderLayout;
use crate::device::Device;
use crate::image::ImageLayout;
use crate::image::TextureFilter;
use crate::image::TextureWrap;
use crate::vk;

pub(crate) struct ShaderImages {
    descriptor: Descriptor,
    samplers: Vec<Sampler>,
    images: Vec<Option<vk::ImageView>>,
    skybox: Option<vk::ImageView>,
    should_update: bool,
}

impl ShaderImages {
    pub(crate) fn new(device: &Device, layout: &ShaderLayout, anisotropy: f32) -> Self {
        let descriptor = layout.image_set(device);

        // create sampler combinations
        let mut samplers = vec![];
        for filter in &[TextureFilter::Linear, TextureFilter::Nearest] {
            for wrap in &[
                TextureWrap::Repeat,
                TextureWrap::ClampBorder,
                TextureWrap::ClampEdge,
            ] {
                for mipmaps in &[true, false] {
                    samplers.push(Sampler::new(device, *wrap, *filter, *mipmaps, anisotropy));
                }
            }
        }

        Self {
            descriptor,
            samplers,
            images: vec![],
            skybox: None,
            should_update: true,
        }
    }

    pub(crate) fn add(&mut self, image: vk::ImageView) -> u32 {
        let next_index = self.images.len();

        // find free index
        let index = self
            .images
            .iter()
            .position(|img| img.is_none())
            .unwrap_or(next_index);

        // add new or replace image
        if index == next_index {
            self.images.push(Some(image));
        } else {
            self.images[index] = Some(image);
        }

        self.should_update = true;
        index as u32
    }

    pub(crate) fn remove(&mut self, index: u32) {
        debug_assert!(
            (index as usize) < self.images.len(),
            "image index out of bounds"
        );

        // mark image as removed
        self.images[index as usize] = None;

        self.should_update = true;
    }

    pub(crate) fn replace(&mut self, index: u32, image: vk::ImageView) {
        debug_assert!(
            (index as usize) < self.images.len(),
            "image index out of bounds"
        );

        self.images[index as usize] = Some(image);

        self.should_update = true;
    }

    pub(crate) fn set_skybox(&mut self, image: vk::ImageView) {
        self.skybox = Some(image);
        self.should_update = true;
    }

    pub(crate) fn update_if_needed(&mut self, device: &Device) {
        // update if image was added/removed
        if self.should_update {
            let mut writes = vec![];

            // configure image writes to descriptor
            let image_infos: Vec<_> = (0..100)
                .map(|i| {
                    // get image or default image
                    let image = match self.images.get(i) {
                        Some(Some(img)) => *img,
                        _ => self.images[0].expect("bad code"),
                    };

                    vk::DescriptorImageInfo {
                        sampler: 0,
                        image_view: image,
                        image_layout: ImageLayout::ShaderColor.flag(),
                    }
                })
                .collect();
            writes.push(vk::WriteDescriptorSet {
                s_type: vk::STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
                p_next: ptr::null(),
                dst_set: self.descriptor.1,
                dst_binding: 0,
                dst_array_element: 0,
                descriptor_count: image_infos.len() as u32,
                descriptor_type: vk::DESCRIPTOR_TYPE_SAMPLED_IMAGE,
                p_image_info: image_infos.as_ptr(),
                p_buffer_info: ptr::null(),
                p_texel_buffer_view: ptr::null(),
            });

            // configure sampler writes to descriptor
            let sampler_info: Vec<_> = self
                .samplers
                .iter()
                .map(|s| vk::DescriptorImageInfo {
                    sampler: s.handle(),
                    image_view: 0,
                    image_layout: ImageLayout::ShaderColor.flag(),
                })
                .collect();
            writes.push(vk::WriteDescriptorSet {
                s_type: vk::STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
                p_next: ptr::null(),
                dst_set: self.descriptor.1,
                dst_binding: 1,
                dst_array_element: 0,
                descriptor_count: sampler_info.len() as u32,
                descriptor_type: vk::DESCRIPTOR_TYPE_SAMPLER,
                p_image_info: sampler_info.as_ptr(),
                p_buffer_info: ptr::null(),
                p_texel_buffer_view: ptr::null(),
            });

            // configure skybox write to descriptor
            let mut skybox_info = vec![];
            if let Some(skybox) = self.skybox {
                skybox_info.push(vk::DescriptorImageInfo {
                    sampler: 0,
                    image_view: skybox,
                    image_layout: ImageLayout::ShaderColor.flag(),
                });
            };
            writes.push(vk::WriteDescriptorSet {
                s_type: vk::STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
                p_next: ptr::null(),
                dst_set: self.descriptor.1,
                dst_binding: 2,
                dst_array_element: 0,
                descriptor_count: skybox_info.len() as u32,
                descriptor_type: vk::DESCRIPTOR_TYPE_SAMPLED_IMAGE,
                p_image_info: skybox_info.as_ptr(),
                p_buffer_info: ptr::null(),
                p_texel_buffer_view: ptr::null(),
            });

            // write data to descriptor
            device.update_descriptor_sets(&writes);
            self.should_update = false;
        }
    }

    pub(crate) const fn descriptor(&self) -> Descriptor {
        self.descriptor
    }

    pub(crate) fn destroy(&self, device: &Device) {
        for sampler in &self.samplers {
            sampler.destroy(device);
        }
    }
}
