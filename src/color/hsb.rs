// Oliver Berzs
// https://github.com/oberzs/duku

use std::cmp;

use super::mixf;
use super::Mix;
use super::Rgb;
use super::Rgbf;

/// Color bytes in HSB with alpha.
///
/// Makes it easier to handle/convert colors.
///
/// HSB is the same as HSV.
///
/// # Examples
///
/// ```
/// # use duku::Hsb;
/// let red = Hsb::new(0, 100, 100);
/// # assert_eq!(red.h, 0);
/// # assert_eq!(red.s, 100);
/// # assert_eq!(red.b, 100);
/// # assert_eq!(red.a, 255);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Hsb {
    /// the hue component in range 0 to 360
    pub h: u16,
    /// the saturation component in range 0 to 100
    pub s: u8,
    /// the brightness component in range 0 to 100
    pub b: u8,
    /// the alpha component
    pub a: u8,
}

impl Hsb {
    /// Create color
    ///
    /// Converts components to correct
    /// ranges
    pub fn new(h: u16, s: u8, b: u8) -> Self {
        Self {
            h: h % 360,
            s: cmp::max(0, cmp::min(s, 100)),
            b: cmp::max(0, cmp::min(b, 100)),
            a: 255,
        }
    }

    /// Create color, modifying the color's alpha
    ///
    /// # Examples
    ///
    /// ```
    /// # use duku::Hsb;
    /// let color = Hsb::new(0, 100, 100).alpha(50);
    /// # assert_eq!(color.h, 0);
    /// # assert_eq!(color.s, 100);
    /// # assert_eq!(color.b, 100);
    /// # assert_eq!(color.a, 50);
    /// ```
    pub const fn alpha(mut self, a: u8) -> Self {
        self.a = a;
        self
    }

    /// Create color, shifting the hue by value
    ///
    /// # Examples
    ///
    /// ```
    /// # use duku::Hsb;
    /// let red = Hsb::new(360, 100, 100);
    /// let yellow = red.shift(60);
    /// # assert_eq!(yellow.h, 60);
    /// ```
    pub const fn shift(mut self, h: u16) -> Self {
        self.h = (self.h + h) % 360;
        self
    }

    /// Create color, darkening by some value
    ///
    /// # Examples
    ///
    /// ```
    /// # use duku::Hsb;
    /// let red = Hsb::new(0, 100, 100);
    /// let dark_red = red.darken(5);
    /// # assert_eq!(dark_red.b, 95);
    /// ```
    pub fn darken(mut self, v: u8) -> Self {
        self.b = cmp::max(0, cmp::min(self.b - v, 100));
        self
    }

    /// Create color, brightening by some value
    ///
    /// # Examples
    ///
    /// ```
    /// # use duku::Hsb;
    /// let red = Hsb::new(0, 100, 50);
    /// let bright_red = red.brighten(5);
    /// # assert_eq!(bright_red.b, 55);
    /// ```
    pub fn brighten(mut self, v: u8) -> Self {
        self.b = cmp::max(0, cmp::min(self.b + v, 100));
        self
    }

    /// Create color, saturating by some value
    ///
    /// # Examples
    ///
    /// ```
    /// # use duku::Hsb;
    /// let red = Hsb::new(0, 50, 100);
    /// let saturated_red = red.saturate(5);
    /// # assert_eq!(saturated_red.s, 55);
    /// ```
    pub fn saturate(mut self, v: u8) -> Self {
        self.s = cmp::max(0, cmp::min(self.s + v, 100));
        self
    }

    /// Create color, desaturating by some value
    ///
    /// # Examples
    ///
    /// ```
    /// # use duku::Hsb;
    /// let red = Hsb::new(0, 100, 100);
    /// let desaturated_red = red.desaturate(5);
    /// # assert_eq!(desaturated_red.s, 95);
    /// ```
    pub fn desaturate(mut self, v: u8) -> Self {
        self.s = cmp::max(0, cmp::min(self.s - v, 100));
        self
    }
}

impl Default for Hsb {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl From<Rgb> for Hsb {
    fn from(rgb: Rgb) -> Self {
        let rgbf = Rgbf::from(rgb);
        Hsb::from(rgbf)
    }
}

impl From<Rgbf> for Hsb {
    fn from(rgbf: Rgbf) -> Self {
        let min = {
            let mut m = if rgbf.r < rgbf.g { rgbf.r } else { rgbf.g };
            m = if m < rgbf.b { m } else { rgbf.b };
            m
        };
        let max = {
            let mut m = if rgbf.r > rgbf.g { rgbf.r } else { rgbf.g };
            m = if m > rgbf.b { m } else { rgbf.b };
            m
        };
        let delta = max - min;

        let bu = (max * 100.0).round() as u8;
        let au = (rgbf.a * 255.0).round() as u8;

        if delta < f32::EPSILON || max <= 0.0 {
            return Self {
                h: 0,
                s: 0,
                b: bu,
                a: au,
            };
        }

        let su = ((delta / max) * 100.0).round() as u8;

        let mut hf = if rgbf.r >= max {
            (rgbf.g - rgbf.b) / delta
        } else if rgbf.g >= max {
            2.0 + (rgbf.b - rgbf.r) / delta
        } else {
            4.0 + (rgbf.r - rgbf.g) / delta
        };

        hf *= 60.0;
        if hf < 0.0 {
            hf += 360.0;
        }

        Self {
            h: hf.round() as u16,
            s: su,
            b: bu,
            a: au,
        }
    }
}

impl Mix for Hsb {
    fn mix(from: Self, to: Self, p: f32) -> Self {
        Self {
            h: mixf(f32::from(from.h), f32::from(to.h), p) as u16 % 360,
            s: mixf(f32::from(from.s), f32::from(to.s), p) as u8,
            b: mixf(f32::from(from.b), f32::from(to.b), p) as u8,
            a: mixf(f32::from(from.a), f32::from(to.a), p) as u8,
        }
    }
}

impl From<&str> for Hsb {
    fn from(s: &str) -> Self {
        let rgb = Rgb::from(s);
        Self::from(rgb)
    }
}

#[cfg(test)]
mod tests {
    use super::Hsb;
    use super::Rgb;
    use super::Rgbf;

    #[test]
    fn from_rgbf() {
        let rgbf = Rgbf::new(0.64, 0.38, 0.78);
        let hsb = Hsb::from(rgbf);

        assert_eq!(hsb.h, 279);
        assert_eq!(hsb.s, 51);
        assert_eq!(hsb.b, 78);
        assert_eq!(hsb.a, 255);
    }

    #[test]
    fn from_rgb() {
        let rgb = Rgb::new(162, 97, 198);
        let hsb = Hsb::from(rgb);

        assert_eq!(hsb.h, 279);
        assert_eq!(hsb.s, 51);
        assert_eq!(hsb.b, 78);
        assert_eq!(hsb.a, 255);
    }
}
