// Oliver Berzs
// https://github.com/oberzs/duku

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

use super::Vec2;
use super::Vec3;
use crate::color::Rgbf;

/// 4-component Vector.
///
/// Used mostly as columns of [Mat4](crate::math::Mat4)
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vec4 {
    /// the X component
    pub x: f32,
    /// the Y component
    pub y: f32,
    /// the Z component
    pub z: f32,
    /// the W component
    pub w: f32,
}

impl Vec4 {
    /// Create a new vector
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Calculate the dot-product of the vector
    pub fn dot(self, other: impl Into<Self>) -> f32 {
        let o = other.into();
        self.x * o.x + self.y * o.y + self.z * o.z + self.w * o.w
    }

    /// Get the [Vec2](crate::math::Vec2)
    /// made from this vectors x and y
    pub const fn xy(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    /// Get the [Vec3](crate::math::Vec3)
    /// made from this vectors x, y and z
    pub const fn xyz(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    /// Floor every component of the vector
    pub fn floor(self) -> Vec4 {
        Vec4::new(
            self.x.floor(),
            self.y.floor(),
            self.z.floor(),
            self.w.floor(),
        )
    }

    /// Ceil every component of the vector
    pub fn ceil(self) -> Vec4 {
        Vec4::new(self.x.ceil(), self.y.ceil(), self.z.ceil(), self.w.ceil())
    }

    /// Round every component of the vector
    pub fn round(self) -> Vec4 {
        Vec4::new(
            self.x.round(),
            self.y.round(),
            self.z.round(),
            self.w.round(),
        )
    }
}

impl From<[f32; 4]> for Vec4 {
    fn from(a: [f32; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }
}

impl From<(Vec3, f32)> for Vec4 {
    fn from(v: (Vec3, f32)) -> Self {
        Self::new(v.0.x, v.0.y, v.0.z, v.1)
    }
}

impl From<Rgbf> for Vec4 {
    fn from(c: Rgbf) -> Self {
        Self::new(c.r, c.g, c.b, c.a)
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index out of range {}", index),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("index out of range {}", index),
        }
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Add<Self> for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub<Self> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl AddAssign<Self> for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign<Self> for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod test {
    use super::Vec4;

    #[test]
    fn default() {
        let v = Vec4::default();
        assert_eq_delta!(v.x, 0.0);
        assert_eq_delta!(v.y, 0.0);
        assert_eq_delta!(v.z, 0.0);
        assert_eq_delta!(v.w, 0.0);
    }

    #[test]
    fn new() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq_delta!(v.x, 1.0);
        assert_eq_delta!(v.y, 2.0);
        assert_eq_delta!(v.z, 3.0);
        assert_eq_delta!(v.w, 4.0);
    }

    #[test]
    fn dot() {
        let a = Vec4::new(1.0, 2.0, 3.0, 1.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 1.0);
        assert_eq_delta!(a.dot(b), 39.0);
    }

    #[test]
    fn operator() {
        let v1 = Vec4::new(2.0, 3.0, 4.0, 1.0);
        let v2 = Vec4::new(2.0, 8.0, 4.0, 1.0);
        assert_eq!(-v1, Vec4::new(-2.0, -3.0, -4.0, -1.0));
        assert_eq!(v1 + v2, Vec4::new(4.0, 11.0, 8.0, 2.0));
        assert_eq!(v1 - v2, Vec4::new(0.0, -5.0, 0.0, 0.0));
        assert_eq!(v1 * 4.0, Vec4::new(8.0, 12.0, 16.0, 4.0));
        assert_eq!(v2 / 2.0, Vec4::new(1.0, 4.0, 2.0, 0.5));
    }
}
