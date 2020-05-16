use ash::version::DeviceV1_0;
use ash::vk::Buffer;
use ash::vk::BufferImageCopy;
use ash::vk::DeviceMemory;
use ash::vk::Extent3D;
use ash::vk::Filter;
use ash::vk::Format;
use ash::vk::Image as VkImage;
use ash::vk::ImageAspectFlags;
use ash::vk::ImageBlit;
use ash::vk::ImageCreateInfo;
use ash::vk::ImageLayout as VkImageLayout;
use ash::vk::ImageSubresourceLayers;
use ash::vk::ImageSubresourceRange;
use ash::vk::ImageTiling;
use ash::vk::ImageType;
use ash::vk::ImageUsageFlags;
use ash::vk::ImageView;
use ash::vk::ImageViewCreateInfo;
use ash::vk::ImageViewType;
use ash::vk::MemoryAllocateInfo;
use ash::vk::MemoryPropertyFlags;
use ash::vk::Offset3D;
use ash::vk::SharingMode;
use std::cmp;
use std::sync::Arc;

use crate::error::Result;
use crate::instance::Commands;
use crate::instance::Device;
use crate::instance::Samples;

pub(crate) struct Image {
    width: u32,
    height: u32,
    mip_levels: u32,
    vk: VkImage,
    memory: Option<DeviceMemory>,
    view: Option<ImageView>,
    format: ImageFormat,
    device: Arc<Device>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct ImageOptions<'usage> {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) format: ImageFormat,
    pub(crate) usage: &'usage [ImageUsage],
    pub(crate) image: Option<VkImage>,
    pub(crate) has_view: bool,
    pub(crate) has_mipmaps: bool,
    pub(crate) has_stencil: bool,
    pub(crate) has_samples: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ImageFormat {
    Rgba,
    Bgra,
    Depth,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ImageUsage {
    Depth,
    Color,
    Transient,
    TransferSrc,
    TransferDst,
    Sampled,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ImageLayout {
    Undefined,
    Depth,
    Color,
    Shader,
    Present,
    TransferSrc,
    TransferDst,
}

impl Image {
    pub(crate) fn new(device: &Arc<Device>, options: ImageOptions<'_>) -> Result<Self> {
        let mip_levels = if options.has_mipmaps {
            (cmp::max(options.width, options.height) as f32)
                .log2()
                .floor() as u32
                + 1
        } else {
            1
        };
        let samples = if options.has_samples {
            device.properties().samples
        } else {
            Samples(1)
        };

        let (vk, memory) = match options.image {
            Some(image) => (image, None),
            None => {
                // create image
                let image_info = ImageCreateInfo::builder()
                    .image_type(ImageType::TYPE_2D)
                    .extent(Extent3D {
                        width: options.width,
                        height: options.height,
                        depth: 1,
                    })
                    .mip_levels(mip_levels)
                    .array_layers(1)
                    .format(options.format.flag())
                    .tiling(ImageTiling::OPTIMAL)
                    .initial_layout(ImageLayout::Undefined.flag())
                    .usage(ImageUsage::combine(options.usage))
                    .sharing_mode(SharingMode::EXCLUSIVE)
                    .samples(samples.flag());

                let vk = unsafe { device.logical().create_image(&image_info, None)? };

                // alloc memory
                let mem_requirements =
                    unsafe { device.logical().get_image_memory_requirements(vk) };

                let mem_type = device.pick_memory_type(
                    mem_requirements.memory_type_bits,
                    MemoryPropertyFlags::DEVICE_LOCAL,
                )?;

                let alloc_info = MemoryAllocateInfo::builder()
                    .allocation_size(mem_requirements.size)
                    .memory_type_index(mem_type);

                let memory = unsafe { device.logical().allocate_memory(&alloc_info, None)? };

                // bind memory
                unsafe {
                    device.logical().bind_image_memory(vk, memory, 0)?;
                }

                (vk, Some(memory))
            }
        };

        // create view
        let view = if options.has_view {
            let mut aspect_flags = if options.format == ImageFormat::Depth {
                ImageAspectFlags::DEPTH
            } else {
                ImageAspectFlags::COLOR
            };
            if options.has_stencil {
                aspect_flags |= ImageAspectFlags::STENCIL;
            }

            let subresource = ImageSubresourceRange::builder()
                .aspect_mask(aspect_flags)
                .base_mip_level(0)
                .base_array_layer(0)
                .layer_count(1)
                .level_count(mip_levels)
                .build();
            let view_info = ImageViewCreateInfo::builder()
                .image(vk)
                .view_type(ImageViewType::TYPE_2D)
                .format(options.format.flag())
                .subresource_range(subresource);

            unsafe { Some(device.logical().create_image_view(&view_info, None)?) }
        } else {
            None
        };

        Ok(Self {
            width: options.width,
            height: options.height,
            mip_levels,
            vk,
            memory,
            view,
            format: options.format,
            device: Arc::clone(device),
        })
    }

    pub(crate) fn copy_data_from(&self, src: Buffer) -> Result<()> {
        let subresource = ImageSubresourceLayers::builder()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_array_layer(0)
            .layer_count(1)
            .mip_level(0)
            .build();

        let region = BufferImageCopy::builder()
            .buffer_offset(0)
            .buffer_row_length(0)
            .buffer_image_height(0)
            .image_subresource(subresource)
            .image_offset(Offset3D { x: 0, y: 0, z: 0 })
            .image_extent(Extent3D {
                width: self.width,
                height: self.height,
                depth: 1,
            })
            .build();

        let cmd = Commands::new(&self.device)?;
        cmd.begin_one_time()?;
        cmd.copy_buffer_to_image(src, self.vk, region);
        self.device.submit_and_wait(cmd.end()?)?;
        Ok(())
    }

    pub(crate) fn generate_mipmaps(&self) -> Result<()> {
        let mut mip_width = self.width as i32;
        let mut mip_height = self.height as i32;

        let cmd = Commands::new(&self.device)?;
        cmd.begin_one_time()?;

        for i in 1..self.mip_levels {
            cmd.change_image_layout(self)
                .with_mips(i - 1, 1)
                .change_from_write()
                .change_to_read()
                .record();

            let src_offsets = [
                Offset3D { x: 0, y: 0, z: 0 },
                Offset3D {
                    x: mip_width,
                    y: mip_height,
                    z: 1,
                },
            ];
            let src_subresource = ImageSubresourceLayers::builder()
                .aspect_mask(ImageAspectFlags::COLOR)
                .mip_level(i - 1)
                .base_array_layer(0)
                .layer_count(1)
                .build();

            mip_width = cmp::max(mip_width / 2, 1);
            mip_height = cmp::max(mip_height / 2, 1);
            let dst_offsets = [
                Offset3D { x: 0, y: 0, z: 0 },
                Offset3D {
                    x: mip_width,
                    y: mip_height,
                    z: 1,
                },
            ];
            let dst_subresource = ImageSubresourceLayers::builder()
                .aspect_mask(ImageAspectFlags::COLOR)
                .mip_level(i)
                .base_array_layer(0)
                .layer_count(1)
                .build();

            let blit = ImageBlit::builder()
                .src_offsets(src_offsets)
                .src_subresource(src_subresource)
                .dst_offsets(dst_offsets)
                .dst_subresource(dst_subresource)
                .build();

            cmd.blit_image(self.vk, self.vk, blit, Filter::LINEAR);

            cmd.change_image_layout(self)
                .with_mips(i - 1, 1)
                .change_from_read()
                .change_to_shader_read()
                .record();
        }

        cmd.change_image_layout(self)
            .with_mips(self.mip_levels - 1, 1)
            .change_from_write()
            .change_to_shader_read()
            .record();

        self.device.submit_and_wait(cmd.end()?)?;
        Ok(())
    }

    pub(crate) fn vk(&self) -> VkImage {
        self.vk
    }

    pub(crate) fn view(&self) -> Option<ImageView> {
        self.view
    }

    pub(crate) fn is_depth_format(&self) -> bool {
        self.format == ImageFormat::Depth
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            if let Some(view) = self.view {
                self.device.logical().destroy_image_view(view, None);
            }
            if let Some(memory) = self.memory {
                self.device.logical().destroy_image(self.vk, None);
                self.device.logical().free_memory(memory, None);
            }
        }
    }
}

impl Default for ImageOptions<'_> {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            format: ImageFormat::Rgba,
            usage: &[],
            image: None,
            has_view: false,
            has_mipmaps: false,
            has_stencil: false,
            has_samples: false,
        }
    }
}

