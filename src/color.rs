// Oliver Berzs
// https://github.com/oberzs/duku

// Color struct

use std::cmp;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn hsv(h: u16, s: u8, v: u8) -> Self {
        let mut h_norm = f32::from(h);
        let s_norm = to_norm(s);
        let v_norm = to_norm(v);

        if s == 0 {
            return Self::rgb(v, v, v);
        }

        h_norm /= 60.0;
        let integr = h_norm.floor() as u16;
        let fract = h_norm - f32::from(integr);
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

    pub const fn gray(v: u8) -> Self {
        Self::rgb(v, v, v)
    }

    pub fn gray_norm(v: f32) -> Self {
        Self::rgb_norm(v, v, v)
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

    pub fn to_hsv(&self) -> (u16, u8, u8) {
        let [r, g, b] = self.to_rgb_norm();

        let min = cmp::min(self.r, cmp::min(self.g, self.b));
        let max = cmp::max(self.r, cmp::max(self.g, self.b));
        let min_norm = to_norm(min);
        let max_norm = to_norm(max);

        let value = max;

        let delta = max_norm - min_norm;

        let saturation = if max == 0 {
            return (0, 0, value);
        } else {
            to_byte(delta / max_norm)
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

    pub fn to_srgb(&self) -> Self {
        fn map(value: f32) -> f32 {
            let l = match value {
                v if v < 0.0 => 0.0,
                v if v > 1.0 => 1.0,
                v => v,
            };
            let cutoff = 0.0031308;
            let gamma = 2.2;

            if l <= cutoff {
                l * 12.92
            } else {
                1.055 * l.powf(1.0 / gamma) - 0.055
            }
        }

        let [r, g, b, a] = self.to_rgba_norm();
        Self::rgba_norm(map(r), map(g), map(b), a)
    }

    pub fn to_linear(&self) -> Self {
        fn map(value: f32) -> f32 {
            let s = match value {
                v if v < 0.0 => 0.0,
                v if v > 1.0 => 1.0,
                v => v,
            };
            let cutoff = 0.04045;
            let gamma = 2.2;

            if s <= cutoff {
                s / 12.92
            } else {
                ((s + 0.055) / 1.055).powf(gamma)
            }
        }

        let [r, g, b, a] = self.to_rgba_norm();
        Self::rgba_norm(map(r), map(g), map(b), a)
    }

    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const SILVER: Self = Self::rgb(192, 192, 192);
    pub const GRAY: Self = Self::rgb(128, 128, 128);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const MAROON: Self = Self::rgb(128, 0, 0);
    pub const YELLOW: Self = Self::rgb(255, 255, 0);
    pub const OLIVE: Self = Self::rgb(128, 128, 0);
    pub const AQUA: Self = Self::rgb(0, 255, 255);
    pub const TEAL: Self = Self::rgb(0, 128, 128);
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    pub const NAVY: Self = Self::rgb(0, 0, 128);
    pub const FUCHSIA: Self = Self::rgb(128, 0, 128);
    pub const PURPLE: Self = Self::rgb(128, 0, 128);

    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const SKY_BLUE: Self = Self::rgb(135, 206, 235);
    pub const ORANGE: Self = Self::rgb(255, 127, 0);
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
    f32::from(value) / 255.0
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
