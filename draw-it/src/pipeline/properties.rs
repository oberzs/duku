// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// enums for possible pipeline properties

use ash::vk;

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
            Self::Back => vk::CullModeFlags::BACK,
            Self::Front => vk::CullModeFlags::FRONT,
            Self::Disabled => vk::CullModeFlags::NONE,
        }
    }
}

impl ShapeMode {
    pub(crate) const fn polygon(&self) -> vk::PolygonMode {
        match *self {
            Self::FilledTriangles => vk::PolygonMode::FILL,
            Self::LinedTriangles => vk::PolygonMode::LINE,
            Self::Lines => vk::PolygonMode::LINE,
        }
    }

    pub(crate) const fn topology(&self) -> vk::PrimitiveTopology {
        match *self {
            Self::FilledTriangles => vk::PrimitiveTopology::TRIANGLE_LIST,
            Self::LinedTriangles => vk::PrimitiveTopology::TRIANGLE_LIST,
            Self::Lines => vk::PrimitiveTopology::LINE_LIST,
        }
    }
}

impl DepthMode {
    pub(crate) const fn test(&self) -> bool {
        matches!(*self, Self::Test | Self::TestAndWrite)
    }

    pub(crate) const fn write(&self) -> bool {
        matches!(*self, Self::Write | Self::TestAndWrite)
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
