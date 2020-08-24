use crate::image::Msaa;
use crate::renderer::Pcf;

#[derive(Debug, Copy, Clone)]
pub enum Quality {
    Low,
    Medium,
    High,
    Custom(QualityOptions),
}

#[derive(Debug, Copy, Clone)]
pub struct QualityOptions {
    pub shadow_map_size: u32,
    pub anisotropy: f32,
    pub msaa: Msaa,
    pub pcf: Pcf,
}

impl Quality {
    pub(crate) fn options(&self) -> QualityOptions {
        match *self {
            Self::Custom(o) => o,
            Self::Low => QualityOptions {
                shadow_map_size: 1024,
                anisotropy: 0.0,
                msaa: Msaa::Disabled,
                pcf: Pcf::Disabled,
            },
            Self::Medium => QualityOptions {
                shadow_map_size: 2048,
                anisotropy: 4.0,
                msaa: Msaa::X4,
                pcf: Pcf::X16,
            },
            Self::High => QualityOptions {
                shadow_map_size: 4096,
                anisotropy: 16.0,
                msaa: Msaa::X4,
                pcf: Pcf::X16,
            },
        }
    }
}
