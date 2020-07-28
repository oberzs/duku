// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// ImageMemory - struct that manages allocated image memory

use ash::vk;
use std::cmp;
use std::sync::Arc;

use super::ImageFormat;
use super::ImageLayout;
use super::ImageMips;
use super::ImageUsage;
use super::Msaa;
use crate::buffer::BufferMemory;
use crate::device::Device;
use crate::error::Result;

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
    device: Arc<Device>,
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
    pub(crate) fn new(device: &Arc<Device>, options: ImageMemoryOptions<'_>) -> Result<Self> {
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
            vk::ImageCreateFlags::CUBE_COMPATIBLE
        } else {
            vk::ImageCreateFlags::empty()
        };

        // allocate memory if a handle was not supplied
        // swapchain images already have memory allocated
        let (handle, memory) = match options.handle {
            Some(handle) => (handle, None),
            None => {
                // create image
                let image_info = vk::ImageCreateInfo::builder()
                    .image_type(vk::ImageType::TYPE_2D)
                    .extent(vk::Extent3D {
                        width: options.width,
                        height: options.height,
                        depth: 1,
                    })
                    .mip_levels(mip_count)
                    .array_layers(array_layers)
                    .format(options.format.flag())
                    .tiling(vk::ImageTiling::OPTIMAL)
                    .initial_layout(layout.flag())
                    .usage(ImageUsage::combine(options.usage))
                    .sharing_mode(vk::SharingMode::EXCLUSIVE)
                    .samples(options.msaa.flag())
                    .flags(flags);

                let (handle, memory) = device.allocate_image(&image_info)?;
                (handle, Some(memory))
            }
        };

        Ok(Self {
            device: Arc::clone(device),
            width: options.width,
            height: options.height,
            format: options.format,
            views: vec![],
            layer_count: array_layers,
            handle,
            mip_count,
            layout,
            memory,
        })
    }

    pub(crate) fn add_view(&mut self) -> Result<vk::ImageView> {
        let aspect_flags = match self.format {
            ImageFormat::Sbgra
            | ImageFormat::Rgb
            | ImageFormat::Rgba
            | ImageFormat::Srgba
            | ImageFormat::Srgb
            | ImageFormat::Float2
            | ImageFormat::Gray => vk::ImageAspectFlags::COLOR,
            ImageFormat::Depth => vk::ImageAspectFlags::DEPTH,
            ImageFormat::DepthStencil => {
                vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL
            }
        };

        let view_type = if self.layer_count == 6 {
            vk::ImageViewType::CUBE
        } else {
            vk::ImageViewType::TYPE_2D
        };

        let subresource = vk::ImageSubresourceRange::builder()
            .aspect_mask(aspect_flags)
            .base_mip_level(0)
            .base_array_layer(0)
            .layer_count(self.layer_count)
            .level_count(self.mip_count)
            .build();
        let view_info = vk::ImageViewCreateInfo::builder()
            .image(self.handle)
            .view_type(view_type)
            .format(self.format.flag())
            .subresource_range(subresource);

        let view = self.device.create_image_view(&view_info)?;
        self.views.push(view);
        Ok(view)
    }

    pub(crate) fn copy_from_memory(&self, memory: &BufferMemory, layer: u32) -> Result<()> {
        debug_assert!(layer < self.layer_count);

        let subresource = vk::ImageSubresourceLayers::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .base_array_layer(layer)
            .layer_count(1)
            .mip_level(0)
            .build();

        let region = vk::BufferImageCopy::builder()
            .buffer_offset(0)
            .buffer_row_length(0)
            .buffer_image_height(0)
            .image_subresource(subresource)
            .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
            .image_extent(vk::Extent3D {
                width: self.width,
                height: self.height,
                depth: 1,
            })
            .build();

        self.device.do_commands(|cmd| {
            self.device
                .cmd_copy_buffer_to_image(cmd, memory.handle(), self.handle, region);
            Ok(())
        })
    }

    pub(crate) fn change_layout(&mut self, new_layout: ImageLayout) -> Result<()> {
        self.device.do_commands(|cmd| {
            self.device.cmd_change_image_layout(
                cmd,
                self,
                self.layout,
                new_layout,
                0,
                self.mip_count,
            );
            Ok(())
        })?;
        self.layout = new_layout;
        Ok(())
    }

    pub(crate) fn change_layout_sync(&mut self, cmd: vk::CommandBuffer, new_layout: ImageLayout) {
        self.device
            .cmd_change_image_layout(cmd, self, self.layout, new_layout, 0, self.mip_count);
        self.layout = new_layout;
    }

    pub(crate) fn generate_mipmaps(&self) -> Result<()> {
        let mut mip_width = self.width as i32;
        let mut mip_height = self.height as i32;

        self.device.do_commands(|cmd| {
            for i in 1..self.mip_count {
                self.device.cmd_change_image_layout(
                    cmd,
                    self,
                    ImageLayout::TransferDst,
                    ImageLayout::TransferSrc,
                    i - 1,
                    1,
                );

                let src_offsets = [
                    vk::Offset3D { x: 0, y: 0, z: 0 },
                    vk::Offset3D {
                        x: mip_width,
                        y: mip_height,
                        z: 1,
                    },
                ];
                let src_subresource = vk::ImageSubresourceLayers::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .mip_level(i - 1)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build();

                mip_width = cmp::max(mip_width / 2, 1);
                mip_height = cmp::max(mip_height / 2, 1);
                let dst_offsets = [
                    vk::Offset3D { x: 0, y: 0, z: 0 },
                    vk::Offset3D {
                        x: mip_width,
                        y: mip_height,
                        z: 1,
                    },
                ];
                let dst_subresource = vk::ImageSubresourceLayers::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .mip_level(i)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build();

                let blit = vk::ImageBlit::builder()
                    .src_offsets(src_offsets)
                    .src_subresource(src_subresource)
                    .dst_offsets(dst_offsets)
                    .dst_subresource(dst_subresource)
                    .build();

                self.device
                    .cmd_blit_image(cmd, self.handle, self.handle, blit, vk::Filter::LINEAR);

                self.device.cmd_change_image_layout(
                    cmd,
                    self,
                    ImageLayout::TransferSrc,
                    ImageLayout::ShaderColor,
                    i - 1,
                    1,
                );
            }
            self.device.cmd_change_image_layout(
                cmd,
                self,
                ImageLayout::TransferDst,
                ImageLayout::ShaderColor,
                self.mip_count - 1,
                1,
            );
            Ok(())
        })
    }

    pub(crate) fn get_view(&self, index: usize) -> vk::ImageView {
        self.views[index]
    }

    pub(crate) fn handle(&self) -> vk::Image {
        self.handle
    }

    pub(crate) fn has_depth_format(&self) -> bool {
        self.format == ImageFormat::Depth || self.format == ImageFormat::DepthStencil
    }

    pub(crate) fn layer_count(&self) -> u32 {
        self.layer_count
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
