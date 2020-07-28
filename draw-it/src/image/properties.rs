// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// enums for possible image properties

use ash::vk;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ImageFormat {
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

#[derive(Debug, Copy, Clone)]
pub(crate) enum ImageMips {
    Log2,
    One,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Msaa {
    X4,
    X8,
    X16,
    Disabled,
}

impl ImageUsage {
    pub(crate) fn combine(usages: &[Self]) -> vk::ImageUsageFlags {
        usages
            .iter()
            .fold(vk::ImageUsageFlags::empty(), |acc, usage| {
                acc | usage.flag()
            })
    }

    pub(crate) fn flag(&self) -> vk::ImageUsageFlags {
        match *self {
            Self::Color => vk::ImageUsageFlags::COLOR_ATTACHMENT,
            Self::Depth => vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            Self::Transient => vk::ImageUsageFlags::TRANSIENT_ATTACHMENT,
            Self::TransferSrc => vk::ImageUsageFlags::TRANSFER_SRC,
            Self::TransferDst => vk::ImageUsageFlags::TRANSFER_DST,
            Self::Sampled => vk::ImageUsageFlags::SAMPLED,
        }
    }
}

impl ImageFormat {
    pub(crate) fn flag(&self) -> vk::Format {
        match *self {
            Self::Rgb => vk::Format::R8G8B8_UNORM,
            Self::Rgba => vk::Format::R8G8B8A8_UNORM,
            Self::Srgb => vk::Format::R8G8B8_SRGB,
            Self::Srgba => vk::Format::R8G8B8A8_SRGB,
            Self::Sbgra => vk::Format::B8G8R8A8_SRGB,
            Self::Depth => vk::Format::D32_SFLOAT_S8_UINT,
            Self::DepthStencil => vk::Format::D32_SFLOAT_S8_UINT,
            Self::Float2 => vk::Format::R32G32_SFLOAT,
            Self::Gray => vk::Format::R8_UNORM,
        }
    }

    pub(crate) fn is_color(&self) -> bool {
        matches!(
            *self,
            Self::Float2
                | Self::Gray
                | Self::Rgb
                | Self::Rgba
                | Self::Sbgra
                | Self::Srgb
                | Self::Srgba
        )
    }

    // probably will some day be needed
    pub(crate) fn _is_depth(&self) -> bool {
        matches!(*self, Self::Depth | Self::DepthStencil)
    }
}

impl ImageLayout {
    pub(crate) fn flag(&self) -> vk::ImageLayout {
        match *self {
            Self::Undefined => vk::ImageLayout::UNDEFINED,
            Self::Color => vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            Self::Depth => vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            Self::ShaderColor => vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            Self::ShaderDepth => vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
            Self::Present => vk::ImageLayout::PRESENT_SRC_KHR,
            Self::TransferSrc => vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            Self::TransferDst => vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        }
    }

    pub(crate) fn access_flag(&self) -> vk::AccessFlags {
        match *self {
            Self::TransferSrc => vk::AccessFlags::TRANSFER_READ,
            Self::TransferDst => vk::AccessFlags::TRANSFER_WRITE,
            Self::ShaderColor => vk::AccessFlags::SHADER_READ,
            Self::ShaderDepth => vk::AccessFlags::SHADER_READ,
            Self::Color => vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            Self::Depth => vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
            _ => vk::AccessFlags::TRANSFER_READ,
        }
    }

    pub(crate) fn stage_flag(&self) -> vk::PipelineStageFlags {
        match *self {
            Self::TransferSrc => vk::PipelineStageFlags::TRANSFER,
            Self::TransferDst => vk::PipelineStageFlags::TRANSFER,
            Self::ShaderColor => vk::PipelineStageFlags::FRAGMENT_SHADER,
            Self::ShaderDepth => vk::PipelineStageFlags::FRAGMENT_SHADER,
            Self::Color => vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            Self::Depth => {
                vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
                    | vk::PipelineStageFlags::LATE_FRAGMENT_TESTS
            }
            _ => vk::PipelineStageFlags::TRANSFER,
        }
    }
}

impl Msaa {
    pub(crate) fn flag(&self) -> vk::SampleCountFlags {
        match *self {
            Self::Disabled => vk::SampleCountFlags::TYPE_1,
            Self::X4 => vk::SampleCountFlags::TYPE_4,
            Self::X8 => vk::SampleCountFlags::TYPE_8,
            Self::X16 => vk::SampleCountFlags::TYPE_16,
        }
    }
}
