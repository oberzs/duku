use ash::version::DeviceV1_0;
use ash::vk::BufferUsageFlags;
use ash::vk::MemoryPropertyFlags;
use std::cmp;
use std::sync::Arc;

use super::Image;
use super::ImageFormat;
use super::ImageOptions;
use super::ImageUsage;
use crate::error::Result;
use crate::instance::Commands;
use crate::instance::Device;
use crate::memory::alloc;
use crate::memory::copy;
use crate::shaders::ImageUniforms;

pub struct Texture {
    _image: Image,
    image_index: i32,
}

impl Texture {
    pub(crate) fn from_raw_rgb(
        device: &Arc<Device>,
        data: &[u8],
        width: u32,
        height: u32,
        image_uniforms: &ImageUniforms,
    ) -> Result<Self> {
        let mut rgba = vec![];
        rgba.reserve(data.len() + data.len() / 3);
        for c in data.chunks(3) {
            rgba.extend(c.iter());
            rgba.push(255);
        }
        Self::from_raw_rgba(device, &rgba, width, height, image_uniforms)
    }

    pub(crate) fn from_raw_rgba(
        device: &Arc<Device>,
        data: &[u8],
        width: u32,
        height: u32,
        image_uniforms: &ImageUniforms,
    ) -> Result<Self> {
        let mip_levels = (cmp::max(width, height) as f32).log2().floor() as u32 + 1;

        let size = width * height * 4;

        let (staging_buffer, staging_memory) = alloc::buffer(
            device,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            size as usize,
        )?;

        copy::data_to_buffer(&device, data, staging_memory, size as usize);

        let image = Image::new(
            device,
            ImageOptions {
                width,
                height,
                format: ImageFormat::Rgba,
                usage: &[
                    ImageUsage::Sampled,
                    ImageUsage::TransferSrc,
                    ImageUsage::TransferDst,
                ],
                has_view: true,
                has_mipmaps: true,
                ..Default::default()
            },
        )?;

        let cmd = Commands::new(device);
        cmd.begin_one_time()?;
        cmd.change_image_layout(&image)
            .with_mips(0, mip_levels)
            .change_to_write()
            .record()?;
        device.submit_buffer(cmd.end()?)?;

        image.copy_data_from(staging_buffer)?;
        image.generate_mipmaps()?;

        unsafe {
            device.logical().destroy_buffer(staging_buffer, None);
            device.logical().free_memory(staging_memory, None);
        }

        let image_index = image_uniforms.image_count() as i32;
        image_uniforms.add(image.view());

        Ok(Self {
            _image: image,
            image_index,
        })
    }

    pub(crate) fn image_index(&self) -> i32 {
        self.image_index
    }
}
