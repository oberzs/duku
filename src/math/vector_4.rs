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

use super::Vector2;
use super::Vector3;
use crate::renderer::Color;

/// 4-component Vector.
///
/// Used mostly as columns of [Matrix4](crate::math::Matrix4)
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector4 {
    /// the X component
    pub x: f32,
    /// the Y component
    pub y: f32,
    /// the Z component
    pub z: f32,
    /// the W component
    pub w: f32,
}

impl Vector4 {
    /// Create a new vector
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Calculate the dot-product of the vector
    pub fn dot(self, other: impl Into<Self>) -> f32 {
        let o = other.into();
        self.x * o.x + self.y * o.y + self.z * o.z + self.w * o.w
    }

    /// Get the [Vector2](crate::math::Vector2)
    /// made from this vectors x and y
    pub const fn xy(self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }

    /// Get the [Vector3](crate::math::Vector3)
    /// made from this vectors x, y and z
    pub const fn xyz(self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }

    /// Floor every component of the vector
    pub fn floor(self) -> Vector4 {
        Vector4::new(
            self.x.floor(),
            self.y.floor(),
            self.z.floor(),
            self.w.floor(),
        )
    }

    /// Ceil every component of the vector
    pub fn ceil(self) -> Vector4 {
        Vector4::new(self.x.ceil(), self.y.ceil(), self.z.ceil(), self.w.ceil())
    }

    /// Round every component of the vector
    pub fn round(self) -> Vector4 {
        Vector4::new(
            self.x.round(),
            self.y.round(),
            self.z.round(),
            self.w.round(),
        )
    }
}

impl From<[f32; 4]> for Vector4 {
    fn from(a: [f32; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }
}

impl From<Color> for Vector4 {
    fn from(c: Color) -> Self {
        Self::from(c.to_rgba_norm())
    }
}

impl From<(Vector3, f32)> for Vector4 {
    fn from(v: (Vector3, f32)) -> Self {
        Self::new(v.0.x, v.0.y, v.0.z, v.1)
    }
}

impl Index<usize> for Vector4 {
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

impl IndexMut<usize> for Vector4 {
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

impl Neg for Vector4 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Add<Self> for Vector4 {
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

impl Sub<Self> for Vector4 {
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

impl Mul<f32> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div<f32> for Vector4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl AddAssign<Self> for Vector4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign<Self> for Vector4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl DivAssign<f32> for Vector4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod test {
    use super::Vector4;

    #[test]
    fn default() {
        let v = Vector4::default();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
        assert_eq!(v.w, 0.0);
    }

    #[test]
    fn new() {
        let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.w, 4.0);
    }

    #[test]
    fn dot() {
        let a = Vector4::new(1.0, 2.0, 3.0, 1.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 1.0);
        assert_eq!(a.dot(b), 39.0);
    }

    #[test]
    fn operator() {
        let v1 = Vector4::new(2.0, 3.0, 4.0, 1.0);
        let v2 = Vector4::new(2.0, 8.0, 4.0, 1.0);
        assert_eq!(-v1, Vector4::new(-2.0, -3.0, -4.0, -1.0));
        assert_eq!(v1 + v2, Vector4::new(4.0, 11.0, 8.0, 2.0));
        assert_eq!(v1 - v2, Vector4::new(0.0, -5.0, 0.0, 0.0));
        assert_eq!(v1 * 4.0, Vector4::new(8.0, 12.0, 16.0, 4.0));
        assert_eq!(v2 / 2.0, Vector4::new(1.0, 4.0, 2.0, 0.5));
    }

    #[test]
    fn operators_assign() {
        let v = Vector4::new(2.0, 2.0, 2.0, 1.0);
        let mut add = Vector4::new(1.0, 3.0, 2.0, 1.0);
        let mut sub = Vector4::new(3.0, 5.0, 2.0, 1.0);
        let mut mul = Vector4::new(1.0, 3.0, 2.0, 1.0);
        let mut div = Vector4::new(4.0, 6.0, 2.0, 1.0);
        add += v;
        sub -= v;
        mul *= 2.0;
        div /= 2.0;
        assert_eq!(add, Vector4::new(3.0, 5.0, 4.0, 2.0));
        assert_eq!(sub, Vector4::new(1.0, 3.0, 0.0, 0.0));
        assert_eq!(mul, Vector4::new(2.0, 6.0, 4.0, 2.0));
        assert_eq!(div, Vector4::new(2.0, 3.0, 1.0, 0.5));
    }
}
