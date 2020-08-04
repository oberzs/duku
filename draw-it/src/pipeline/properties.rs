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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SamplerFilter {
    Linear,
    Nearest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SamplerAddress {
    Repeat,
    Clamp,
    ClampEdge,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SamplerMipmaps {
    Enabled,
    Disabled,
}

pub(crate) fn sampler_index(
    filter: SamplerFilter,
    address: SamplerAddress,
    mipmaps: SamplerMipmaps,
) -> i32 {
    use SamplerAddress as A;
    use SamplerFilter as F;
    use SamplerMipmaps as M;

    match (filter, address, mipmaps) {
        (F::Linear, A::Repeat, M::Enabled) => 0,
        (F::Linear, A::Repeat, M::Disabled) => 1,
        (F::Linear, A::Clamp, M::Enabled) => 2,
        (F::Linear, A::Clamp, M::Disabled) => 3,
        (F::Linear, A::ClampEdge, M::Enabled) => 4,
        (F::Linear, A::ClampEdge, M::Disabled) => 5,
        (F::Nearest, A::Repeat, M::Enabled) => 6,
        (F::Nearest, A::Repeat, M::Disabled) => 7,
        (F::Nearest, A::Clamp, M::Enabled) => 8,
        (F::Nearest, A::Clamp, M::Disabled) => 9,
        (F::Nearest, A::ClampEdge, M::Enabled) => 10,
        (F::Nearest, A::ClampEdge, M::Disabled) => 11,
    }
}

impl CullMode {
    pub(crate) fn flag(&self) -> vk::CullModeFlags {
        match *self {
            Self::Back => vk::CullModeFlags::BACK,
            Self::Front => vk::CullModeFlags::FRONT,
            Self::Disabled => vk::CullModeFlags::NONE,
        }
    }
}

impl ShapeMode {
    pub(crate) fn polygon(&self) -> vk::PolygonMode {
        match *self {
            Self::FilledTriangles => vk::PolygonMode::FILL,
            Self::LinedTriangles => vk::PolygonMode::LINE,
            Self::Lines => vk::PolygonMode::LINE,
        }
    }

    pub(crate) fn topology(&self) -> vk::PrimitiveTopology {
        match *self {
            Self::FilledTriangles => vk::PrimitiveTopology::TRIANGLE_LIST,
            Self::LinedTriangles => vk::PrimitiveTopology::TRIANGLE_LIST,
            Self::Lines => vk::PrimitiveTopology::LINE_LIST,
        }
    }
}

impl DepthMode {
    pub(crate) fn test(&self) -> bool {
        matches!(*self, Self::Test | Self::TestAndWrite)
    }

    pub(crate) fn write(&self) -> bool {
        matches!(*self, Self::Write | Self::TestAndWrite)
    }
}

impl SamplerAddress {
    pub(crate) fn flag(&self) -> vk::SamplerAddressMode {
        match *self {
            Self::Clamp => vk::SamplerAddressMode::CLAMP_TO_BORDER,
            Self::ClampEdge => vk::SamplerAddressMode::CLAMP_TO_EDGE,
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

impl SamplerMipmaps {
    pub(crate) fn flag(&self) -> vk::SamplerMipmapMode {
        match *self {
            Self::Enabled => vk::SamplerMipmapMode::LINEAR,
            Self::Disabled => vk::SamplerMipmapMode::NEAREST,
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
