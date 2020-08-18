// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Texture - simple image that can be used for rendering

use std::rc::Rc;

use super::with_alpha;
use super::ImageFormat;
use super::ImageLayout;
use super::ImageMemory;
use super::ImageMemoryOptions;
use super::ImageMips;
use super::ImageUsage;
use crate::buffer::BufferAccess;
use crate::buffer::BufferMemory;
use crate::buffer::BufferUsage;
use crate::device::Device;
use crate::error::Result;
use crate::pipeline::ImageUniform;
use crate::storage::Index;

// user facing texture data
#[derive(Debug)]
pub struct Texture {
    pub(crate) index: Index,
}

// GPU data storage for a texture
pub(crate) struct CoreTexture {
    _memory: ImageMemory,
    image_index: i32,
}

pub(crate) struct TextureOptions {
    pub(crate) data: Vec<u8>,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) format: ImageFormat,
}

impl Texture {
    pub(crate) fn new(index: Index) -> Self {
        Self { index }
    }
}

impl CoreTexture {
    pub(crate) fn new(
        device: &Rc<Device>,
        uniform: &mut ImageUniform,
        options: TextureOptions,
    ) -> Result<Self> {
        // get byte count based on format
        let pixel_size = match options.format {
            ImageFormat::Srgba | ImageFormat::Rgba | ImageFormat::Srgb | ImageFormat::Rgb => 4,
            ImageFormat::Gray => 1,
            _ => panic!("unsupported texture format {:?}", options.format),
        };

        // convert 3-byte data to 4-byte data
        let data = match options.format {
            ImageFormat::Srgb | ImageFormat::Rgb => with_alpha(options.data),
            _ => options.data,
        };
        let format = match options.format {
            ImageFormat::Srgb => ImageFormat::Srgba,
            ImageFormat::Rgb => ImageFormat::Rgba,
            f => f,
        };

        let size = (options.width * options.height) as usize * pixel_size;

        let staging_memory =
            BufferMemory::new(device, &[BufferUsage::TransferSrc], BufferAccess::Cpu, size)?;
        staging_memory.copy_from_data(&data, size)?;

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
                format,
                ..Default::default()
            },
        )?;

        // copy image from staging memory
        memory.change_layout(ImageLayout::TransferDst)?;
        memory.copy_from_memory(&staging_memory, 0)?;
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
