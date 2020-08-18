// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// properties for surface

use ash::vk;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ColorSpace {
    Srgb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VSync {
    On,
    Off,
}

impl ColorSpace {
    pub(crate) const fn flag(&self) -> vk::ColorSpaceKHR {
        vk::ColorSpaceKHR::SRGB_NONLINEAR
    }
}

impl VSync {
    pub(crate) const fn flag(&self) -> vk::PresentModeKHR {
        match *self {
            Self::On => vk::PresentModeKHR::FIFO,
            Self::Off => vk::PresentModeKHR::IMMEDIATE,
        }
    }
}
