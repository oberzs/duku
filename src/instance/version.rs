// Oliver Berzs
// https://github.com/oberzs/duku

// Vulkan version struct

use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct Version {
    pub(crate) major: u16,
    pub(crate) minor: u16,
    pub(crate) patch: u16,
}

impl Version {
    pub(crate) const fn from_vk(value: u32) -> Self {
        Self {
            major: ((value & 0xffc00000) >> 22) as u16,
            minor: ((value & 0x003ff000) >> 12) as u16,
            patch: (value & 0x00000fff) as u16,
        }
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, formatter)
    }
}
