// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// properties for surface

use crate::vk;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VSync {
    On,
    Off,
}

impl VSync {
    pub(crate) fn flag(&self) -> vk::PresentModeKHR {
        match *self {
            Self::On => vk::PRESENT_MODE_FIFO_KHR,
            Self::Off => vk::PRESENT_MODE_IMMEDIATE_KHR,
        }
    }
}
