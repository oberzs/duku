// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// enums for possible pipeline properties

use crate::vk;

#[derive(Debug, Copy, Clone)]
pub(crate) enum CullMode {
    Back,
    Front,
    Disabled,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum ShapeMode {
    LinedTriangles,
    FilledTriangles,
    Lines,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum DepthMode {
    Test,
    Write,
    TestAndWrite,
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

impl From<&String> for CullMode {
    fn from(s: &String) -> Self {
        match s.as_str() {
            "back" => Self::Back,
            "front" => Self::Front,
            "disabled" => Self::Disabled,
            _ => unreachable!(),
        }
    }
}

impl From<&String> for ShapeMode {
    fn from(s: &String) -> Self {
        match s.as_str() {
            "lined_triangles" => Self::LinedTriangles,
            "filled_triangles" => Self::FilledTriangles,
            "lines" => Self::Lines,
            _ => unreachable!(),
        }
    }
}

impl From<&String> for DepthMode {
    fn from(s: &String) -> Self {
        match s.as_str() {
            "test" => Self::Test,
            "write" => Self::Write,
            "test_and_write" => Self::TestAndWrite,
            "disabled" => Self::Disabled,
            _ => unreachable!(),
        }
    }
}
