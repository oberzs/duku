// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// enums for possible pipeline properties

use ash::vk;

#[derive(Debug, Copy, Clone)]
pub enum CullMode {
    Back,
    Front,
    Disable,
}

#[derive(Debug, Copy, Clone)]
pub enum WindingMode {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Copy, Clone)]
pub enum PolygonMode {
    Line,
    Fill,
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum AttachmentType {
    Color,
    Depth,
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
            Self::Disable => vk::CullModeFlags::NONE,
        }
    }
}

impl WindingMode {
    pub(crate) fn flag(&self) -> vk::FrontFace {
        match *self {
            Self::CounterClockwise => vk::FrontFace::COUNTER_CLOCKWISE,
            Self::Clockwise => vk::FrontFace::CLOCKWISE,
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
