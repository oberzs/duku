// Oliver Berzs
// https://github.com/oberzs/duku

use crate::vk;

/// Color value representation mode.
///
/// Defines how textures should be sampled in the shader.
///
/// # Example
///
/// ```ignore
/// let texture = duku.create_texture_png("path/to/image.png", ColorSpace::Srgb, Mips::Log2);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ColorSpace {
    /// a linear representation of color values
    Linear,
    /// a representation closer to how humans perceive light
    Srgb,
}

/// Texture mipmapping mode.
///
/// Generates extra smaller textures that are used
/// when objects are rendered further away.
///
/// Rendering with mips is faster to sample, but they
/// require more memory.
///
/// # Example
///
/// ```ignore
/// let texture = duku.create_texture_png("path/to/image.png", ColorSpace::Srgb, Mips::Log2);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mips {
    /// generates as many mips as possible
    Log2,
    /// generates no mips
    Zero,
}

/// Multi Sample Anti-Aliasing mode.
///
/// Makes mesh outlines smoother (adds anti-aliasing)
/// by sampling each pixel multiple times.
///
/// Higher settings greatly impact performance, also
/// require more memory for bigger framebuffers.
///
/// # Example
///
/// ```ignore
/// let duku = Duku::builder().msaa(Msaa::X4).build()?;
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Msaa {
    /// samples each pixel 4 times
    X4,
    /// samples each pixel 8 times
    X8,
    /// samples each pixel 16 times
    X16,
    /// no MSAA is applied
    Disabled,
}

/// Filtering mode for texture sampling.
///
/// # Example
///
/// ```ignore
/// duku.draw_on_window(None, |target| {
///     target.texture_filter = Filter::Nearest;
/// });
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Filter {
    /// interpolates linearly between texture pixels
    Linear,
    /// does no interpolation (good for pixel art)
    Nearest,
}

