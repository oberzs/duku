// Oliver Berzs
// https://github.com/oberzs/duku

use super::Matrix4;
use super::Vector3;

use std::ops::Mul;
use std::ops::MulAssign;

/// Compact 3D rotation representation.
///
/// Used for rotating vectors
///
/// # Example
///
/// ```ignore
/// let vector = Vector3::UP;
/// let quat = Quaternion::euler_rotation(0.0, 0.0, 90.0);
/// assert_eq!(quat * vector, Vector3::RIGHT);
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion {
    /// the X component
    pub x: f32,
    /// the Y component
    pub y: f32,
    /// the Z component
    pub z: f32,
    /// the W component
    pub w: f32,
}

impl Quaternion {
    /// Create quaternion
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Create quaternion with euler angles
    ///
    /// This rotation's yaw, pitch and roll are z, y and x
    pub fn euler_rotation(x: f32, y: f32, z: f32) -> Self {
        let m = Matrix4::euler_rotation(x, y, z);
        Self::from(m)
    }

    /// Create quaternion around axis
    ///
    /// This rotates vectors around axis by the angle
    pub fn axis_rotation(axis: impl Into<Vector3>, angle: f32) -> Self {
        let m = Matrix4::axis_rotation(axis, angle);
        Self::from(m)
    }

    /// Create quaternion to rotate towards direction
    ///
    /// Note: `global_up` is used as a guide to try aligning to
    pub fn look_rotation(dir: impl Into<Vector3>, global_up: impl Into<Vector3>) -> Self {
        let m = Matrix4::look_rotation(dir, global_up);
        Self::from(m)
    }

    /// Calculate the inverse rotation
    pub fn inverse(self) -> Quaternion {
        let mut result = self;
        result.w = -result.w;
        result
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

impl Mul<Vector3> for Quaternion {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        let u = Vector3::new(self.x, self.y, self.z);
        let s = self.w;

        u * 2.0 * u.dot(rhs) + rhs * (s * s - u.dot(u)) + u.cross(rhs) * 2.0 * s
    }
}

impl Mul<Self> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;
        let x = self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y;
        let y = self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x;
        let z = self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w;

        Self::new(x, y, z, w)
    }
}

impl MulAssign<Self> for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl From<Matrix4> for Quaternion {
    fn from(m: Matrix4) -> Self {
        fn pos(value: f32) -> f32 {
            if value < 0.0 {
                0.0
            } else {
                value
            }
        }

        let qw = pos(1.0 + m.x.x + m.y.y + m.z.z).sqrt() / 2.0;
        let mut qx = pos(1.0 + m.x.x - m.y.y - m.z.z).sqrt() / 2.0;
        let mut qy = pos(1.0 - m.x.x + m.y.y - m.z.z).sqrt() / 2.0;
        let mut qz = pos(1.0 - m.x.x - m.y.y + m.z.z).sqrt() / 2.0;
        qx = qx.copysign(m.z.y - m.y.z);
        qy = -qy.copysign(m.x.z - m.z.x);
        qz = qz.copysign(m.y.x - m.x.y);

        Self::new(qx, qy, qz, qw)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod test {
    use super::Quaternion;
    use super::Vector3;

    #[test]
    fn default() {
        assert_eq!(Quaternion::default(), Quaternion::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn axis_rotation() {
        let q = Quaternion::axis_rotation([1.0, 0.0, 0.0], 90.0);
        assert_eq!(q, Quaternion::new(0.707_106_77, 0.0, 0.0, 0.707_106_77));
    }

    #[test]
    fn euler_rotation() {
        let q = Quaternion::euler_rotation(90.0, 0.0, 0.0);
        assert_eq!(q, Quaternion::new(0.707_106_77, 0.0, 0.0, 0.707_106_77));
    }

    #[test]
    fn look_rotation() {
        let q = Quaternion::look_rotation([1.0, 0.0, 0.0], Vector3::UP);
        assert_eq!(q, Quaternion::new(0.0, 0.707_106_77, 0.0, 0.707_106_77));
    }

    #[test]
    fn mul_vector() {
        let q = Quaternion::axis_rotation([0.0, 1.0, 0.0], 90.0);
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(q * v, Vector3::new(2.999_999_8, 1.999_999_9, -0.999_999_94));

        let q1 = Quaternion::look_rotation([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert_eq!(
            q1 * v,
            Vector3::new(2.999_999_8, 1.999_999_9, -0.999_999_94)
        );
    }

    #[test]
    fn mul_self() {
        let mut q1 = Quaternion::axis_rotation([1.0, 0.0, 0.0], 90.0);
        let r = q1 * q1;
        q1 *= q1;
        assert_eq!(r, Quaternion::new(0.999_999_94, 0.0, 0.0, 0.0));
        assert_eq!(q1, Quaternion::new(0.999_999_94, 0.0, 0.0, 0.0));
    }
}
