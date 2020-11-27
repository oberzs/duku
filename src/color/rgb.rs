// Oliver Berzs
// https://github.com/oberzs/duku

use super::mixf;
use super::Hsb;
use super::Mix;
use super::Rgbf;

/// Color bytes in RGB with alpha.
///
/// Makes it easier to handle/convert colors.
///
/// # Examples
///
/// ```
/// # use duku::Rgb;
/// let red = Rgb::red(50);
/// # assert_eq!(red.r, 50);
/// # assert_eq!(red.g, 0);
/// # assert_eq!(red.b, 0);
/// # assert_eq!(red.a, 255);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Rgb {
    /// the red component
    pub r: u8,
    /// the green component
    pub g: u8,
    /// the blue component
    pub b: u8,
    /// the alpha component
    pub a: u8,
}

impl Rgb {
    /// Create color
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create color with the same value for all
    /// components
    pub const fn gray(v: u8) -> Self {
        Self::new(v, v, v)
    }

    /// Create color with setting only the red
    /// component
    pub const fn red(v: u8) -> Self {
        Self::new(v, 0, 0)
    }

    /// Create color with setting only the green
    /// component
    pub const fn green(v: u8) -> Self {
        Self::new(0, v, 0)
    }

    /// Create color with setting only the blue
    /// component
    pub const fn blue(v: u8) -> Self {
        Self::new(0, 0, v)
    }

    /// Create transparent color
    pub const fn clear() -> Self {
        Self::gray(0).alpha(0)
    }

    /// Create color, modifying the color's alpha
    ///
    /// # Examples
    ///
    /// ```
    /// # use duku::Rgb;
    /// let color = Rgb::blue(128).alpha(50);
    /// # assert_eq!(color.r, 0);
    /// # assert_eq!(color.g, 0);
    /// # assert_eq!(color.b, 128);
    /// # assert_eq!(color.a, 50);
    /// ```
    pub const fn alpha(mut self, a: u8) -> Self {
        self.a = a;
        self
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Self::gray(0)
    }
}

impl Mix for Rgb {
    fn mix(from: Self, to: Self, p: f32) -> Self {
        Self {
            r: mixf(f32::from(from.r), f32::from(to.r), p) as u8,
            g: mixf(f32::from(from.g), f32::from(to.g), p) as u8,
            b: mixf(f32::from(from.b), f32::from(to.b), p) as u8,
            a: mixf(f32::from(from.a), f32::from(to.a), p) as u8,
        }
    }
}

impl From<Rgbf> for Rgb {
    fn from(c: Rgbf) -> Self {
        Self {
            r: (c.r * 255.0).round() as u8,
            g: (c.g * 255.0).round() as u8,
            b: (c.b * 255.0).round() as u8,
            a: (c.a * 255.0).round() as u8,
        }
    }
}

impl From<Hsb> for Rgb {
    fn from(c: Hsb) -> Self {
        let rgbf = Rgbf::from(c);
        Self::from(rgbf)
    }
}

impl From<&str> for Rgb {
    fn from(s: &str) -> Self {
        let black = Rgb::gray(0);
        let trimmed = s.trim_start_matches('#');

        if trimmed.len() < 6 || trimmed.len() > 8 {
            return black;
        }

        let rb = trimmed.get(0..2).unwrap_or("00");
        let gb = trimmed.get(2..4).unwrap_or("00");
        let bb = trimmed.get(4..6).unwrap_or("00");
        let ab = trimmed.get(6..8).unwrap_or("FF");

        let ru = u8::from_str_radix(rb, 16).unwrap_or(0);
        let gu = u8::from_str_radix(gb, 16).unwrap_or(0);
        let bu = u8::from_str_radix(bb, 16).unwrap_or(0);
        let au = u8::from_str_radix(ab, 16).unwrap_or(255);

        Self {
            r: ru,
            g: gu,
            b: bu,
            a: au,
        }
    }
}

impl From<[u8; 3]> for Rgb {
    fn from(a: [u8; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl From<[u8; 4]> for Rgb {
    fn from(a: [u8; 4]) -> Self {
        Self::new(a[0], a[1], a[2]).alpha(a[3])
    }
}

#[cfg(test)]
mod tests {
    use super::Hsb;
    use super::Rgb;
    use super::Rgbf;

    #[test]
    fn from_rgbf() {
        let rgbf = Rgbf::new(1.0, 0.5, 0.0);
        let rgb = Rgb::from(rgbf);

        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 128);
        assert_eq!(rgb.b, 0);
        assert_eq!(rgb.a, 255);
    }

    #[test]
    fn from_hsb() {
        let hsb = Hsb::new(7, 89, 83);
        let rgb = Rgb::from(hsb);

        assert_eq!(rgb.r, 212);
        assert_eq!(rgb.g, 45);
        assert_eq!(rgb.b, 23);
        assert_eq!(rgb.a, 255);
    }

    #[test]
    fn from_hex() {
        let aa = Rgb::from("#de8c78");
        assert_eq!(aa.r, 222);
        assert_eq!(aa.g, 140);
        assert_eq!(aa.b, 120);
        assert_eq!(aa.a, 255);

        let bb = Rgb::from("#de8c78ff");
        assert_eq!(bb.r, 222);
        assert_eq!(bb.g, 140);
        assert_eq!(bb.b, 120);
        assert_eq!(bb.a, 255);

        let cc = Rgb::from("de8c78");
        assert_eq!(cc.r, 222);
        assert_eq!(cc.g, 140);
        assert_eq!(cc.b, 120);
        assert_eq!(cc.a, 255);

        let dd = Rgb::from("fake");
        assert_eq!(dd.r, 0);
        assert_eq!(dd.g, 0);
        assert_eq!(dd.b, 0);
        assert_eq!(dd.a, 255);

        let ee = Rgb::from("evenfaker");
        assert_eq!(ee.r, 0);
        assert_eq!(ee.g, 0);
        assert_eq!(ee.b, 0);
        assert_eq!(ee.a, 255);
    }
}
