use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[repr(C, align(8))]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(self, other: impl Into<Self>) -> f32 {
        let o = other.into();
        self.x * o.x + self.y * o.y
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
}

impl From<[f32; 2]> for Vector2 {
    fn from(array: [f32; 2]) -> Self {
        Self::new(array[0], array[1])
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

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod test {
    use super::Vector2;

    #[test]
    fn default() {
        let v = Vector2::default();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
    }

    #[test]
    fn new() {
        let v = Vector2::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn from_array() {
        let v: Vector2 = [5.0, 6.0].into();
        assert_eq!(v.x, 5.0);
        assert_eq!(v.y, 6.0);
    }

    #[test]
    fn dot() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(5.0, 6.0);
        assert_eq!(a.dot(b), 17.0);
    }

    #[test]
    fn length() {
        let v = Vector2::new(2.0, 4.0);
        assert_eq!(v.length(), 4.472_136);
    }

    #[test]
    fn unit() {
        let v = Vector2::new(3.0, 4.0);
        assert_eq!(v.unit(), Vector2::new(0.6, 0.8));
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
