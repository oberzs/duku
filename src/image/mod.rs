// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Image - struct that manages allocated image memory

mod cubemap;
mod framebuffer;
mod properties;
mod texture;

use std::cmp;
use std::ptr;
use std::rc::Rc;

use crate::buffer::Buffer;
use crate::device::Commands;
use crate::device::Device;
use crate::pipeline::Attachment;
use crate::vk;

pub(crate) use cubemap::Cubemap;
pub(crate) use framebuffer::CoreFramebuffer;
pub(crate) use framebuffer::FramebufferData;
pub(crate) use framebuffer::FramebufferOptions;
pub(crate) use properties::with_alpha;
pub(crate) use properties::ImageFormat;
pub(crate) use properties::ImageLayout;
pub(crate) use properties::ImageUsage;
pub(crate) use texture::CoreTexture;

pub use cubemap::CubemapSides;
pub use framebuffer::Framebuffer;
pub use properties::Msaa;
pub use properties::TextureFilter;
pub use properties::TextureWrap;
pub use texture::Texture;

pub(crate) struct Image {
    handle: vk::Image,
    memory: Option<vk::DeviceMemory>,
    views: Vec<vk::ImageView>,
    width: u32,
    height: u32,
    mip_count: u32,
    layer_count: u32,
    format: ImageFormat,
    device: Rc<Device>,
}

impl Image {
    pub(crate) fn texture(
        device: &Rc<Device>,
        format: ImageFormat,
        width: u32,
        height: u32,
        cubemap: bool,
    ) -> Self {
        // calculate mip count
        let mip_count = (cmp::max(width, height) as f32).log2().floor() as u32 + 1;

        // cubemap info
        let layer_count = if cubemap { 6 } else { 1 };
        let flags = if cubemap {
            vk::IMAGE_CREATE_CUBE_COMPATIBLE_BIT
        } else {
            0
        };

        // create image
        let image_info = vk::ImageCreateInfo {
            s_type: vk::STRUCTURE_TYPE_IMAGE_CREATE_INFO,
            p_next: ptr::null(),
            image_type: vk::IMAGE_TYPE_2D,
            format: format.flag(),
            extent: vk::Extent3D {
                width,
                height,
                depth: 1,
            },
            mip_levels: mip_count,
            samples: Msaa::Disabled.flag(),
            tiling: vk::IMAGE_TILING_OPTIMAL,
            usage: ImageUsage::combine(&[
                ImageUsage::Sampled,
                ImageUsage::TransferSrc,
                ImageUsage::TransferDst,
            ]),
            sharing_mode: vk::SHARING_MODE_EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
            initial_layout: ImageLayout::Undefined.flag(),
            array_layers: layer_count,
            flags,
        };

        let (handle, memory) = device.allocate_image(&image_info);

        Self {
            device: Rc::clone(device),
            memory: Some(memory),
            views: vec![],
            layer_count,
            handle,
            mip_count,
            format,
            width,
            height,
        }
    }

    pub(crate) fn shader(device: &Rc<Device>, width: u32, height: u32) -> Self {
        let format = ImageFormat::Sbgra;

        // create image
        let image_info = vk::ImageCreateInfo {
            s_type: vk::STRUCTURE_TYPE_IMAGE_CREATE_INFO,
            p_next: ptr::null(),
            image_type: vk::IMAGE_TYPE_2D,
            format: format.flag(),
            extent: vk::Extent3D {
                width,
                height,
                depth: 1,
            },
            mip_levels: 1,
            samples: Msaa::Disabled.flag(),
            tiling: vk::IMAGE_TILING_OPTIMAL,
            usage: ImageUsage::combine(&[
                ImageUsage::TransferDst,
                ImageUsage::Sampled,
                ImageUsage::Color,
            ]),
            sharing_mode: vk::SHARING_MODE_EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
            initial_layout: ImageLayout::Undefined.flag(),
            array_layers: 1,
            flags: 0,
        };

        let (handle, memory) = device.allocate_image(&image_info);

        Self {
            device: Rc::clone(device),
            memory: Some(memory),
            views: vec![],
            layer_count: 1,
            mip_count: 1,
            format,
            handle,
            width,
            height,
        }
    }

    pub(crate) fn attachment(
        device: &Rc<Device>,
        attachment: &Attachment,
        width: u32,
        height: u32,
        external: Option<vk::Image>,
    ) -> Self {
        // configure usage
        let mut usage = vec![];

        match attachment.layout() {
            ImageLayout::Color => usage.push(ImageUsage::Color),
            ImageLayout::Depth => usage.push(ImageUsage::Depth),
            ImageLayout::ShaderColor => usage.push(ImageUsage::Color),
            ImageLayout::ShaderDepth => usage.push(ImageUsage::Depth),
            _ => (),
        }

        // attachments that stay in memory can be read from
        if attachment.is_stored() {
            usage.push(ImageUsage::Sampled);

            if external.is_none() {
                // swapchain images don't need to be transfered
                usage.push(ImageUsage::TransferSrc);
            }
        } else {
            usage.push(ImageUsage::Transient);
        }

        let format = attachment.format();

        // allocate memory if a handle was not supplied
        // swapchain images already have memory allocated
        let (handle, memory) = match external {
            Some(handle) => (handle, None),
            None => {
                // create image
                let image_info = vk::ImageCreateInfo {
                    s_type: vk::STRUCTURE_TYPE_IMAGE_CREATE_INFO,
                    p_next: ptr::null(),
                    image_type: vk::IMAGE_TYPE_2D,
                    format: format.flag(),
                    extent: vk::Extent3D {
                        width,
                        height,
                        depth: 1,
                    },
                    mip_levels: 1,
                    samples: attachment.msaa().flag(),
                    tiling: vk::IMAGE_TILING_OPTIMAL,
                    usage: ImageUsage::combine(&usage),
                    sharing_mode: vk::SHARING_MODE_EXCLUSIVE,
                    queue_family_index_count: 0,
                    p_queue_family_indices: ptr::null(),
                    initial_layout: ImageLayout::Undefined.flag(),
                    array_layers: 1,
                    flags: 0,
                };

                let (handle, memory) = device.allocate_image(&image_info);
                (handle, Some(memory))
            }
        };

        Self {
            device: Rc::clone(device),
            views: vec![],
            layer_count: 1,
            mip_count: 1,
            memory,
            format,
            handle,
            width,
            height,
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

    pub(crate) fn copy_from_buffer(&self, buffer: &Buffer<u8>, layer: u32) {
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

            cmd.copy_buffer_to_image(buffer.handle(), self.handle, region);
        });
    }

    pub(crate) fn change_layout(&self, from: ImageLayout, to: ImageLayout) {
        self.device.do_commands(|cmd| {
            cmd.change_image_layout(self, from, to, 0..self.mip_count, 0..self.layer_count);
        });
    }

    pub(crate) fn change_layout_sync(&self, cmd: &Commands, from: ImageLayout, to: ImageLayout) {
        cmd.change_image_layout(self, from, to, 0..self.mip_count, 0..self.layer_count);
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

    pub(crate) const fn all_aspects(&self) -> vk::ImageAspectFlags {
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

impl Drop for Image {
    fn drop(&mut self) {
        for view in &self.views {
            self.device.destroy_image_view(*view);
        }
        if let Some(memory) = self.memory {
            self.device.free_image(self.handle, memory);
        }
    }
}
