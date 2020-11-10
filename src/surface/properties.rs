// Oliver Berzs
// https://github.com/oberzs/duku

// properties for surface

use crate::vk;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VSync {
    On,
    Off,
}

impl VSync {
    pub(crate) const fn flag(&self) -> vk::PresentModeKHR {
        match *self {
            Self::On => vk::PRESENT_MODE_FIFO_KHR,
            Self::Off => vk::PRESENT_MODE_IMMEDIATE_KHR,
        }
    }
}
