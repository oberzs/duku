// Oliver Berzs
// https://github.com/oberzs/duku

use crate::color::Rgbf;
use crate::math::Vec3;
use crate::math::Vec4;
use crate::pipeline::ShaderLight;

/// Light used in shadowing calculations.
///
/// # Examples
///
/// ```ignore
/// target.lights[0] = Light::main([-1.0, 1.0, 0.0], Color::RED, 5.0);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Light {
    /// either direction or position of the light
    pub coords: Vec3,
    /// color of the light
    pub color: Rgbf,
    /// brightness of the light,
    /// multiplied with the color in shaders
    pub brightness: f32,
    /// the type of the light
    pub light_type: LightType,
}

/// Type of a light.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LightType {
    /// directional light that casts shadows
    Main,
    /// directional light (like from the sun)
    Directional,
    /// point light (like from a lightbulb)
    Point,
}

impl Light {
    /// Create main light
    pub fn main(direction: impl Into<Vec3>, color: impl Into<Rgbf>, brightness: f32) -> Self {
        Self {
            light_type: LightType::Main,
            coords: direction.into().unit(),
            color: color.into(),
            brightness,
        }
    }

    /// Create directional light
    pub fn directional(
        direction: impl Into<Vec3>,
        color: impl Into<Rgbf>,
        brightness: f32,
    ) -> Self {
        Self {
            light_type: LightType::Directional,
            coords: direction.into().unit(),
            color: color.into(),
            brightness,
        }
    }

    /// Create point light
    pub fn point(position: impl Into<Vec3>, color: impl Into<Rgbf>, brightness: f32) -> Self {
        Self {
            light_type: LightType::Point,
            coords: position.into(),
            color: color.into(),
            brightness,
        }
    }

    pub(crate) fn shader(&self) -> ShaderLight {
        let light_type = match self.light_type {
            LightType::Main => 0,
            LightType::Directional => 1,
            LightType::Point => 2,
        };

        ShaderLight {
            coords: self.coords,
            color: Vec4::from(self.color) * self.brightness,
            light_type,
        }
    }
}
