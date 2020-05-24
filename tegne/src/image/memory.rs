// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// ImageMemory - struct that manages allocated image memory

use ash::version::DeviceV1_0;
use ash::vk;
use log::error;
use std::cmp;
use std::sync::Arc;

use super::ImageFormat;
use super::ImageLayout;
use super::ImageMips;
use super::ImageSamples;
use super::ImageUsage;
use crate::buffer::BufferMemory;
use crate::error::Result;
use crate::instance::Commands;
use crate::instance::Device;
use crate::instance::LayoutChangeOptions;

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

                let handle = unsafe { device.logical().create_image(&image_info, None)? };

                // alloc memory
                let mem_requirements =
                    unsafe { device.logical().get_image_memory_requirements(handle) };

                let mem_type = device
                    .find_memory_type(
                        mem_requirements.memory_type_bits,
                        vk::MemoryPropertyFlags::DEVICE_LOCAL,
                    )
                    .unwrap_or_else(|| {
                        panic!(error!("device does not support device local image memory"));
                    });

                let alloc_info = vk::MemoryAllocateInfo::builder()
                    .allocation_size(mem_requirements.size)
                    .memory_type_index(mem_type);

                let memory = unsafe { device.logical().allocate_memory(&alloc_info, None)? };

                // bind memory
                unsafe {
                    device.logical().bind_image_memory(handle, memory, 0)?;
                }

                (handle, Some(memory))
            }
        };

        // create view if it is needed
        let view = if !options.create_view {
            None
        } else {
            let aspect_flags = match options.format {
                ImageFormat::Bgra => vk::ImageAspectFlags::COLOR,
                ImageFormat::Rgba => vk::ImageAspectFlags::COLOR,
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

            unsafe { Some(device.logical().create_image_view(&view_info, None)?) }
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

        let cmd = Commands::new(&self.device)?;
        cmd.begin()?;
        cmd.copy_buffer_to_image(memory.handle(), self.handle, region);
        self.device.submit_and_wait(cmd.end()?)?;
        Ok(())
    }

    pub(crate) fn generate_mipmaps(&self) -> Result<()> {
        let mut mip_width = self.width as i32;
        let mut mip_height = self.height as i32;

        let cmd = Commands::new(&self.device)?;
        cmd.begin()?;

        for i in 1..self.mip_count {
            cmd.change_image_layout(
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

            cmd.blit_image(self.handle, self.handle, blit, vk::Filter::LINEAR);

            cmd.change_image_layout(
                self,
                LayoutChangeOptions {
                    base_mip: i - 1,
                    mip_count: 1,
                    old_layout: ImageLayout::TransferSrc,
                    new_layout: ImageLayout::Shader,
                },
            );
        }

        cmd.change_image_layout(
            self,
            LayoutChangeOptions {
                base_mip: self.mip_count - 1,
                mip_count: 1,
                old_layout: ImageLayout::TransferDst,
                new_layout: ImageLayout::Shader,
            },
        );

        self.device.submit_and_wait(cmd.end()?)?;
        Ok(())
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
        unsafe {
            if let Some(view) = self.view {
                self.device.logical().destroy_image_view(view, None);
            }
            if let Some(memory) = self.memory {
                self.device.logical().destroy_image(self.handle, None);
                self.device.logical().free_memory(memory, None);
            }
        }
    }
}

impl Default for ImageMemoryOptions<'_> {
    fn default() -> Self {
        Self {
            handle: None,
            width: 1,
            height: 1,
            format: ImageFormat::Rgba,
            mips: ImageMips::One,
            samples: ImageSamples(1),
            usage: &[],
            create_view: false,
        }
    }
}
