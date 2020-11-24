// Oliver Berzs
// https://github.com/oberzs/duku

use crate::vk;

/// VSync setting for rendering.
///
/// If this is on, rendering fps is locked
/// to the screen's refresh-rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VSync {
    /// turn vsync on
    On,
    /// turn vsync off
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
