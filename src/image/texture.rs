// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Texture - simple image that can be used for rendering

use super::Image;
use super::ImageFormat;
use super::ImageLayout;
use super::Size;
use crate::buffer::Buffer;
use crate::device::Device;
use crate::pipeline::ShaderImages;

pub struct Texture {
    image: Image,
    shader_index: u32,
}

impl Texture {
    pub(crate) fn new(
        device: &Device,
        shader_images: &mut ShaderImages,
        data: Vec<u8>,
        size: Size,
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
        let mut image = Image::texture(device, format, size, false);

        // copy image from staging buffer
        image.change_layout(device, ImageLayout::Undefined, ImageLayout::TransferDst);
        image.copy_from_buffer(device, &staging_buffer, 0);
        image.generate_mipmaps(device);

        // destroy staging buffer
        staging_buffer.destroy(device);

        let shader_index = shader_images.add(image.add_view(device));

        Self {
            image,
            shader_index,
        }
    }

    #[cfg(feature = "png")]
    pub(crate) fn from_png_bytes(
        device: &Device,
        shader_images: &mut ShaderImages,
        bytes: Vec<u8>,
    ) -> crate::error::Result<Self> {
        use png::ColorType;
        use png::Decoder;

        use crate::error::Error;

        let decoder = Decoder::new(bytes.as_slice());
        let (info, mut reader) = decoder.read_info().map_err(|_| Error::InvalidPng)?;
        let size = Size::new(info.width, info.height);

        let mut data = vec![0; info.buffer_size()];
        reader.next_frame(&mut data).expect("bad read");

        let format = match info.color_type {
            ColorType::RGBA => ImageFormat::Srgba,
            ColorType::RGB => ImageFormat::Srgb,
            ColorType::Grayscale => ImageFormat::Gray,
            _ => return Err(Error::UnsupportedColorType),
        };

        Ok(Self::new(device, shader_images, data, size, format))
    }

    pub(crate) fn destroy(&self, device: &Device) {
        self.image.destroy(device);
    }

    pub(crate) const fn shader_index(&self) -> u32 {
        self.shader_index
    }
}

pub(crate) fn with_alpha(data: Vec<u8>) -> Vec<u8> {
    let mut new_data = Vec::with_capacity(4 * data.len() / 3);
    for pixel in data.chunks(3) {
        new_data.extend(&[pixel[0], pixel[1], pixel[2], 255]);
    }
    new_data
}
