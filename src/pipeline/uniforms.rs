// Oliver Berzs
// https://github.com/oberzs/duku

// shader uniform management structs

use std::mem;
use std::ptr;

use super::Sampler;
use crate::buffer::Buffer;
use crate::device::Device;
use crate::device::FRAMES_IN_FLIGHT;
use crate::error::Error;
use crate::error::Result;
use crate::image::Filter;
use crate::image::ImageLayout;
use crate::image::Wrap;
use crate::math::Matrix4;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::vk;

const MAX_WORLDS: u32 = 100;
const MAX_MATERIALS: u32 = 100;
const MAX_TEXTURES: u32 = 100;
const MAX_CUBEMAPS: u32 = 100;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ShaderWorld {
    pub(crate) world_to_view: Matrix4,
    pub(crate) view_to_clip: Matrix4,
    pub(crate) lights: [ShaderLight; 4],
    pub(crate) camera_position: Vector3,
    pub(crate) time: f32,
    pub(crate) world_to_shadow: [Matrix4; 4],
    pub(crate) shadow_splits: [f32; 4],
    pub(crate) shadow_texels: [f32; 4],
    pub(crate) shadow_diameters: [f32; 4],
    pub(crate) ambient_color: Vector3,
    pub(crate) shadow_pcf: f32,
    pub(crate) skybox_index: u32,
    pub(crate) max_white_point: f32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ShaderMaterial {
    pub(crate) a: Vector4,
    pub(crate) b: Vector4,
    pub(crate) c: Vector4,
    pub(crate) d: Vector4,
    pub(crate) e: Vector4,
    pub(crate) f: Vector4,
    pub(crate) g: Vector4,
    pub(crate) h: Vector4,
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub(crate) struct ShaderLight {
    pub(crate) coords: Vector3,
    pub(crate) light_type: i32,
    pub(crate) color: Vector4,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ShaderConstants {
    pub(crate) local_to_world: Matrix4,
    pub(crate) tint_color: Vector3,
    pub(crate) sampler_index: u32,
}

pub(crate) struct Uniforms {
    pipeline_layout: vk::PipelineLayout,
    world_layout: vk::DescriptorSetLayout,
    material_layout: vk::DescriptorSetLayout,
    image_layout: vk::DescriptorSetLayout,
    shadow_map_layout: vk::DescriptorSetLayout,

    world_count: u32,
    material_count: u32,

    descriptor_pool: vk::DescriptorPool,
    image_descriptor: Descriptor,
    should_update_images: bool,

    samplers: Vec<Sampler>,
    textures: Vec<Option<vk::ImageView>>,
    cubemaps: Vec<Option<vk::ImageView>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Descriptor(pub(crate) u32, pub(crate) vk::DescriptorSet);

impl Uniforms {
    pub(crate) fn new(device: &Device, anisotropy: f32) -> Self {
        // world uniform layout
        let world_layout = device.create_descriptor_set_layout(&[vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::SHADER_STAGE_VERTEX_BIT | vk::SHADER_STAGE_FRAGMENT_BIT,
            p_immutable_samplers: ptr::null(),
        }]);

        // material uniform layout
        let material_layout =
            device.create_descriptor_set_layout(&[vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
                descriptor_count: 1,
                stage_flags: vk::SHADER_STAGE_VERTEX_BIT | vk::SHADER_STAGE_FRAGMENT_BIT,
                p_immutable_samplers: ptr::null(),
            }]);

        // image uniform layout
        let image_layout = device.create_descriptor_set_layout(&[
            // textures
            vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk::DESCRIPTOR_TYPE_SAMPLED_IMAGE,
                descriptor_count: MAX_TEXTURES,
                stage_flags: vk::SHADER_STAGE_FRAGMENT_BIT,
                p_immutable_samplers: ptr::null(),
            },
            // samplers
            vk::DescriptorSetLayoutBinding {
                binding: 1,
                descriptor_type: vk::DESCRIPTOR_TYPE_SAMPLER,
                descriptor_count: 2 * 3 * 2,
                stage_flags: vk::SHADER_STAGE_FRAGMENT_BIT,
                p_immutable_samplers: ptr::null(),
            },
            // cubemaps
            vk::DescriptorSetLayoutBinding {
                binding: 2,
                descriptor_type: vk::DESCRIPTOR_TYPE_SAMPLED_IMAGE,
                descriptor_count: MAX_CUBEMAPS,
                stage_flags: vk::SHADER_STAGE_FRAGMENT_BIT,
                p_immutable_samplers: ptr::null(),
            },
        ]);

        // shadow map layout
        let shadow_map_layout =
            device.create_descriptor_set_layout(&[vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk::DESCRIPTOR_TYPE_SAMPLED_IMAGE,
                descriptor_count: 4,
                stage_flags: vk::SHADER_STAGE_FRAGMENT_BIT,
                p_immutable_samplers: ptr::null(),
            }]);

        // descriptor pool
        let descriptor_pool = device.create_descriptor_pool(&[
            vk::DescriptorPoolSize {
                vk_type: vk::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
                descriptor_count: MAX_WORLDS,
            },
            vk::DescriptorPoolSize {
                vk_type: vk::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
                descriptor_count: MAX_MATERIALS,
            },
            vk::DescriptorPoolSize {
                vk_type: vk::DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
                descriptor_count: 1,
            },
            vk::DescriptorPoolSize {
                vk_type: vk::DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
                descriptor_count: FRAMES_IN_FLIGHT as u32,
            },
        ]);

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
        let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_info);

        // allocate image descriptor
        let image_descriptor = Descriptor(
            2,
            device.allocate_descriptor_set(image_layout, descriptor_pool),
        );

        // create sampler combinations
        let mut samplers = vec![];
        for filter in &[Filter::Linear, Filter::Nearest] {
            for wrap in &[Wrap::Repeat, Wrap::ClampBorder, Wrap::ClampEdge] {
                for mipmaps in &[true, false] {
                    samplers.push(Sampler::new(device, *wrap, *filter, *mipmaps, anisotropy));
                }
            }
        }

        Self {
            pipeline_layout,
            world_layout,
            material_layout,
            image_layout,
            shadow_map_layout,
            descriptor_pool,
            image_descriptor,
            samplers,
            world_count: 0,
            material_count: 0,
            textures: vec![],
            cubemaps: vec![],
            should_update_images: true,
        }
    }

    pub(crate) fn add_texture(&mut self, image: vk::ImageView) -> Result<u32> {
        // check if full
        if self.textures.len() == MAX_TEXTURES as usize {
            return Err(Error::TextureLimit);
        }

        let next_index = self.textures.len();

        // find free index
        let index = self
            .textures
            .iter()
            .position(|img| img.is_none())
            .unwrap_or(next_index);

        // add new or replace image
        if index == next_index {
            self.textures.push(Some(image));
        } else {
            self.textures[index] = Some(image);
        }

        self.should_update_images = true;
        Ok(index as u32)
    }

    pub(crate) fn remove_texture(&mut self, index: u32) {
        debug_assert!(
            (index as usize) < self.textures.len(),
            "image index out of bounds"
        );

        // mark image as removed
        self.textures[index as usize] = None;

        self.should_update_images = true;
    }

    pub(crate) fn replace_texture(&mut self, index: u32, image: vk::ImageView) {
        debug_assert!(
            (index as usize) < self.textures.len(),
            "image index out of bounds"
        );

        self.textures[index as usize] = Some(image);

        self.should_update_images = true;
    }

    pub(crate) fn add_cubemap(&mut self, image: vk::ImageView) -> Result<u32> {
        // check if full
        if self.cubemaps.len() == MAX_CUBEMAPS as usize {
            return Err(Error::CubemapLimit);
        }

        let next_index = self.cubemaps.len();

        // find free index
        let index = self
            .cubemaps
            .iter()
            .position(|img| img.is_none())
            .unwrap_or(next_index);

        // add new or replace image
        if index == next_index {
            self.cubemaps.push(Some(image));
        } else {
            self.cubemaps[index] = Some(image);
        }

        self.should_update_images = true;
        Ok(index as u32)
    }

    pub(crate) fn remove_cubemap(&mut self, index: u32) {
        debug_assert!(
            (index as usize) < self.cubemaps.len(),
            "image index out of bounds"
        );

        // mark image as removed
        self.cubemaps[index as usize] = None;

        self.should_update_images = true;
    }

    pub(crate) fn update_if_needed(&mut self, device: &Device) {
        // update if image was added/removed
        if self.should_update_images {
            let mut writes = vec![];

            // configure image writes to descriptor
            let image_infos: Vec<_> = (0..MAX_TEXTURES)
                .map(|i| {
                    // get image or default image
                    let image = match self.textures.get(i as usize) {
                        Some(Some(img)) => *img,
                        _ => self.textures[0].expect("bad code"),
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
                dst_set: self.image_descriptor.1,
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
                dst_set: self.image_descriptor.1,
                dst_binding: 1,
                dst_array_element: 0,
                descriptor_count: sampler_info.len() as u32,
                descriptor_type: vk::DESCRIPTOR_TYPE_SAMPLER,
                p_image_info: sampler_info.as_ptr(),
                p_buffer_info: ptr::null(),
                p_texel_buffer_view: ptr::null(),
            });

            // configure cubemap writes to descriptor
            let cubemap_infos: Vec<_> = (0..MAX_CUBEMAPS)
                .map(|i| {
                    // get cubemap or default cubemap
                    let cubemap = match self.cubemaps.get(i as usize) {
                        Some(Some(cbm)) => *cbm,
                        _ => self.cubemaps[0].expect("bad code"),
                    };

                    vk::DescriptorImageInfo {
                        sampler: 0,
                        image_view: cubemap,
                        image_layout: ImageLayout::ShaderColor.flag(),
                    }
                })
                .collect();
            writes.push(vk::WriteDescriptorSet {
                s_type: vk::STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
                p_next: ptr::null(),
                dst_set: self.image_descriptor.1,
                dst_binding: 2,
                dst_array_element: 0,
                descriptor_count: cubemap_infos.len() as u32,
                descriptor_type: vk::DESCRIPTOR_TYPE_SAMPLED_IMAGE,
                p_image_info: cubemap_infos.as_ptr(),
                p_buffer_info: ptr::null(),
                p_texel_buffer_view: ptr::null(),
            });

            // write data to descriptor
            device.update_descriptor_sets(&writes);
            self.should_update_images = false;
        }
    }

    pub(crate) fn world_set(
        &mut self,
        device: &Device,
        buffer: &Buffer<ShaderWorld>,
    ) -> Result<Descriptor> {
        // check limits
        if self.world_count == MAX_WORLDS {
            return Err(Error::CanvasLimit);
        }
        self.world_count += 1;

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

        Ok(Descriptor(0, set))
    }

    pub(crate) fn material_set(
        &mut self,
        device: &Device,
        buffer: &Buffer<ShaderMaterial>,
    ) -> Result<Descriptor> {
        // check limits
        if self.material_count == MAX_MATERIALS {
            return Err(Error::MaterialLimit);
        }
        self.material_count += 1;

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

        Ok(Descriptor(1, set))
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

    pub(crate) const fn pipeline_layout(&self) -> vk::PipelineLayout {
        self.pipeline_layout
    }

    pub(crate) const fn image_descriptor(&self) -> Descriptor {
        self.image_descriptor
    }

    pub(crate) fn destroy(&self, device: &Device) {
        for sampler in &self.samplers {
            sampler.destroy(device);
        }
        device.destroy_pipeline_layout(self.pipeline_layout);
        device.destroy_descriptor_set_layout(self.world_layout);
        device.destroy_descriptor_set_layout(self.material_layout);
        device.destroy_descriptor_set_layout(self.image_layout);
        device.destroy_descriptor_set_layout(self.shadow_map_layout);
        device.destroy_descriptor_pool(self.descriptor_pool);
    }
}
