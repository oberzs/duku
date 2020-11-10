// Oliver Berzs
// https://github.com/oberzs/duku

// Cubemap - image with 6 layers to render a skybox

use super::with_alpha;
use super::Format;
use super::Image;
use super::ImageLayout;
use super::Size;
use crate::buffer::Buffer;
use crate::device::Device;
use crate::pipeline::Uniforms;

pub struct Cubemap {
    image: Image,
    shader_index: u32,
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
        device: &Device,
        uniforms: &mut Uniforms,
        size: u32,
        format: Format,
        sides: CubemapSides<Vec<u8>>,
    ) -> Self {
        // convert 3-byte data to 4-byte data
        let sides = if matches!(format, Format::Srgb | Format::Rgb) {
            CubemapSides {
                top: with_alpha(sides.top),
                bottom: with_alpha(sides.bottom),
                front: with_alpha(sides.front),
                back: with_alpha(sides.back),
                left: with_alpha(sides.left),
                right: with_alpha(sides.right),
            }
        } else {
            sides
        };
        let format = match format {
            Format::Srgb => Format::Srgba,
            Format::Rgb => Format::Rgba,
            f => f,
        };

        // create staging buffers
        let top_staging_buffer = Buffer::staging(device, &sides.top);
        let bottom_staging_buffer = Buffer::staging(device, &sides.bottom);
        let front_staging_buffer = Buffer::staging(device, &sides.front);
        let back_staging_buffer = Buffer::staging(device, &sides.back);
        let left_staging_buffer = Buffer::staging(device, &sides.left);
        let right_staging_buffer = Buffer::staging(device, &sides.right);

        // create image
        let mut image = Image::cubemap(device, format, Size::new(size, size));

        // copy images from staging buffer
        image.change_layout(device, ImageLayout::Undefined, ImageLayout::TransferDst);
        image.copy_from_buffer(device, &right_staging_buffer, 0);
        image.copy_from_buffer(device, &left_staging_buffer, 1);
        image.copy_from_buffer(device, &top_staging_buffer, 2);
        image.copy_from_buffer(device, &bottom_staging_buffer, 3);
        image.copy_from_buffer(device, &front_staging_buffer, 4);
        image.copy_from_buffer(device, &back_staging_buffer, 5);
        image.change_layout(device, ImageLayout::TransferDst, ImageLayout::ShaderColor);

        // destroy staging buffers
        top_staging_buffer.destroy(device);
        bottom_staging_buffer.destroy(device);
        front_staging_buffer.destroy(device);
        back_staging_buffer.destroy(device);
        left_staging_buffer.destroy(device);
        right_staging_buffer.destroy(device);

        let shader_index = uniforms.add_cubemap(image.add_view(device));

        Self {
            image,
            shader_index,
        }
    }

    #[cfg(feature = "png")]
    pub(crate) fn from_png_bytes(
        device: &Device,
        uniforms: &mut Uniforms,
        sides: CubemapSides<Vec<u8>>,
    ) -> crate::error::Result<Self> {
        use png::ColorType;
        use png::Decoder;

        use crate::error::Error;

        let (format, size) = {
            let decoder = Decoder::new(sides.top.as_slice());
            let (info, _) = decoder.read_info().map_err(|_| Error::InvalidPng)?;

            let f = match info.color_type {
                ColorType::RGBA => Format::Srgba,
                ColorType::RGB => Format::Srgb,
                ColorType::Grayscale => Format::Gray,
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
            uniforms,
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

    pub(crate) fn destroy(&self, device: &Device, uniforms: &mut Uniforms) {
        uniforms.remove_cubemap(self.shader_index);
        self.image.destroy(device);
    }

    pub(crate) const fn shader_index(&self) -> u32 {
        self.shader_index
    }
}
