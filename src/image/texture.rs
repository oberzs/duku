// Oliver Berzs
// https://github.com/oberzs/duku

use super::with_alpha;
use super::Format;
use super::Image;
use super::ImageLayout;
use super::Mips;
use crate::buffer::Buffer;
use crate::device::Device;
use crate::error::Result;
use crate::pipeline::Uniforms;
use crate::renderer::Color;

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
/// let mut material = duku.create_material_pbr()?;
/// material.albedo_texture(&texture);
/// ```
pub struct Texture {
    /// pixel data as bytes
    pub data: Vec<u8>,
    opaque: bool,

    image: Image,
    shader_index: u32,
}

impl Texture {
    pub(crate) fn new(
        device: &Device,
        uniforms: &mut Uniforms,
        data: Vec<u8>,
        width: u32,
        height: u32,
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

        // check if opaque
        let opaque = !matches!(format, Format::Srgba | Format::Rgba)
            || image_data.iter().skip(3).step_by(4).all(|b| *b == 255);

        let staging_buffer = Buffer::staging(device, &image_data);
        let mut image = Image::texture(device, format, mips, width, height);

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
            opaque,
            image,
            shader_index,
        })
    }

    /// Get the width of the texture
    pub const fn width(&self) -> u32 {
        self.image.width()
    }

    /// Get the height of the texture
    pub const fn height(&self) -> u32 {
        self.image.height()
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

        let width = self.image.width();
        let height = self.image.height();
        if x < width && y < height {
            let i = (x + y * width) as usize * 4;
            self.data[i] = color.r;
            self.data[i + 1] = color.g;
            self.data[i + 2] = color.b;
            self.data[i + 3] = color.a;

            if color.a < 255 {
                self.opaque = false;
            }
        }
    }

    /// Get a pixel's color in the image
    pub fn pixel(&self, x: u32, y: u32) -> Color {
        debug_assert!(matches!(self.image.format(), Format::Rgba | Format::Srgba));

        let width = self.image.width();
        let height = self.image.height();
        if x < width && y < height {
            let i = (x + y * width) as usize * 4;
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

    pub(crate) fn update(&mut self, device: &Device) {
        let staging_buffer = Buffer::staging(device, &self.data);

        self.image
            .change_layout(device, ImageLayout::ShaderColor, ImageLayout::TransferDst);
        self.image.copy_from_buffer(device, &staging_buffer, 0);

        if self.image.mip_count() > 1 {
            self.image.generate_mipmaps(device);
        } else {
            self.image
                .change_layout(device, ImageLayout::TransferDst, ImageLayout::ShaderColor);
        }

        staging_buffer.destroy(device);
    }

    pub(crate) const fn opaque(&self) -> bool {
        self.opaque
    }

    pub(crate) fn destroy(&self, device: &Device, uniforms: &mut Uniforms) {
        uniforms.remove_texture(self.shader_index);
        self.image.destroy(device);
    }

    /// Get index to be used in shader for sampling
    pub const fn shader_index(&self) -> u32 {
        self.shader_index
    }
}
