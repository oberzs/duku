// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// enums for possible pipeline properties

use ash::vk;

#[derive(Debug, Copy, Clone)]
pub enum CullMode {
    Back,
    Front,
    Disabled,
}

#[derive(Debug, Copy, Clone)]
pub enum PolygonMode {
    Line,
    Fill,
}

#[derive(Debug, Copy, Clone)]
pub enum DepthMode {
    Test,
    Write,
    TestAndWrite,
    Disabled,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SamplerFilter {
    Linear,
    Nearest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SamplerAddress {
    Repeat,
    Clamp,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SamplerMipmaps {
    Enabled,
    Disabled,
}

impl CullMode {
    pub(crate) fn flag(&self) -> vk::CullModeFlags {
        match *self {
            Self::Back => vk::CullModeFlags::BACK,
            Self::Front => vk::CullModeFlags::FRONT,
            Self::Disabled => vk::CullModeFlags::NONE,
        }
    }
}

impl PolygonMode {
    pub(crate) fn flag(&self) -> vk::PolygonMode {
        match *self {
            Self::Fill => vk::PolygonMode::FILL,
            Self::Line => vk::PolygonMode::LINE,
        }
    }
}

impl DepthMode {
    pub(crate) fn test(&self) -> bool {
        match *self {
            Self::Test | Self::TestAndWrite => true,
            _ => false,
        }
    }

    pub(crate) fn write(&self) -> bool {
        match *self {
            Self::Write | Self::TestAndWrite => true,
            _ => false,
        }
    }
}

impl SamplerAddress {
    pub(crate) fn flag(&self) -> vk::SamplerAddressMode {
        match *self {
            Self::Clamp => vk::SamplerAddressMode::CLAMP_TO_BORDER,
            Self::Repeat => vk::SamplerAddressMode::REPEAT,
        }
    }
}

impl SamplerFilter {
    pub(crate) fn flag(&self) -> vk::Filter {
        match *self {
            Self::Linear => vk::Filter::LINEAR,
            Self::Nearest => vk::Filter::NEAREST,
        }
    }
}

impl SamplerMipmaps {
    pub(crate) fn flag(&self) -> vk::SamplerMipmapMode {
        match *self {
            Self::Enabled => vk::SamplerMipmapMode::LINEAR,
            Self::Disabled => vk::SamplerMipmapMode::NEAREST,
        }
    }
}
