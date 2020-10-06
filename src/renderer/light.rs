// Oliver Berzs
// https://github.com/oberzs/draw-it

// Light - struct for light data

use crate::color::Color;
use crate::math::Vector3;
use crate::pipeline::ShaderLight;

#[derive(Copy, Clone, Debug)]
pub struct Light {
    pub coords: Vector3,
    pub color: Color,
    pub light_type: LightType,
    pub shadows: bool,
    pub mesh: bool,
}

#[derive(Copy, Clone, Debug)]
pub enum LightType {
    Directional,
    Point,
}

impl Light {
    pub fn directional(
        direction: impl Into<Vector3>,
        color: impl Into<Color>,
        shadows: bool,
    ) -> Self {
        Self {
            light_type: LightType::Directional,
            coords: direction.into().unit(),
            color: color.into(),
            mesh: false,
            shadows,
        }
    }

    pub fn point(position: impl Into<Vector3>, color: impl Into<Color>, mesh: bool) -> Self {
        Self {
            light_type: LightType::Point,
            coords: position.into(),
            color: color.into(),
            shadows: false,
            mesh,
        }
    }

    pub(crate) fn shader(&self) -> ShaderLight {
        let light_type = match self.light_type {
            LightType::Directional => 0,
            LightType::Point => 1,
        };

        ShaderLight {
            coords: self.coords,
            color: self.color.to_rgba_norm_vec(),
            light_type,
        }
    }

    pub(crate) const NONE: Self = Self {
        light_type: LightType::Point,
        coords: Vector3::ZERO,
        color: Color::BLACK,
        mesh: false,
        shadows: false,
    };
}
