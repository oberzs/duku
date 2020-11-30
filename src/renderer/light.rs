// Oliver Berzs
// https://github.com/oberzs/duku

use crate::color::Rgb;
use crate::color::Rgbf;
use crate::math::Vec3;
use crate::math::Vec4;
use crate::pipeline::ShaderLight;

/// Light used in shadowing calculations.
///
/// # Examples
///
/// ```no_run
/// # use duku::Duku;
/// # use duku::Light;
/// # let (mut d, _) = Duku::windowed(1, 1).unwrap();
/// let light = Light::directional("#ffffff", [-1.0, 1.0, 0.0]);
///
/// # d.draw(None, |t| {
/// t.light(light);
/// # });
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Light {
    /// either direction or position of the light
    pub coords: Vec3,
    /// color of the light
    pub color: Rgb,
    /// brightness of the light,
    /// multiplied with the color in shaders
    pub brightness: f32,
    /// the type of the light
    pub light_type: LightType,
}

/// Type of a light.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LightType {
    /// directional light (like from the sun)
    Directional,
    /// point light (like from a lightbulb)
    Point,
}

impl Light {
    /// Create directional light
    pub fn directional(color: impl Into<Rgb>, direction: impl Into<Vec3>) -> Self {
        Self {
            light_type: LightType::Directional,
            coords: direction.into().unit(),
            color: color.into(),
            brightness: 1.0,
        }
    }

    /// Create point light
    pub fn point(color: impl Into<Rgb>, position: impl Into<Vec3>) -> Self {
        Self {
            light_type: LightType::Point,
            coords: position.into(),
            color: color.into(),
            brightness: 1.0,
        }
    }

    pub(crate) fn none() -> Self {
        Self::point(Rgb::clear(), [0.0, 0.0, 0.0])
    }

    pub(crate) const fn is_none(&self) -> bool {
        self.color.a == 0
    }

    pub(crate) fn shader(&self) -> ShaderLight {
        let light_type = match self.light_type {
            LightType::Directional => 0,
            LightType::Point => 1,
        };

        ShaderLight {
            coords: self.coords,
            color: Vec4::from(Rgbf::from(self.color)) * self.brightness,
            light_type,
        }
    }
}
