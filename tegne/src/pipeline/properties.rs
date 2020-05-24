// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// enums for possible pipeline properties

use ash::vk;

#[derive(Debug, Clone, Copy)]
pub(crate) enum DependencyType {
    Depth,
    Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum SamplerFilter {
    Linear,
    Nearest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum SamplerAddress {
    Repeat,
    Clamp,
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
