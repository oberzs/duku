pub mod colors;

use std::cmp;

use crate::math::Vector3;
use crate::math::Vector4;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
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

    pub fn hsv(h: u16, s: u8, v: u8) -> Self {
        let mut h_norm = h as f32;
        let s_norm = to_norm(s);
        let v_norm = to_norm(v);

        if s == 0 {
            return Self::rgb(v, v, v);
        }

        h_norm /= 60.0;
        let integr = h_norm.floor() as u16;
        let fract = h_norm - integr as f32;
        let pv = v_norm * (1.0 - s_norm);
        let qv = v_norm * (1.0 - s_norm * fract);
        let tv = v_norm * (1.0 - s_norm * (1.0 - fract));

        match integr {
            0 => Self::rgb_norm(v_norm, tv, pv),
            1 => Self::rgb_norm(qv, v_norm, pv),
            2 => Self::rgb_norm(pv, v_norm, tv),
            3 => Self::rgb_norm(pv, qv, v_norm),
            4 => Self::rgb_norm(tv, pv, v_norm),
            _ => Self::rgb_norm(v_norm, pv, qv),
        }
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

    pub fn to_hsv(&self) -> (u16, u8, u8) {
        let [r, g, b] = self.to_rgb_norm();

        let min = cmp::min(self.r, cmp::min(self.g, self.b));
        let max = cmp::max(self.r, cmp::max(self.g, self.b));
        let min_norm = to_norm(min);
        let max_norm = to_norm(max);

        let value = max;

        let delta = max_norm - min_norm;

        let saturation = if max != 0 {
            to_byte(delta / max_norm)
        } else {
            return (0, 0, value);
        };

        let mut hue = if self.r == max {
            (g - b) / delta
        } else if self.g == max {
            2.0 + (b - r) / delta
        } else {
            4.0 + (r - g) / delta
        };

        hue *= 60.0;
        if hue < 0.0 {
            hue += 360.0;
        }

        (hue as u16, saturation, value)
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

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn hsv() {
        assert_eq!(Color::hsv(0, 0, 255), Color::rgb(255, 255, 255));
        assert_eq!(Color::hsv(0, 0, 0), Color::rgb(0, 0, 0));
        assert_eq!(Color::hsv(0, 255, 255), Color::rgb(255, 0, 0));
    }

    #[test]
    fn to_hsv() {
        assert_eq!(Color::rgb(255, 255, 255).to_hsv(), (0, 0, 255));
        assert_eq!(Color::rgb(0, 0, 0).to_hsv(), (0, 0, 0));
        assert_eq!(Color::rgb(255, 0, 0).to_hsv(), (0, 255, 255));
    }
}
