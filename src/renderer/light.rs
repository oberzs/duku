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
    pub brightness: f32,
    pub light_type: LightType,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LightType {
    Main,
    Directional,
    Point,
}

impl Light {
    pub fn main(direction: impl Into<Vector3>, color: impl Into<Color>, brightness: f32) -> Self {
        Self {
            light_type: LightType::Main,
            coords: direction.into().unit(),
            color: color.into(),
            brightness,
        }
    }

    pub fn directional(
        direction: impl Into<Vector3>,
        color: impl Into<Color>,
        brightness: f32,
    ) -> Self {
        Self {
            light_type: LightType::Directional,
            coords: direction.into().unit(),
            color: color.into(),
            brightness,
        }
    }

    pub fn point(position: impl Into<Vector3>, color: impl Into<Color>, brightness: f32) -> Self {
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
            color: self.color.to_rgba_norm_vec() * self.brightness,
            light_type,
        }
    }

    pub(crate) const NONE: Self = Self {
        light_type: LightType::Point,
        coords: Vector3::ZERO,
        color: Color::BLACK,
        brightness: 0.0,
    };
}
