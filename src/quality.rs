// Oliver Berzs
// https://github.com/oberzs/draw-it

// Quality - various renderer quality settings

use crate::image::Msaa;
use crate::renderer::Pcf;

#[derive(Debug, Copy, Clone)]
pub enum Quality {
    Low,
    Medium,
    High,
    Custom(QualitySettings),
}

#[derive(Debug, Copy, Clone)]
pub struct QualitySettings {
    pub shadow_map_size: u32,
    pub anisotropy: f32,
    pub msaa: Msaa,
    pub pcf: Pcf,
}

impl Quality {
    pub(crate) const fn settings(&self) -> QualitySettings {
        match *self {
            Self::Custom(o) => o,
            Self::Low => QualitySettings {
                shadow_map_size: 1024,
                anisotropy: 1.0,
                msaa: Msaa::Disabled,
                pcf: Pcf::Disabled,
            },
            Self::Medium => QualitySettings {
                shadow_map_size: 2048,
                anisotropy: 4.0,
                msaa: Msaa::X4,
                pcf: Pcf::X16,
            },
            Self::High => QualitySettings {
                shadow_map_size: 4096,
                anisotropy: 16.0,
                msaa: Msaa::X4,
                pcf: Pcf::X16,
            },
        }
    }
}