impl ImageUsage {
    pub(crate) fn combine(usages: &[Self]) -> ImageUsageFlags {
        usages
            .iter()
            .fold(ImageUsageFlags::empty(), |acc, usage| acc | usage.flag())
    }

    pub(crate) fn flag(&self) -> ImageUsageFlags {
        match *self {
            Self::Color => ImageUsageFlags::COLOR_ATTACHMENT,
            Self::Depth => ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            Self::Transient => ImageUsageFlags::TRANSIENT_ATTACHMENT,
            Self::TransferSrc => ImageUsageFlags::TRANSFER_SRC,
            Self::TransferDst => ImageUsageFlags::TRANSFER_DST,
            Self::Sampled => ImageUsageFlags::SAMPLED,
        }
    }
}

impl ImageFormat {
    pub(crate) fn flag(&self) -> Format {
        match *self {
            Self::Rgba => Format::R8G8B8A8_UNORM,
            Self::Bgra => Format::B8G8R8A8_UNORM,
            Self::Depth => Format::D32_SFLOAT_S8_UINT,
        }
    }
}

impl ImageLayout {
    pub(crate) fn flag(&self) -> VkImageLayout {
        match *self {
            Self::Undefined => VkImageLayout::UNDEFINED,
            Self::Color => VkImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            Self::Depth => VkImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            Self::Shader => VkImageLayout::SHADER_READ_ONLY_OPTIMAL,
            Self::Present => VkImageLayout::PRESENT_SRC_KHR,
            Self::TransferSrc => VkImageLayout::TRANSFER_SRC_OPTIMAL,
            Self::TransferDst => VkImageLayout::TRANSFER_DST_OPTIMAL,
        }
    }
}
