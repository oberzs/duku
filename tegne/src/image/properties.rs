// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// enums for possible image properties

use ash::vk;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ImageFormat {
    Rgba,
    Bgra,
    Depth,
    DepthStencil,
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
    Shader,
    Present,
    TransferSrc,
    TransferDst,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum ImageMips {
    Log2,
    One,
}

pub(crate) struct LayoutChangeOptions {
    pub(crate) old_layout: ImageLayout,
    pub(crate) new_layout: ImageLayout,
    pub(crate) base_mip: u32,
    pub(crate) mip_count: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct ImageSamples(pub(crate) u8);

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
            Self::Rgba => vk::Format::R8G8B8A8_SRGB,
            Self::Bgra => vk::Format::B8G8R8A8_SRGB,
            Self::Depth => vk::Format::D32_SFLOAT_S8_UINT,
            Self::DepthStencil => vk::Format::D32_SFLOAT_S8_UINT,
        }
    }
}

impl ImageLayout {
    pub(crate) fn flag(&self) -> vk::ImageLayout {
        match *self {
            Self::Undefined => vk::ImageLayout::UNDEFINED,
            Self::Color => vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            Self::Depth => vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            Self::Shader => vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            Self::Present => vk::ImageLayout::PRESENT_SRC_KHR,
            Self::TransferSrc => vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            Self::TransferDst => vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        }
    }

    pub(crate) fn access_flag(&self) -> vk::AccessFlags {
        match *self {
            Self::TransferSrc => vk::AccessFlags::TRANSFER_READ,
            Self::TransferDst => vk::AccessFlags::TRANSFER_WRITE,
            Self::Shader => vk::AccessFlags::SHADER_READ,
            Self::Color => vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            Self::Depth => vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
            _ => vk::AccessFlags::TRANSFER_READ,
        }
    }

    pub(crate) fn stage_flag(&self) -> vk::PipelineStageFlags {
        match *self {
            Self::TransferSrc => vk::PipelineStageFlags::TRANSFER,
            Self::TransferDst => vk::PipelineStageFlags::TRANSFER,
            Self::Shader => vk::PipelineStageFlags::FRAGMENT_SHADER,
            Self::Color => vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            Self::Depth => {
                vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
                    | vk::PipelineStageFlags::LATE_FRAGMENT_TESTS
            }
            _ => vk::PipelineStageFlags::TRANSFER,
        }
    }
}

impl ImageSamples {
    pub(crate) fn flag(&self) -> vk::SampleCountFlags {
        match self.0 {
            1 => vk::SampleCountFlags::TYPE_1,
            2 => vk::SampleCountFlags::TYPE_2,
            4 => vk::SampleCountFlags::TYPE_4,
            8 => vk::SampleCountFlags::TYPE_8,
            16 => vk::SampleCountFlags::TYPE_16,
            32 => vk::SampleCountFlags::TYPE_32,
            64 => vk::SampleCountFlags::TYPE_64,
            _ => vk::SampleCountFlags::TYPE_1,
        }
    }
}

impl Default for LayoutChangeOptions {
    fn default() -> Self {
        Self {
            old_layout: ImageLayout::Undefined,
            new_layout: ImageLayout::Undefined,
            base_mip: 0,
            mip_count: 1,
        }
    }
}
