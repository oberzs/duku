// Oliver Berzs
// https://github.com/oberzs/duku

// enums for possible image properties

use crate::vk;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Format {
    Rgb,
    Rgba,
    Srgb,
    Srgba,
    Sbgra,
    Depth,
    DepthStencil,
    Float2,
    Gray,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mips {
    Log2,
    Zero,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Msaa {
    X4,
    X8,
    X16,
    Disabled,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TextureFilter {
    Linear,
    Nearest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TextureWrap {
    ClampBorder,
    ClampEdge,
    Repeat,
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
            Self::Sbgra => vk::FORMAT_B8G8R8A8_SRGB,
            Self::Depth => vk::FORMAT_D32_SFLOAT_S8_UINT,
            Self::DepthStencil => vk::FORMAT_D32_SFLOAT_S8_UINT,
            Self::Float2 => vk::FORMAT_R32G32_SFLOAT,
            Self::Gray => vk::FORMAT_R8_UNORM,
        }
    }

    pub(crate) const fn aspect(&self) -> vk::ImageAspectFlags {
        match *self {
            Self::Sbgra
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

impl TextureWrap {
    pub(crate) const fn flag(&self) -> vk::SamplerAddressMode {
        match *self {
            Self::ClampBorder => vk::SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER,
            Self::ClampEdge => vk::SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
            Self::Repeat => vk::SAMPLER_ADDRESS_MODE_REPEAT,
        }
    }
}

impl TextureFilter {
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
