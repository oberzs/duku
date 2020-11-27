// Oliver Berzs
// https://github.com/oberzs/duku

use super::with_alpha;
use super::Format;
use super::Image;
use super::ImageLayout;
use crate::buffer::Buffer;
use crate::device::Device;
use crate::error::Result;
use crate::pipeline::Uniforms;

/// Texture representation of an environment.
///
/// Contains 6 square side textures of a cube that
/// can be sampled by a direction in the shader.
///
/// # Examples
///
/// ```ignore
/// let skybox = duku.create_cubemap_png(CubemapSides {
///     top: "path/to/top.png",
///     bottom: "path/to/bottom.png",
///     front: "path/to/front.png",
///     back: "path/to/back.png",
///     left: "path/to/left.png",
///     right: "path/to/right.png",
/// });
/// ```
pub struct Cubemap {
    image: Image,
    shader_index: u32,
}

/// 6 square sides of the cubemap.
///
/// # Examples
///
/// ```ignore
/// let skybox = duku.create_cubemap_png(CubemapSides {
///     top: "path/to/top.png",
///     bottom: "path/to/bottom.png",
///     front: "path/to/front.png",
///     back: "path/to/back.png",
///     left: "path/to/left.png",
///     right: "path/to/right.png",
/// });
/// ```
pub struct CubemapSides<T> {
    /// top face of the cube
    pub top: T,
    /// bottom face of the cube
    pub bottom: T,
    /// front face of the cube
    pub front: T,
    /// back face of the cube
    pub back: T,
    /// left face of the cube
    pub left: T,
    /// right face of the cube
    pub right: T,
}

impl Cubemap {
    pub(crate) fn new(
        device: &Device,
        uniforms: &mut Uniforms,
        size: u32,
        format: Format,
        sides: CubemapSides<Vec<u8>>,
    ) -> Result<Self> {
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
        let mut image = Image::cubemap(device, format, size);

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

        let shader_index = uniforms.add_cubemap(image.add_view(device))?;

        Ok(Self {
            image,
            shader_index,
        })
    }

    pub(crate) fn destroy(&self, device: &Device, uniforms: &mut Uniforms) {
        uniforms.remove_cubemap(self.shader_index);
        self.image.destroy(device);
    }

    /// Get index to be used in shader for sampling
    pub const fn shader_index(&self) -> u32 {
        self.shader_index
    }
}
