// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// ImageMemory - struct that manages allocated image memory

use ash::vk;
use std::cmp;
use std::sync::Arc;

use super::ImageFormat;
use super::ImageLayout;
use super::ImageMips;
use super::ImageSamples;
use super::ImageUsage;
use super::LayoutChangeOptions;
use crate::buffer::BufferMemory;
use crate::device::Device;
use crate::error::Result;

pub(crate) struct ImageMemory {
    handle: vk::Image,
    memory: Option<vk::DeviceMemory>,
    view: Option<vk::ImageView>,
    width: u32,
    height: u32,
    mip_count: u32,
    format: ImageFormat,
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
    pub(crate) samples: ImageSamples,
    pub(crate) create_view: bool,
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
                    .array_layers(1)
                    .format(options.format.flag())
                    .tiling(vk::ImageTiling::OPTIMAL)
                    .initial_layout(ImageLayout::Undefined.flag())
                    .usage(ImageUsage::combine(options.usage))
                    .sharing_mode(vk::SharingMode::EXCLUSIVE)
                    .samples(options.samples.flag());

                let (handle, memory) = device.allocate_image(&image_info)?;
                (handle, Some(memory))
            }
        };

        // create view if it is needed
        let view = if !options.create_view {
            None
        } else {
            let aspect_flags = match options.format {
                ImageFormat::Sbgra
                | ImageFormat::Rgb
                | ImageFormat::Rgba
                | ImageFormat::Srgba
                | ImageFormat::Srgb => vk::ImageAspectFlags::COLOR,
                ImageFormat::Depth => vk::ImageAspectFlags::DEPTH,
                ImageFormat::DepthStencil => {
                    vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL
                }
            };

            let subresource = vk::ImageSubresourceRange::builder()
                .aspect_mask(aspect_flags)
                .base_mip_level(0)
                .base_array_layer(0)
                .layer_count(1)
                .level_count(mip_count)
                .build();
            let view_info = vk::ImageViewCreateInfo::builder()
                .image(handle)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(options.format.flag())
                .subresource_range(subresource);

            Some(device.create_image_view(&view_info)?)
        };

        Ok(Self {
            handle,
            width: options.width,
            height: options.height,
            mip_count,
            memory,
            view,
            format: options.format,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn copy_from_memory(&self, memory: &BufferMemory) -> Result<()> {
        let subresource = vk::ImageSubresourceLayers::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .base_array_layer(0)
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

    pub(crate) fn generate_mipmaps(&self) -> Result<()> {
        let mut mip_width = self.width as i32;
        let mut mip_height = self.height as i32;

        self.device.do_commands(|cmd| {
            for i in 1..self.mip_count {
                self.device.cmd_change_image_layout(
                    cmd,
                    self,
                    LayoutChangeOptions {
                        base_mip: i - 1,
                        mip_count: 1,
                        old_layout: ImageLayout::TransferDst,
                        new_layout: ImageLayout::TransferSrc,
                    },
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
                    LayoutChangeOptions {
                        base_mip: i - 1,
                        mip_count: 1,
                        old_layout: ImageLayout::TransferSrc,
                        new_layout: ImageLayout::ShaderColor,
                    },
                );
            }
            self.device.cmd_change_image_layout(
                cmd,
                self,
                LayoutChangeOptions {
                    base_mip: self.mip_count - 1,
                    mip_count: 1,
                    old_layout: ImageLayout::TransferDst,
                    new_layout: ImageLayout::ShaderColor,
                },
            );
            Ok(())
        })
    }

    pub(crate) fn handle(&self) -> vk::Image {
        self.handle
    }

    pub(crate) fn view(&self) -> Option<vk::ImageView> {
        self.view
    }

    pub(crate) fn mip_count(&self) -> u32 {
        self.mip_count
    }

    pub(crate) fn has_depth_format(&self) -> bool {
        self.format == ImageFormat::Depth || self.format == ImageFormat::DepthStencil
    }
}

impl Drop for ImageMemory {
    fn drop(&mut self) {
        if let Some(view) = self.view {
            self.device.destroy_image_view(view);
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
            samples: ImageSamples(1),
            usage: &[],
            create_view: false,
        }
    }
}
