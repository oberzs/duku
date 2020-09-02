// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Cubemap - image with 6 layers to render a skybox

use std::rc::Rc;

use super::ImageFormat;
use super::ImageLayout;
use super::ImageMemory;
use super::ImageMemoryOptions;
use super::ImageMips;
use super::ImageUsage;
use crate::buffer::Buffer;
use crate::device::Device;

use crate::vk;

pub(crate) struct Cubemap {
    memory: ImageMemory,
}

pub struct CubemapSides<T> {
    pub top: T,
    pub bottom: T,
    pub front: T,
    pub back: T,
    pub left: T,
    pub right: T,
}

impl Cubemap {
    pub(crate) fn new(
        device: &Rc<Device>,
        size: u32,
        format: ImageFormat,
        sides: CubemapSides<Vec<u8>>,
    ) -> Self {
        // create staging buffers
        let top_staging_buffer = Buffer::staging(device, &sides.top);
        let bottom_staging_buffer = Buffer::staging(device, &sides.bottom);
        let front_staging_buffer = Buffer::staging(device, &sides.front);
        let back_staging_buffer = Buffer::staging(device, &sides.back);
        let left_staging_buffer = Buffer::staging(device, &sides.left);
        let right_staging_buffer = Buffer::staging(device, &sides.right);

        // create image
        let memory = ImageMemory::new(
            device,
            ImageMemoryOptions {
                width: size,
                height: size,
                mips: ImageMips::Log2,
                usage: &[
                    ImageUsage::Sampled,
                    ImageUsage::TransferSrc,
                    ImageUsage::TransferDst,
                ],
                cubemap: true,
                format,
                ..Default::default()
            },
        );

        // copy images from staging memory
        memory.change_layout(ImageLayout::Undefined, ImageLayout::TransferDst);
        memory.copy_from_buffer(&right_staging_buffer, 0);
        memory.copy_from_buffer(&left_staging_buffer, 1);
        memory.copy_from_buffer(&top_staging_buffer, 2);
        memory.copy_from_buffer(&bottom_staging_buffer, 3);
        memory.copy_from_buffer(&front_staging_buffer, 4);
        memory.copy_from_buffer(&back_staging_buffer, 5);
        memory.change_layout(ImageLayout::TransferDst, ImageLayout::ShaderColor);

        Self { memory }
    }

    #[cfg(feature = "png")]
    pub(crate) fn from_png_bytes(
        device: &Rc<Device>,
        sides: CubemapSides<Vec<u8>>,
    ) -> crate::error::Result<Self> {
        use png::ColorType;
        use png::Decoder;

        use crate::error::Error;

        let (format, size) = {
            let decoder = Decoder::new(sides.top.as_slice());
            let (info, _) = decoder.read_info().map_err(|_| Error::InvalidPng)?;

            let f = match info.color_type {
                ColorType::RGBA => ImageFormat::Srgba,
                ColorType::RGB => ImageFormat::Srgb,
                ColorType::Grayscale => ImageFormat::Gray,
                _ => return Err(Error::UnsupportedColorType),
            };
            (f, info.width)
        };

        let mut side_data: Vec<_> = [
            sides.top,
            sides.bottom,
            sides.front,
            sides.back,
            sides.left,
            sides.right,
        ]
        .iter()
        .map(|side| {
            let decoder = Decoder::new(side.as_slice());
            let (info, mut reader) = decoder.read_info().map_err(|_| Error::InvalidPng)?;

            let mut data = vec![0; info.buffer_size()];
            reader.next_frame(&mut data).expect("bad read");
            Ok(data)
        })
        .collect::<crate::error::Result<_>>()?;

        Ok(Self::new(
            device,
            size,
            format,
            CubemapSides {
                top: side_data.remove(0),
                bottom: side_data.remove(0),
                front: side_data.remove(0),
                back: side_data.remove(0),
                left: side_data.remove(0),
                right: side_data.remove(0),
            },
        ))
    }

    pub(crate) fn add_view(&mut self) -> vk::ImageView {
        self.memory.add_view()
    }
}
