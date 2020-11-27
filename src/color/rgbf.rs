// Oliver Berzs
// https://github.com/oberzs/duku

use super::mixf;
use super::Hsb;
use super::Mix;
use super::Rgb;

/// Color floats in RGB with alpha.
///
/// Makes it easier to handle/convert colors.
///
/// # Examples
///
/// ```
/// # use duku::Rgbf;
/// let red = Rgbf::red(0.7);
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rgbf {
    /// the red component
    pub r: f32,
    /// the green component
    pub g: f32,
    /// the blue component
    pub b: f32,
    /// the alpha component
    pub a: f32,
}

impl Rgbf {
    /// Create color
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Create color with the same value for all
    /// components
    pub const fn gray(v: f32) -> Self {
        Self::new(v, v, v)
    }

    /// Create color with setting only the red
    /// component
    pub const fn red(v: f32) -> Self {
        Self::new(v, 0.0, 0.0)
    }

    /// Create color with setting only the green
    /// component
    pub const fn green(v: f32) -> Self {
        Self::new(0.0, v, 0.0)
    }

    /// Create color with setting only the blue
    /// component
    pub const fn blue(v: f32) -> Self {
        Self::new(0.0, 0.0, v)
    }

    /// Create transparent color
    pub const fn clear() -> Self {
        Self::gray(0.0).alpha(0.0)
    }

    /// Create color, modifying the color's alpha
    ///
    /// # Examples
    ///
    /// ```
    /// # use duku::Rgbf;
    /// let color = Rgbf::blue(0.5).alpha(0.3);
    /// ```
    pub const fn alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }
}

impl Mix for Rgbf {
    fn mix(from: Self, to: Self, p: f32) -> Self {
        Self {
            r: mixf(from.r, to.r, p),
            g: mixf(from.g, to.g, p),
            b: mixf(from.b, to.b, p),
            a: mixf(from.a, to.a, p),
        }
    }
}

impl Default for Rgbf {
    fn default() -> Self {
        Self::gray(0.0)
    }
}

impl From<Rgb> for Rgbf {
    fn from(c: Rgb) -> Self {
        Self {
            r: f32::from(c.r) / 255.0,
            g: f32::from(c.g) / 255.0,
            b: f32::from(c.b) / 255.0,
            a: f32::from(c.a) / 255.0,
        }
    }
}

impl From<Hsb> for Rgbf {
    fn from(hsb: Hsb) -> Self {
        let hf = f32::from(hsb.h % 360);
        let sf = f32::from(hsb.s) / 100.0;
        let bf = f32::from(hsb.b) / 100.0;
        let af = f32::from(hsb.a) / 255.0;

        if hsb.s == 0 {
            Self {
                r: bf,
                g: bf,
                b: bf,
                a: af,
            }
        } else {
            let hh = hf / 60.0;
            let i = hh.floor();
            let ff = hh - i;
            let p = bf * (1.0 - sf);
            let q = bf * (1.0 - (sf * ff));
            let t = bf * (1.0 - (sf * (1.0 - ff)));

            match i as u32 {
                0 => Self {
                    r: bf,
                    g: t,
                    b: p,
                    a: af,
                },
                1 => Self {
                    r: q,
                    g: bf,
                    b: p,
                    a: af,
                },
                2 => Self {
                    r: p,
                    g: bf,
                    b: t,
                    a: af,
                },
                3 => Self {
                    r: p,
                    g: q,
                    b: bf,
                    a: af,
                },
                4 => Self {
                    r: t,
                    g: p,
                    b: bf,
                    a: af,
                },
                _ => Self {
                    r: bf,
                    g: p,
                    b: q,
                    a: af,
                },
            }
        }
    }
}

impl From<&str> for Rgbf {
    fn from(s: &str) -> Self {
        let rgb = Rgb::from(s);
        Self::from(rgb)
    }
}

impl From<[f32; 3]> for Rgbf {
    fn from(a: [f32; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl From<[f32; 4]> for Rgbf {
    fn from(a: [f32; 4]) -> Self {
        Self::new(a[0], a[1], a[2]).alpha(a[3])
    }
}

impl Into<[f32; 4]> for Rgbf {
    fn into(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

#[cfg(test)]
mod tests {
    use super::Hsb;
    use super::Rgb;
    use super::Rgbf;

    #[test]
    fn from_rgb() {
        let rgb = Rgb::new(255, 128, 0);
        let rgbf = Rgbf::from(rgb);

        assert_eq_delta!(rgbf.r, 1.0);
        assert_eq_delta!(rgbf.g, 0.5019608);
        assert_eq_delta!(rgbf.b, 0.0);
        assert_eq_delta!(rgbf.a, 1.0);
    }

    #[test]
    fn from_hsb() {
        let hsb = Hsb::new(7, 89, 83);
        let rgbf = Rgbf::from(hsb);

        assert_eq_delta!(rgbf.r, 0.83);
        assert_eq_delta!(rgbf.g, 0.177482);
        assert_eq_delta!(rgbf.b, 0.0913);
        assert_eq_delta!(rgbf.a, 1.0);
    }
}
