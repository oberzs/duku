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

/// 2-component Vector.
///
/// Used for 2D sizing and positioning.
///
/// # Examples
///
/// ```ignore
/// let point_1 = Vector2::new(-10.0, -10.0);
/// let point_2 = Vector2::new(10.0, 10.0);
///
/// target.draw_lines(&[point_1, point_2], false);
/// ```
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    /// the X component
    pub x: f32,
    /// the Y component
    pub y: f32,
}

impl Vector2 {
    /// Create a new vector
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Calculate the dot-product of the vector
    pub fn dot(&self, other: impl Into<Self>) -> f32 {
        let o = other.into();
        self.x * o.x + self.y * o.y
    }

    /// Calculate the squared length of a vector
    ///
    /// Can sometimes be used instead of
    /// [length](crate::math::Vector2::length),
    /// because this is faster.
    pub fn sqr_length(&self) -> f32 {
        self.dot(*self)
    }

    /// Calculate the length of a vector
    pub fn length(&self) -> f32 {
        self.sqr_length().sqrt()
    }

    /// Calculate the unit vector
    ///
    /// The unit vector is of length 1 and can also be
    /// thought of as the direction of the vector.
    pub fn unit(&self) -> Self {
        let scale = 1.0 / self.length();
        *self * if scale.is_infinite() { 0.0 } else { scale }
    }

    /// Calculate the normal vector
    ///
    /// The normal vector is a vector perpendicular to
    /// the original.
    ///
    /// May not be unit length
    pub fn normal(&self) -> Self {
        Vector2::new(-self.y, self.x)
    }

    /// Calculate the angle between 2 vectors
    ///
    /// Resulting angle is in degrees
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let up = Vector2::UP;
    /// let right = Vector2::RIGHT;
    /// let angle = up.angle_between(right);
    /// // angle is ~90 degrees
    /// ```
    pub fn angle_between(&self, other: impl Into<Self>) -> f32 {
        let o = other.into();
        let cos = self.dot(o) / (self.length() * o.length());
        cos.acos().to_degrees()
    }

    /// Floor every component of the vector
    pub fn floor(&self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }

    /// Ceil every component of the vector
    pub fn ceil(&self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil())
    }

    /// Round every component of the vector
    pub fn round(&self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }

    /// Shorthand for `Vector2::new(0.0, 1.0)`
    pub const UP: Self = Self::new(0.0, 1.0);
    /// Shorthand for `Vector2::new(0.0, -1.0)`
    pub const DOWN: Self = Self::new(0.0, -1.0);
    /// Shorthand for `Vector2::new(-1.0, 0.0)`
    pub const LEFT: Self = Self::new(-1.0, 0.0);
    /// Shorthand for `Vector2::new(1.0, 0.0)`
    pub const RIGHT: Self = Self::new(1.0, 0.0);
}

impl<N: Into<f32> + Copy> From<[N; 2]> for Vector2 {
    fn from(a: [N; 2]) -> Self {
        Self::new(a[0].into(), a[1].into())
    }
}

impl Index<usize> for Vector2 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of range {}", index),
        }
    }
}

impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of range {}", index),
        }
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Add<Self> for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Self> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl AddAssign<Self> for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign<Self> for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod test {
    use super::Vector2;

    #[test]
    fn default() {
        let v = Vector2::default();
        assert_eq_delta!(v.x, 0.0);
        assert_eq_delta!(v.y, 0.0);
    }

    #[test]
    fn new() {
        let v = Vector2::new(1.0, 2.0);
        assert_eq_delta!(v.x, 1.0);
        assert_eq_delta!(v.y, 2.0);
    }

    #[test]
    fn dot() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(5.0, 6.0);
        assert_eq_delta!(a.dot(b), 17.0);
    }

    #[test]
    fn sqr_length() {
        let v = Vector2::new(3.0, 4.0);
        assert_eq_delta!(v.sqr_length(), 25.0);
    }

    #[test]
    fn length() {
        let v = Vector2::new(3.0, 4.0);
        assert_eq_delta!(v.length(), 5.0);
    }

    #[test]
    fn unit() {
        let v = Vector2::new(3.0, 4.0);
        let u = v.unit();
        assert_eq_delta!(u.x, 0.6);
        assert_eq_delta!(u.y, 0.8);
    }

    #[test]
    fn normal() {
        let v = Vector2::new(6.0, 7.0);
        let n = v.normal();
        assert_eq_delta!(n.x, -7.0);
        assert_eq_delta!(n.y, 6.0);
    }

    #[test]
    fn angle_between() {
        let v1 = Vector2::new(1.0, 3.0);
        let v2 = Vector2::new(-3.0, 1.0);
        assert_eq_delta!(v1.angle_between(v2), 90.0);
    }

    #[test]
    fn operators() {
        let v1 = Vector2::new(2.0, 3.0);
        let v2 = Vector2::new(2.0, 8.0);
        assert_eq!(-v1, Vector2::new(-2.0, -3.0));
        assert_eq!(v1 + v2, Vector2::new(4.0, 11.0));
        assert_eq!(v1 - v2, Vector2::new(0.0, -5.0));
        assert_eq!(v1 * 4.0, Vector2::new(8.0, 12.0));
        assert_eq!(v2 / 2.0, Vector2::new(1.0, 4.0));
    }
}
