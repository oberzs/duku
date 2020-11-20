// Oliver Berzs
// https://github.com/oberzs/duku

use super::with_alpha;
use super::Format;
use super::Image;
use super::ImageLayout;
use super::Mips;
use super::Size;
use crate::buffer::Buffer;
use crate::device::Device;
use crate::error::Result;
use crate::pipeline::Uniforms;
use crate::renderer::Color;

#[cfg(feature = "jpeg")]
use super::ColorSpace;

/// Image that can be sampled in a shader.
///
/// These can be created from bytes of color data, PNGs
/// JPEGs, etc.
///
/// # Example
///
/// ```ignore
/// let texture = duku.create_texture_png("path/to/image.png", ColorSpace::Srgb, Mips::Log2);
///
/// // use the texture in a material
/// let material = duku.build_material_pbr()?
///     .albedo_texture(&texture)
///     .build();
/// ```
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
    ) -> Result<Self> {
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

        let shader_index = uniforms.add_texture(image.add_view(device))?;

        Ok(Self {
            data: image_data,
            should_update: false,
            image,
            shader_index,
        })
    }

    #[cfg(feature = "jpeg")]
    pub(crate) fn from_jpeg_bytes(
        device: &Device,
        uniforms: &mut Uniforms,
        bytes: &[u8],
        color_space: ColorSpace,
        mips: Mips,
    ) -> Result<Self> {
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

        Self::new(device, uniforms, data, size, format, mips)
    }

    /// Get the width of the texture
    pub const fn width(&self) -> u32 {
        self.image.size().width
    }

    /// Get the height of the texture
    pub const fn height(&self) -> u32 {
        self.image.size().height
    }

    /// Get the data of the texture
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Set a pixel in the image to a specific color.
    ///
    /// Note: works only if the texture has no mips.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let tex = duku.texture_mut(&texture);
    /// tex.set_pixel(0, 0, Color::SKY_BLUE);
    /// tex.set_pixel(1, 1, Color::RED);
    /// ```
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

    /// Get a pixel's color in the image
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
        uniforms.remove_texture(self.shader_index);
        self.image.destroy(device);
    }

    pub(crate) const fn shader_index(&self) -> u32 {
        self.shader_index
    }
}
