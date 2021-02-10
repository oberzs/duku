// Oliver Berzs
// https://github.com/oberzs/duku

// enums for possible pipeline properties

use std::convert::TryFrom;

use crate::error;
use crate::error::Result;
use crate::vk;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum CullMode {
    Back,
    Front,
    Disabled,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ShapeMode {
    LinedTriangles,
    FilledTriangles,
    Lines,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum DepthMode {
    Test,
    Write,
    TestAndWrite,
    Disabled,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Store {
    Enabled,
    Disabled,
}

impl CullMode {
    pub(crate) const fn flag(&self) -> vk::CullModeFlags {
        match *self {
            Self::Back => vk::CULL_MODE_BACK_BIT,
            Self::Front => vk::CULL_MODE_FRONT_BIT,
            Self::Disabled => vk::CULL_MODE_NONE,
        }
    }
}

impl ShapeMode {
    pub(crate) const fn polygon(&self) -> vk::PolygonMode {
        match *self {
            Self::FilledTriangles => vk::POLYGON_MODE_FILL,
            Self::LinedTriangles => vk::POLYGON_MODE_LINE,
            Self::Lines => vk::POLYGON_MODE_LINE,
        }
    }

    pub(crate) const fn topology(&self) -> vk::PrimitiveTopology {
        match *self {
            Self::FilledTriangles => vk::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            Self::LinedTriangles => vk::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            Self::Lines => vk::PRIMITIVE_TOPOLOGY_LINE_LIST,
        }
    }
}

impl DepthMode {
    pub(crate) const fn test(&self) -> vk::Bool32 {
        if matches!(*self, Self::Test | Self::TestAndWrite) {
            vk::TRUE
        } else {
            vk::FALSE
        }
    }

    pub(crate) const fn write(&self) -> vk::Bool32 {
        if matches!(*self, Self::Write | Self::TestAndWrite) {
            vk::TRUE
        } else {
            vk::FALSE
        }
    }
}

impl Store {
    pub(crate) const fn flag(&self) -> vk::AttachmentStoreOp {
        match *self {
            Self::Enabled => vk::ATTACHMENT_STORE_OP_STORE,
            Self::Disabled => vk::ATTACHMENT_STORE_OP_DONT_CARE,
        }
    }
}

impl TryFrom<u8> for CullMode {
    type Error = error::Error;

    fn try_from(byte: u8) -> Result<Self> {
        match byte {
            0 => Ok(Self::Back),
            1 => Ok(Self::Front),
            2 => Ok(Self::Disabled),
            _ => Err(error::Error::InvalidSpirv),
        }
    }
}

impl TryFrom<u8> for ShapeMode {
    type Error = error::Error;

    fn try_from(byte: u8) -> Result<Self> {
        match byte {
            0 => Ok(Self::LinedTriangles),
            1 => Ok(Self::FilledTriangles),
            2 => Ok(Self::Lines),
            _ => Err(error::Error::InvalidSpirv),
        }
    }
}

impl TryFrom<u8> for DepthMode {
    type Error = error::Error;

    fn try_from(byte: u8) -> Result<Self> {
        match byte {
            0 => Ok(Self::Test),
            1 => Ok(Self::Write),
            2 => Ok(Self::TestAndWrite),
            3 => Ok(Self::Disabled),
            _ => Err(error::Error::InvalidSpirv),
        }
    }
}

impl From<bool> for Store {
    fn from(b: bool) -> Self {
        if b {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}