/// Wrapping mode for texture sampling.
///
/// Defines what should be returned when sampling outside
/// of the texture's UV range
///
/// # Example
///
/// ```ignore
/// duku.draw_on_window(None, |target| {
///     target.texture_filter = Wrap::Repeat;
/// });
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Wrap {
    /// samples black outside of texture
    ClampBorder,
    /// samples edge pixels outside of texture
    ClampEdge,
    /// repeats the texture
    Repeat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Format {
    Rgb,
    Rgba,
    Srgb,
    Srgba,
    Bgra,
    Depth,
    DepthStencil,
    Float2,
    Gray,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ImageUsage {
    Depth,
    Color,
    Transient,
    TransferSrc,
    TransferDst,
    Sampled,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ImageLayout {
    Undefined,
    Depth,
    Color,
    ShaderColor,
    ShaderDepth,
    Present,
    TransferSrc,
    TransferDst,
}

impl ImageUsage {
    pub(crate) fn combine(usages: &[Self]) -> vk::ImageUsageFlags {
        usages.iter().fold(0, |acc, usage| acc | usage.flag())
    }

    pub(crate) const fn flag(&self) -> vk::ImageUsageFlags {
        match *self {
            Self::Color => vk::IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
            Self::Depth => vk::IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT,
            Self::Transient => vk::IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT,
            Self::TransferSrc => vk::IMAGE_USAGE_TRANSFER_SRC_BIT,
            Self::TransferDst => vk::IMAGE_USAGE_TRANSFER_DST_BIT,
            Self::Sampled => vk::IMAGE_USAGE_SAMPLED_BIT,
        }
    }
}

impl Format {
    pub(crate) const fn flag(&self) -> vk::Format {
        match *self {
            Self::Rgb => vk::FORMAT_R8G8B8_UNORM,
            Self::Rgba => vk::FORMAT_R8G8B8A8_UNORM,
            Self::Srgb => vk::FORMAT_R8G8B8_SRGB,
            Self::Srgba => vk::FORMAT_R8G8B8A8_SRGB,
            Self::Bgra => vk::FORMAT_B8G8R8A8_UNORM,
            Self::Depth => vk::FORMAT_D32_SFLOAT_S8_UINT,
            Self::DepthStencil => vk::FORMAT_D32_SFLOAT_S8_UINT,
            Self::Float2 => vk::FORMAT_R32G32_SFLOAT,
            Self::Gray => vk::FORMAT_R8_UNORM,
        }
    }

    pub(crate) const fn aspect(&self) -> vk::ImageAspectFlags {
        match *self {
            Self::Bgra
            | Self::Rgb
            | Self::Rgba
            | Self::Srgba
            | Self::Srgb
            | Self::Float2
            | Self::Gray => vk::IMAGE_ASPECT_COLOR_BIT,
            Self::Depth => vk::IMAGE_ASPECT_DEPTH_BIT,
            Self::DepthStencil => vk::IMAGE_ASPECT_DEPTH_BIT | vk::IMAGE_ASPECT_STENCIL_BIT,
        }
    }

    pub(crate) const fn all_aspects(&self) -> vk::ImageAspectFlags {
        let aspect = self.aspect();
        if aspect == vk::IMAGE_ASPECT_DEPTH_BIT {
            aspect | vk::IMAGE_ASPECT_STENCIL_BIT
        } else {
            aspect
        }
    }

    pub(crate) const fn is_depth(&self) -> bool {
        matches!(*self, Self::Depth | Self::DepthStencil)
    }
}

impl ImageLayout {
    pub(crate) const fn flag(&self) -> vk::ImageLayout {
        match *self {
            Self::Undefined => vk::IMAGE_LAYOUT_UNDEFINED,
            Self::Color => vk::IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
            Self::Depth => vk::IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            Self::ShaderColor => vk::IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
            Self::ShaderDepth => vk::IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL,
            Self::Present => vk::IMAGE_LAYOUT_PRESENT_SRC_KHR,
            Self::TransferSrc => vk::IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
            Self::TransferDst => vk::IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
        }
    }

    pub(crate) const fn access_flag(&self) -> vk::AccessFlags {
        match *self {
            Self::TransferSrc => vk::ACCESS_TRANSFER_READ_BIT,
            Self::TransferDst => vk::ACCESS_TRANSFER_WRITE_BIT,
            Self::ShaderColor => vk::ACCESS_SHADER_READ_BIT,
            Self::ShaderDepth => vk::ACCESS_SHADER_READ_BIT,
            Self::Color => vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
            Self::Depth => vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
            _ => vk::ACCESS_TRANSFER_READ_BIT,
        }
    }

    pub(crate) const fn stage_flag(&self) -> vk::PipelineStageFlags {
        match *self {
            Self::TransferSrc => vk::PIPELINE_STAGE_TRANSFER_BIT,
            Self::TransferDst => vk::PIPELINE_STAGE_TRANSFER_BIT,
            Self::ShaderColor => vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT,
            Self::ShaderDepth => vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT,
            Self::Color => vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
            Self::Depth => {
                vk::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT
                    | vk::PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT
            }
            _ => vk::PIPELINE_STAGE_TRANSFER_BIT,
        }
    }
}

impl Msaa {
    pub(crate) const fn flag(&self) -> vk::SampleCountFlags {
        match *self {
            Self::Disabled => vk::SAMPLE_COUNT_1_BIT,
            Self::X4 => vk::SAMPLE_COUNT_4_BIT,
            Self::X8 => vk::SAMPLE_COUNT_8_BIT,
            Self::X16 => vk::SAMPLE_COUNT_16_BIT,
        }
    }
}

impl Wrap {
    pub(crate) const fn flag(&self) -> vk::SamplerAddressMode {
        match *self {
            Self::ClampBorder => vk::SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER,
            Self::ClampEdge => vk::SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
            Self::Repeat => vk::SAMPLER_ADDRESS_MODE_REPEAT,
        }
    }
}

impl Filter {
    pub(crate) const fn flag(&self) -> vk::Filter {
        match *self {
            Self::Linear => vk::FILTER_LINEAR,
            Self::Nearest => vk::FILTER_NEAREST,
        }
    }
}

pub(crate) fn with_alpha(data: Vec<u8>) -> Vec<u8> {
    let mut new_data = Vec::with_capacity(4 * data.len() / 3);
    for pixel in data.chunks(3) {
        new_data.extend(&[pixel[0], pixel[1], pixel[2], 255]);
    }
    new_data
}
