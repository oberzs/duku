pub mod colors;

use crate::math::Vector3;
use crate::math::Vector4;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb_norm(r: f32, g: f32, b: f32) -> Self {
        Self::rgb(to_byte(r), to_byte(g), to_byte(b))
    }

    pub fn rgba_norm(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::rgba(to_byte(r), to_byte(g), to_byte(b), to_byte(a))
    }

    pub fn to_rgb_norm(&self) -> [f32; 3] {
        [to_norm(self.r), to_norm(self.g), to_norm(self.b)]
    }

    pub fn to_rgba_norm(&self) -> [f32; 4] {
        [
            to_norm(self.r),
            to_norm(self.g),
            to_norm(self.b),
            to_norm(self.a),
        ]
    }

    pub fn to_rgb_norm_vec(&self) -> Vector3 {
        self.to_rgb_norm().into()
    }

    pub fn to_rgba_norm_vec(&self) -> Vector4 {
        self.to_rgba_norm().into()
    }
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Self::rgb(value[0], value[1], value[2])
    }
}

impl From<[u8; 4]> for Color {
    fn from(value: [u8; 4]) -> Self {
        Self::rgba(value[0], value[1], value[2], value[3])
    }
}

impl From<[f32; 3]> for Color {
    fn from(value: [f32; 3]) -> Self {
        Self::rgb_norm(value[0], value[1], value[2])
    }
}

impl From<[f32; 4]> for Color {
    fn from(value: [f32; 4]) -> Self {
        Self::rgba_norm(value[0], value[1], value[2], value[3])
    }
}

fn to_norm(value: u8) -> f32 {
    value as f32 / 255.0
}

fn to_byte(value: f32) -> u8 {
    (value * 255.0).round() as u8
}
