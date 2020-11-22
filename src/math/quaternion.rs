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
        let mut q = Self::default();
        let trace = m.x.x + m.y.y + m.z.z;
        if trace > 0.0 {
            let s = 0.5 / (trace + 1.0).sqrt();
            q.w = 0.25 / s;
            q.x = (m.y.z - m.z.y) * s;
            q.y = (m.z.x - m.x.z) * s;
            q.z = (m.x.y - m.y.x) * s;
        } else if m.x.x > m.y.y && m.x.x > m.z.z {
            let s = 2.0 * (1.0 + m.x.x - m.y.y - m.z.z).sqrt();
            q.w = (m.y.z - m.z.y) / s;
            q.x = 0.25 * s;
            q.y = (m.y.x + m.x.y) / s;
            q.z = (m.z.x + m.x.z) / s;
        } else if m.y.y > m.z.z {
            let s = 2.0 * (1.0 + m.y.y - m.x.x - m.z.z).sqrt();
            q.w = (m.z.x - m.x.z) / s;
            q.x = (m.y.x + m.x.y) / s;
            q.y = 0.25 * s;
            q.z = (m.z.y + m.y.z) / s;
        } else {
            let s = 2.0 * (1.0 + m.z.z - m.x.x - m.y.y).sqrt();
            q.w = (m.x.y - m.y.x) / s;
            q.x = (m.z.x + m.x.z) / s;
            q.y = (m.z.y + m.y.z) / s;
            q.z = 0.25 * s;
        }
        q
    }
}

#[cfg(test)]
mod test {
    use super::Matrix4;
    use super::Quaternion;
    use super::Vector3;

    #[test]
    fn default() {
        assert_eq!(Quaternion::default(), Quaternion::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn axis_rotation() {
        let q = Quaternion::axis_rotation([1.0, 0.0, 0.0], 180.0);
        let v = Vector3::new(1.0, 1.0, 1.0);
        let r = q * v;
        assert_eq_delta!(r.x, 1.0);
        assert_eq_delta!(r.y, -1.0);
        assert_eq_delta!(r.z, -1.0);
    }

    #[test]
    fn look_rotation_x() {
        let q = Quaternion::look_rotation([1.0, 0.0, 0.0], Vector3::UP);
        let r = q * Vector3::FORWARD;
        assert_eq_delta!(r.x, -1.0);
        assert_eq_delta!(r.y, 0.0);
        assert_eq_delta!(r.z, 0.0);
    }

    #[test]
    fn look_rotation_y() {
        let q = Quaternion::look_rotation([0.0, 1.0, 0.0], Vector3::FORWARD);
        let r = q * Vector3::FORWARD;
        assert_eq_delta!(r.x, 0.0);
        assert_eq_delta!(r.y, 1.0);
        assert_eq_delta!(r.z, 0.0);
    }

    #[test]
    fn look_rotation_z() {
        let q = Quaternion::look_rotation([0.0, 0.0, -1.0], Vector3::UP);
        let r = q * Vector3::FORWARD;
        assert_eq_delta!(r.x, 0.0);
        assert_eq_delta!(r.y, 0.0);
        assert_eq_delta!(r.z, -1.0);
    }

    #[test]
    fn euler_rotation_x() {
        let q = Quaternion::euler_rotation(90.0, 0.0, 0.0);
        let v = Vector3::new(0.0, 0.0, 1.0);
        let r = q * v;
        assert_eq_delta!(r.x, 0.0);
        assert_eq_delta!(r.y, -1.0);
        assert_eq_delta!(r.z, 0.0);
    }

    #[test]
    fn euler_rotation_y() {
        let q = Quaternion::euler_rotation(0.0, 90.0, 0.0);
        let v = Vector3::new(0.0, 0.0, 1.0);
        let r = q * v;
        assert_eq_delta!(r.x, 1.0);
        assert_eq_delta!(r.y, 0.0);
        assert_eq_delta!(r.z, 0.0);
    }

    #[test]
    fn euler_rotation_z() {
        let q = Quaternion::euler_rotation(0.0, 0.0, 90.0);
        let v = Vector3::new(1.0, 0.0, 0.0);
        let r = q * v;
        assert_eq_delta!(r.x, 0.0);
        assert_eq_delta!(r.y, 1.0);
        assert_eq_delta!(r.z, 0.0);
    }

    #[test]
    fn quat_to_mat_to_quat() {
        let q1 = Quaternion::euler_rotation(0.0, 45.0, 45.0);
        let q2 = Quaternion::from(Matrix4::from(q1));

        assert_eq_delta!(q1.x, q2.x);
        assert_eq_delta!(q1.y, q2.y);
        assert_eq_delta!(q1.z, q2.z);
        assert_eq_delta!(q1.w, q2.w);
    }

    #[test]
    fn mul_vector() {
        let q = Quaternion::euler_rotation(0.0, 0.0, 90.0);
        let v = Vector3::RIGHT;
        let r = q * v;

        assert_eq_delta!(r.x, 0.0);
        assert_eq_delta!(r.y, 1.0);
        assert_eq_delta!(r.z, 0.0);
    }
    #[test]
    fn mul_self() {
        let q1 = Quaternion::euler_rotation(0.0, 0.0, 180.0);
        let q2 =
            Quaternion::euler_rotation(0.0, 0.0, 90.0) * Quaternion::euler_rotation(0.0, 0.0, 90.0);

        assert_eq_delta!(q1.x, q2.x);
        assert_eq_delta!(q1.y, q2.y);
        assert_eq_delta!(q1.z, q2.z);
        assert_eq_delta!(q1.w, q2.w);
    }
}
