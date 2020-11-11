// Oliver Berzs
// https://github.com/oberzs/duku

// Texture - simple image that can be used for rendering

use super::with_alpha;
use super::Format;
use super::Image;
use super::ImageLayout;
use super::Mips;
use super::Size;
use crate::buffer::Buffer;
use crate::color::Color;
use crate::device::Device;
use crate::pipeline::Uniforms;

#[cfg(any(feature = "png", feature = "jpeg"))]
use super::ColorSpace;

pub struct Texture {
    image: Image,
    data: Vec<u8>,
    shader_index: u32,
    should_update: bool,
}

impl Texture {
    pub(crate) fn new(
        device: &Device,
        uniforms: &mut Uniforms,
        data: Vec<u8>,
        size: Size,
        format: Format,
        mips: Mips,
    ) -> Self {
        // convert 3-byte data to 4-byte data
        let image_data = match format {
            Format::Srgb | Format::Rgb => with_alpha(data),
            _ => data,
        };
        let format = match format {
            Format::Srgb => Format::Srgba,
            Format::Rgb => Format::Rgba,
            f => f,
        };

        let staging_buffer = Buffer::staging(device, &image_data);
        let mut image = Image::texture(device, format, mips, size);

        // copy image from staging buffer
        image.change_layout(device, ImageLayout::Undefined, ImageLayout::TransferDst);
        image.copy_from_buffer(device, &staging_buffer, 0);
        match mips {
            Mips::Log2 => image.generate_mipmaps(device),
            Mips::Zero => {
                image.change_layout(device, ImageLayout::TransferDst, ImageLayout::ShaderColor)
            }
        }

        // destroy staging buffer
        staging_buffer.destroy(device);

        let shader_index = uniforms.add_image(image.add_view(device));

        Self {
            data: image_data,
            should_update: false,
            image,
            shader_index,
        }
    }

    #[cfg(feature = "png")]
    pub(crate) fn from_png_bytes(
        device: &Device,
        uniforms: &mut Uniforms,
        bytes: &[u8],
        color_space: ColorSpace,
        mips: Mips,
    ) -> crate::error::Result<Self> {
        use png::ColorType;
        use png::Decoder;

        use crate::error::Error;

        let decoder = Decoder::new(bytes);
        let (info, mut reader) = decoder.read_info().map_err(|_| Error::InvalidPng)?;
        let size = Size::new(info.width, info.height);

        let mut data = vec![0; info.buffer_size()];
        reader.next_frame(&mut data).expect("bad read");

        let format = match info.color_type {
            ColorType::RGBA if color_space == ColorSpace::Linear => Format::Rgba,
            ColorType::RGBA => Format::Srgba,
            ColorType::RGB if color_space == ColorSpace::Linear => Format::Rgb,
            ColorType::RGB => Format::Srgb,
            ColorType::Grayscale => Format::Gray,
            _ => return Err(Error::UnsupportedFormat),
        };

        Ok(Self::new(device, uniforms, data, size, format, mips))
    }

    #[cfg(feature = "jpeg")]
    pub(crate) fn from_jpeg_bytes(
        device: &Device,
        uniforms: &mut Uniforms,
        bytes: &[u8],
        color_space: ColorSpace,
        mips: Mips,
    ) -> crate::error::Result<Self> {
        use jpeg_decoder::Decoder;
        use jpeg_decoder::PixelFormat;

        use crate::error::Error;

        let mut decoder = Decoder::new(bytes);
        let data = decoder.decode().map_err(|_| Error::InvalidJpeg)?;
        let info = decoder.info().ok_or(Error::InvalidJpeg)?;
        let size = Size::new(u32::from(info.width), u32::from(info.height));

        let format = match info.pixel_format {
            PixelFormat::RGB24 if color_space == ColorSpace::Linear => Format::Rgb,
            PixelFormat::RGB24 => Format::Srgb,
            _ => return Err(Error::UnsupportedFormat),
        };

        Ok(Self::new(device, uniforms, data, size, format, mips))
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        debug_assert!(matches!(self.image.format(), Format::Rgba | Format::Srgba));

        let size = self.image.size();
        if x < size.width && y < size.height {
            let i = (x + y * size.width) as usize * 4;
            self.data[i] = color.r;
            self.data[i + 1] = color.g;
            self.data[i + 2] = color.b;
            self.data[i + 3] = color.a;
            self.should_update = true;
        }
    }

    pub fn pixel(&self, x: u32, y: u32) -> Color {
        debug_assert!(matches!(self.image.format(), Format::Rgba | Format::Srgba));

        let size = self.image.size();
        if x < size.width && y < size.height {
            let i = (x + y * size.width) as usize * 4;
            Color::rgba(
                self.data[i],
                self.data[i + 1],
                self.data[i + 2],
                self.data[i + 3],
            )
        } else {
            Color::BLACK
        }
    }

    pub(crate) fn update_if_needed(&mut self, device: &Device) {
        if self.should_update {
            let staging_buffer = Buffer::staging(device, &self.data);

            self.image
                .change_layout(device, ImageLayout::ShaderColor, ImageLayout::TransferDst);
            self.image.copy_from_buffer(device, &staging_buffer, 0);

            if self.image.mip_count() > 1 {
                self.image.generate_mipmaps(device);
            } else {
                self.image.change_layout(
                    device,
                    ImageLayout::TransferDst,
                    ImageLayout::ShaderColor,
                );
            }

            staging_buffer.destroy(device);

            self.should_update = false;
        }
    }

    pub(crate) fn destroy(&self, device: &Device, uniforms: &mut Uniforms) {
        uniforms.remove_image(self.shader_index);
        self.image.destroy(device);
    }

    pub(crate) const fn shader_index(&self) -> u32 {
        self.shader_index
    }
}
