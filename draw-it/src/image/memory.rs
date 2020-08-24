// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// ImageMemory - struct that manages allocated image memory

use std::cmp;
use std::ptr;
use std::rc::Rc;

use super::ImageFormat;
use super::ImageLayout;
use super::ImageMips;
use super::ImageUsage;
use super::Msaa;
use crate::buffer::BufferMemory;
use crate::device::Commands;
use crate::device::Device;
use crate::vk;

pub(crate) struct ImageMemory {
    handle: vk::Image,
    memory: Option<vk::DeviceMemory>,
    views: Vec<vk::ImageView>,
    width: u32,
    height: u32,
    mip_count: u32,
    layer_count: u32,
    format: ImageFormat,
    layout: ImageLayout,
    device: Rc<Device>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct ImageMemoryOptions<'usage> {
    pub(crate) handle: Option<vk::Image>,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) format: ImageFormat,
    pub(crate) usage: &'usage [ImageUsage],
    pub(crate) mips: ImageMips,
    pub(crate) msaa: Msaa,
    pub(crate) cubemap: bool,
}

impl ImageMemory {
    pub(crate) fn new(device: &Rc<Device>, options: ImageMemoryOptions<'_>) -> Self {
        // calculate mip count
        let mip_count = match options.mips {
            ImageMips::One => 1,
            ImageMips::Log2 => {
                (cmp::max(options.width, options.height) as f32)
                    .log2()
                    .floor() as u32
                    + 1
            }
        };

        // initial layout
        let layout = ImageLayout::Undefined;

        // cubemap info
        let array_layers = if options.cubemap { 6 } else { 1 };
        let flags = if options.cubemap {
            vk::IMAGE_CREATE_CUBE_COMPATIBLE_BIT
        } else {
            0
        };

        // allocate memory if a handle was not supplied
        // swapchain images already have memory allocated
        let (handle, memory) = match options.handle {
            Some(handle) => (handle, None),
            None => {
                // create image
                let image_info = vk::ImageCreateInfo {
                    s_type: vk::STRUCTURE_TYPE_IMAGE_CREATE_INFO,
                    p_next: ptr::null(),
                    image_type: vk::IMAGE_TYPE_2D,
                    format: options.format.flag(),
                    extent: vk::Extent3D {
                        width: options.width,
                        height: options.height,
                        depth: 1,
                    },
                    mip_levels: mip_count,
                    samples: options.msaa.flag(),
                    tiling: vk::IMAGE_TILING_OPTIMAL,
                    usage: ImageUsage::combine(options.usage),
                    sharing_mode: vk::SHARING_MODE_EXCLUSIVE,
                    queue_family_index_count: 0,
                    p_queue_family_indices: ptr::null(),
                    initial_layout: layout.flag(),
                    array_layers,
                    flags,
                };

                let (handle, memory) = device.allocate_image(&image_info);
                (handle, Some(memory))
            }
        };

        Self {
            device: Rc::clone(device),
            width: options.width,
            height: options.height,
            format: options.format,
            views: vec![],
            layer_count: array_layers,
            handle,
            mip_count,
            layout,
            memory,
        }
    }

    pub(crate) fn add_view(&mut self) -> vk::ImageView {
        let view_type = if self.layer_count == 6 {
            vk::IMAGE_VIEW_TYPE_CUBE
        } else {
            vk::IMAGE_VIEW_TYPE_2D
        };

        let subresource = vk::ImageSubresourceRange {
            aspect_mask: self.format.aspect(),
            base_mip_level: 0,
            level_count: self.mip_count,
            base_array_layer: 0,
            layer_count: self.layer_count,
        };
        let view_info = vk::ImageViewCreateInfo {
            s_type: vk::STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            image: self.handle,
            format: self.format.flag(),
            components: vk::ComponentMapping {
                r: vk::COMPONENT_SWIZZLE_R,
                g: vk::COMPONENT_SWIZZLE_G,
                b: vk::COMPONENT_SWIZZLE_B,
                a: vk::COMPONENT_SWIZZLE_A,
            },
            subresource_range: subresource,
            view_type,
        };

        let view = self.device.create_image_view(&view_info);
        self.views.push(view);
        view
    }

    pub(crate) fn copy_from_memory(&self, memory: &BufferMemory, layer: u32) {
        debug_assert!(layer < self.layer_count, "layer out of bounds");

        self.device.do_commands(|cmd| {
            let subresource = vk::ImageSubresourceLayers {
                aspect_mask: self.format.aspect(),
                mip_level: 0,
                base_array_layer: layer,
                layer_count: 1,
            };
            let region = vk::BufferImageCopy {
                buffer_offset: 0,
                buffer_row_length: 0,
                buffer_image_height: 0,
                image_subresource: subresource,
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                image_extent: vk::Extent3D {
                    width: self.width,
                    height: self.height,
                    depth: 1,
                },
            };

            cmd.copy_buffer_to_image(memory.handle(), self.handle, region);
        });
    }

    pub(crate) fn change_layout(&mut self, new_layout: ImageLayout) {
        self.device.do_commands(|cmd| {
            cmd.change_image_layout(
                self,
                self.layout,
                new_layout,
                0..self.mip_count,
                0..self.layer_count,
            );
        });
        self.layout = new_layout;
    }

    pub(crate) fn change_layout_sync(&mut self, cmd: &Commands, new_layout: ImageLayout) {
        cmd.change_image_layout(
            self,
            self.layout,
            new_layout,
            0..self.mip_count,
            0..self.layer_count,
        );
        self.layout = new_layout;
    }

    pub(crate) fn generate_mipmaps(&self) {
        self.device.do_commands(|cmd| {
            for i in 1..self.mip_count {
                cmd.change_image_layout(
                    self,
                    ImageLayout::TransferDst,
                    ImageLayout::TransferSrc,
                    (i - 1)..i,
                    0..self.layer_count,
                );

                cmd.blit_image_mip(self, i - 1, i);

                cmd.change_image_layout(
                    self,
                    ImageLayout::TransferSrc,
                    ImageLayout::ShaderColor,
                    (i - 1)..i,
                    0..self.layer_count,
                );
            }
            cmd.change_image_layout(
                self,
                ImageLayout::TransferDst,
                ImageLayout::ShaderColor,
                (self.mip_count - 1)..self.mip_count,
                0..self.layer_count,
            );
        });
    }

    pub(crate) fn get_view(&self, index: usize) -> vk::ImageView {
        self.views[index]
    }

    pub(crate) const fn handle(&self) -> vk::Image {
        self.handle
    }

    pub(crate) fn has_depth_format(&self) -> bool {
        self.format == ImageFormat::Depth || self.format == ImageFormat::DepthStencil
    }

    pub(crate) fn all_aspects(&self) -> vk::ImageAspectFlags {
        self.format.all_aspects()
    }

    pub(crate) const fn layer_count(&self) -> u32 {
        self.layer_count
    }

    pub(crate) const fn width(&self) -> u32 {
        self.width
    }

    pub(crate) const fn height(&self) -> u32 {
        self.height
    }
}

impl Drop for ImageMemory {
    fn drop(&mut self) {
        for view in &self.views {
            self.device.destroy_image_view(*view);
        }
        if let Some(memory) = self.memory {
            self.device.free_image(self.handle, memory);
        }
    }
}

impl Default for ImageMemoryOptions<'_> {
    fn default() -> Self {
        Self {
            handle: None,
            width: 1,
            height: 1,
            format: ImageFormat::Srgba,
            mips: ImageMips::One,
            msaa: Msaa::Disabled,
            cubemap: false,
            usage: &[],
        }
    }
}
