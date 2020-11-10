// Oliver Berzs
// https://github.com/oberzs/duku

// size struct

use crate::vk;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Size {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Size {
    pub(crate) const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl From<u32> for Size {
    fn from(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

impl From<vk::Extent2D> for Size {
    fn from(extent: vk::Extent2D) -> Self {
        Self {
            width: extent.width,
            height: extent.height,
        }
    }
}

impl Into<vk::Extent2D> for Size {
    fn into(self) -> vk::Extent2D {
        vk::Extent2D {
            width: self.width,
            height: self.height,
        }
    }
}

impl Into<vk::Extent3D> for Size {
    fn into(self) -> vk::Extent3D {
        vk::Extent3D {
            width: self.width,
            height: self.height,
            depth: 1,
        }
    }
}

impl Into<vk::Offset3D> for Size {
    fn into(self) -> vk::Offset3D {
        vk::Offset3D {
            x: self.width as i32,
            y: self.height as i32,
            z: 1,
        }
    }
}
