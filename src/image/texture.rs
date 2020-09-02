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
use crate::buffer::Buffer;
use crate::device::Device;
use crate::pipeline::ImageUniform;
use crate::storage::Index;

// user facing texture data
#[derive(Debug)]
pub struct Texture {
    pub(crate) index: Index,
    pub(crate) image_index: i32,
}

// GPU data storage for a texture
pub(crate) struct CoreTexture {
    _memory: ImageMemory,
    image_index: i32,
}

impl Texture {
    pub(crate) const fn new(index: Index, image_index: i32) -> Self {
        Self { index, image_index }
    }

    pub const fn index(&self) -> i32 {
        self.image_index
    }
}

impl CoreTexture {
    pub(crate) fn new(
        device: &Rc<Device>,
        uniform: &mut ImageUniform,
        data: Vec<u8>,
        width: u32,
        height: u32,
        format: ImageFormat,
    ) -> Self {
        // convert 3-byte data to 4-byte data
        let image_data = match format {
            ImageFormat::Srgb | ImageFormat::Rgb => with_alpha(data),
            _ => data,
        };
        let format = match format {
            ImageFormat::Srgb => ImageFormat::Srgba,
            ImageFormat::Rgb => ImageFormat::Rgba,
            f => f,
        };

        let staging_buffer = Buffer::staging(device, &image_data);

        let mut memory = ImageMemory::new(
            device,
            ImageMemoryOptions {
                mips: ImageMips::Log2,
                usage: &[
                    ImageUsage::Sampled,
                    ImageUsage::TransferSrc,
                    ImageUsage::TransferDst,
                ],
                width,
                height,
                format,
                ..Default::default()
            },
        );

        // copy image from staging memory
        memory.change_layout(ImageLayout::Undefined, ImageLayout::TransferDst);
        memory.copy_from_buffer(&staging_buffer, 0);
        memory.generate_mipmaps();

        let image_index = uniform.add(memory.add_view());

        Self {
            _memory: memory,
            image_index,
        }
    }

    #[cfg(feature = "png")]
    pub(crate) fn from_png_bytes(
        device: &Rc<Device>,
        uniform: &mut ImageUniform,
        bytes: Vec<u8>,
    ) -> crate::error::Result<Self> {
        use png::ColorType;
        use png::Decoder;

        use crate::error::Error;

        let decoder = Decoder::new(bytes.as_slice());
        let (info, mut reader) = decoder.read_info().map_err(|_| Error::InvalidPng)?;

        let mut data = vec![0; info.buffer_size()];
        reader.next_frame(&mut data).expect("bad read");

        let format = match info.color_type {
            ColorType::RGBA => ImageFormat::Srgba,
            ColorType::RGB => ImageFormat::Srgb,
            ColorType::Grayscale => ImageFormat::Gray,
            _ => return Err(Error::UnsupportedColorType),
        };

        Ok(Self::new(
            device,
            uniform,
            data,
            info.width,
            info.height,
            format,
        ))
    }

    pub(crate) const fn image_index(&self) -> i32 {
        self.image_index
    }
}
