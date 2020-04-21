use ash::version::DeviceV1_0;
use ash::vk::Buffer;
use ash::vk::BufferImageCopy;
use ash::vk::DeviceMemory;
use ash::vk::Extent3D;
use ash::vk::Format;
use ash::vk::Image as VkImage;
use ash::vk::ImageAspectFlags;
use ash::vk::ImageBlit;
use ash::vk::ImageCreateInfo;
use ash::vk::ImageLayout;
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
use ash::vk::SampleCountFlags;
use ash::vk::SharingMode;
use std::cmp;
use std::rc::Rc;

use super::LayoutChange;
use crate::tegne::CommandRecorder;
use crate::tegne::Device;
use crate::utils::OrError;

pub struct Image {
    width: u32,
    height: u32,
    mip_levels: u32,
    vk: VkImage,
    memory: Option<DeviceMemory>,
    view: Option<ImageView>,
    device: Rc<Device>,
    _format: Format,
}

pub struct ImageBuilder {
    device: Rc<Device>,
    width: u32,
    height: u32,
    samples: SampleCountFlags,
    format: Format,
    usage: ImageUsageFlags,
    view: bool,
    mipmaps: bool,
    image: Option<VkImage>,
}

impl Image {
    pub fn builder(device: &Rc<Device>) -> ImageBuilder {
        ImageBuilder {
            device: Rc::clone(device),
            width: 1,
            height: 1,
            samples: SampleCountFlags::TYPE_1,
            format: Format::R8G8B8A8_UNORM,
            usage: ImageUsageFlags::empty(),
            view: false,
            mipmaps: false,
            image: None,
        }
    }

    pub fn copy_data_from(&self, src: Buffer) {
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

        let recorder = CommandRecorder::new(&self.device);
        recorder.begin_one_time();
        recorder.copy_buffer_to_image(src, self.vk, region);
        self.device.submit_buffer(recorder.end());
    }

    pub fn generate_mipmaps(&self) {
        let mut mip_width = self.width as i32;
        let mut mip_height = self.height as i32;

        let recorder = CommandRecorder::new(&self.device);
        recorder.begin_one_time();

        for i in 1..self.mip_levels {
            LayoutChange::new(&recorder, self)
                .with_mips(i - 1, 1)
                .from_write()
                .to_read()
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

            recorder.blit_image(self.vk, self.vk, blit);

            LayoutChange::new(&recorder, self)
                .with_mips(i - 1, 1)
                .from_read()
                .to_shader_read()
                .record();
        }

        LayoutChange::new(&recorder, self)
            .with_mips(self.mip_levels - 1, 1)
            .from_write()
            .to_shader_read()
            .record();

        self.device.submit_buffer(recorder.end());
    }

    pub fn vk(&self) -> VkImage {
        self.vk
    }

    pub fn view(&self) -> ImageView {
        self.view.or_error("image does not have a view")
    }
}

impl ImageBuilder {
    pub fn with_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_mipmaps(&mut self) -> &mut Self {
        self.mipmaps = true;
        self
    }

    pub fn with_samples(&mut self) -> &mut Self {
        self.samples = self.device.pick_sample_count();
        self
    }

    pub fn with_bgra_color(&mut self) -> &mut Self {
        self.format = self.device.pick_bgra_format();
        self
    }

    pub fn with_rgba_color(&mut self) -> &mut Self {
        self.format = self.device.pick_rgba_format();
        self
    }

    pub fn with_depth(&mut self) -> &mut Self {
        self.format = self.device.pick_depth_format();
        self
    }

    pub fn with_usage(&mut self, usage: ImageUsageFlags) -> &mut Self {
        self.usage |= usage;
        self
    }

    pub fn with_view(&mut self) -> &mut Self {
        self.view = true;
        self
    }

    pub fn from_image(&mut self, image: VkImage) -> &mut Self {
        self.image = Some(image);
        self
    }

    pub fn build(&self) -> Image {
        let mip_levels = if self.mipmaps {
            (cmp::max(self.width, self.height) as f32).log2().floor() as u32 + 1
        } else {
            1
        };

        let (vk, memory) = match self.image {
            Some(image) => (image, None),
            None => {
                // create image
                let image_info = ImageCreateInfo::builder()
                    .image_type(ImageType::TYPE_2D)
                    .extent(Extent3D {
                        width: self.width,
                        height: self.height,
                        depth: 1,
                    })
                    .mip_levels(mip_levels)
                    .array_layers(1)
                    .format(self.format)
                    .tiling(ImageTiling::OPTIMAL)
                    .initial_layout(ImageLayout::UNDEFINED)
                    .usage(self.usage)
                    .sharing_mode(SharingMode::EXCLUSIVE)
                    .samples(self.samples);

                let vk = unsafe {
                    self.device
                        .logical()
                        .create_image(&image_info, None)
                        .or_error("cannot create texture image")
                };

                // alloc memory
                let mem_requirements =
                    unsafe { self.device.logical().get_image_memory_requirements(vk) };

                let mem_type = self.device.pick_memory_type(
                    mem_requirements.memory_type_bits,
                    MemoryPropertyFlags::DEVICE_LOCAL,
                );

                let alloc_info = MemoryAllocateInfo::builder()
                    .allocation_size(mem_requirements.size)
                    .memory_type_index(mem_type);

                let memory = unsafe {
                    self.device
                        .logical()
                        .allocate_memory(&alloc_info, None)
                        .or_error("cannot allocate texture memory")
                };

                // bind memory
                unsafe {
                    self.device
                        .logical()
                        .bind_image_memory(vk, memory, 0)
                        .or_error("cannot bind texture memory");
                }

                (vk, Some(memory))
            }
        };

        // create view
        let view = match self.view {
            true => {
                let aspect_flags = match self.format {
                    Format::D32_SFLOAT_S8_UINT => {
                        ImageAspectFlags::DEPTH | ImageAspectFlags::STENCIL
                    }
                    _ => ImageAspectFlags::COLOR,
                };
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
                    .format(self.format)
                    .subresource_range(subresource);

                unsafe {
                    Some(
                        self.device
                            .logical()
                            .create_image_view(&view_info, None)
                            .or_error("cannot create image view"),
                    )
                }
            }
            _ => None,
        };

        Image {
            width: self.width,
            height: self.height,
            mip_levels,
            vk,
            memory,
            view,
            device: Rc::clone(&self.device),
            _format: self.format,
        }
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
