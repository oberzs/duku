// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Texture - simple image that can be used for rendering

use std::sync::Arc;

use super::ImageFormat;
use super::ImageLayout;
use super::ImageMemory;
use super::ImageMemoryOptions;
use super::ImageMips;
use super::ImageUsage;
use super::LayoutChangeOptions;
use crate::buffer::BufferAccess;
use crate::buffer::BufferMemory;
use crate::buffer::BufferUsage;
use crate::device::Device;
use crate::error::Result;
use crate::pipeline::ImageUniform;

pub struct Texture {
    _memory: ImageMemory,
    image_index: i32,
}

pub(crate) struct TextureOptions<'data> {
    pub data: &'data [u8],
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
}

impl Texture {
    pub(crate) fn new(
        device: &Arc<Device>,
        uniform: &ImageUniform,
        options: TextureOptions<'_>,
    ) -> Result<Self> {
        let pixel_size = match options.format {
            ImageFormat::Srgba | ImageFormat::Rgba => 4,
            _ => panic!("unsupported texture format {:?}", options.format),
        };

        let size = (options.width * options.height) as usize * pixel_size;

        let staging_memory =
            BufferMemory::new(device, &[BufferUsage::TransferSrc], BufferAccess::Cpu, size)?;
        staging_memory.copy_from_data(options.data, size)?;

        let mut memory = ImageMemory::new(
            device,
            ImageMemoryOptions {
                width: options.width,
                height: options.height,
                mips: ImageMips::Log2,
                usage: &[
                    ImageUsage::Sampled,
                    ImageUsage::TransferSrc,
                    ImageUsage::TransferDst,
                ],
                format: options.format,
                ..Default::default()
            },
        )?;

        // prepare image for data copy
        device.do_commands(|cmd| {
            device.cmd_change_image_layout(
                cmd,
                &memory,
                LayoutChangeOptions {
                    base_mip: 0,
                    mip_count: memory.mip_count(),
                    new_layout: ImageLayout::TransferDst,
                    ..Default::default()
                },
            );
            Ok(())
        })?;

        memory.copy_from_memory(&staging_memory)?;
        memory.generate_mipmaps()?;

        let image_index = uniform.add(memory.add_view()?);

        Ok(Self {
            _memory: memory,
            image_index,
        })
    }

    pub(crate) fn image_index(&self) -> i32 {
        self.image_index
    }
}

impl Default for TextureOptions<'_> {
    fn default() -> Self {
        Self {
            data: &[255, 255, 255, 255],
            width: 1,
            height: 1,
            format: ImageFormat::Rgba,
        }
    }
}
