use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

use super::Vector2;

#[repr(C, align(16))]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn backward() -> Self {
        Self::new(0.0, 0.0, -1.0)
    }

    pub fn forward() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn up() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn down() -> Self {
        Self::new(0.0, -1.0, 0.0)
    }

    pub fn left() -> Self {
        Self::new(-1.0, 0.0, 0.0)
    }

    pub fn right() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn dot(self, other: impl Into<Self>) -> f32 {
        let o = other.into();
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn cross(self, other: impl Into<Self>) -> Self {
        let o = other.into();
        let x = self.y * o.z - self.z * o.y;
        let y = self.z * o.x - self.x * o.z;
        let z = self.x * o.y - self.y * o.x;
        Self::new(x, y, z)
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn unit(self) -> Self {
        let scale = 1.0 / self.length();
        self * if scale.is_infinite() { 0.0 } else { scale }
    }

    pub fn angle_between(self, other: impl Into<Self>) -> f32 {
        let o = other.into();
        let cos = self.dot(o) / (self.length() * o.length());
        cos.acos().to_degrees()
    }

    pub fn shrink(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

impl From<[f32; 3]> for Vector3 {
    fn from(array: [f32; 3]) -> Self {
        Self::new(array[0], array[1], array[2])
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Add<Self> for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Self> for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl AddAssign<Self> for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign<Self> for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod test {
    use super::Vector2;
    use super::Vector3;

    #[test]
    fn default() {
        let v = Vector3::default();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn new() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn from_array() {
        let v: Vector3 = [5.0, 6.0, 7.0].into();
        assert_eq!(v.x, 5.0);
        assert_eq!(v.y, 6.0);
        assert_eq!(v.z, 7.0);
    }

    #[test]
    fn dot() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(5.0, 6.0, 7.0);
        assert_eq!(a.dot(b), 38.0);
    }

    #[test]
    fn cross() {
        let a = Vector3::new(2.0, 3.0, 4.0);
        let b = Vector3::new(5.0, 6.0, 7.0);
        assert_eq!(a.cross(b), Vector3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn length() {
        let v = Vector3::new(2.0, 4.0, -2.0);
        assert_eq!(v.length(), 4.898_979_7);
    }

    #[test]
    fn unit() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        assert_eq!(v.unit(), Vector3::new(0.6, 0.8, 0.0));
    }

    #[test]
    fn angle_between() {
        let a = Vector3::new(4.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 13.0, 0.0);
        assert_eq!(a.angle_between(b), 90.0);
    }

    #[test]
    fn shrink() {
        let v = Vector3::new(1.0, 3.0, 2.0);
        assert_eq!(v.shrink(), Vector2::new(1.0, 3.0));
    }

    #[test]
    fn direction() {
        assert_eq!(Vector3::forward(), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(Vector3::backward(), Vector3::new(0.0, 0.0, -1.0));
        assert_eq!(Vector3::down(), Vector3::new(0.0, -1.0, 0.0));
        assert_eq!(Vector3::up(), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(Vector3::right(), Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(Vector3::left(), Vector3::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn operator() {
        let v1 = Vector3::new(2.0, 3.0, 4.0);
        let v2 = Vector3::new(2.0, 8.0, 4.0);
        assert_eq!(-v1, Vector3::new(-2.0, -3.0, -4.0));
        assert_eq!(v1 + v2, Vector3::new(4.0, 11.0, 8.0));
        assert_eq!(v1 - v2, Vector3::new(0.0, -5.0, 0.0));
        assert_eq!(v1 * 4.0, Vector3::new(8.0, 12.0, 16.0));
        assert_eq!(v2 / 2.0, Vector3::new(1.0, 4.0, 2.0));
    }

    #[test]
    fn operators_assign() {
        let v = Vector3::new(2.0, 2.0, 2.0);
        let mut add = Vector3::new(1.0, 3.0, 2.0);
        let mut sub = Vector3::new(3.0, 5.0, 2.0);
        let mut mul = Vector3::new(1.0, 3.0, 2.0);
        let mut div = Vector3::new(4.0, 6.0, 2.0);
        add += v;
        sub -= v;
        mul *= 2.0;
        div /= 2.0;
        assert_eq!(add, Vector3::new(3.0, 5.0, 4.0));
        assert_eq!(sub, Vector3::new(1.0, 3.0, 0.0));
        assert_eq!(mul, Vector3::new(2.0, 6.0, 4.0));
        assert_eq!(div, Vector3::new(2.0, 3.0, 1.0));
    }
}
